;;; mpk-net.el --- MPK Network Interface -*- lexical-binding: t; -*-
;; Copyright (C) 2021  ellis

;; Author: ellis
;; Keywords: local, vc, net, process

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

;; Commentary:

;; Network interface for interacting with the MPK daemon.

;;; Code:

;;;; Custom 
(defgroup mpk nil
  "mpk Emacs Modules")

(defcustom mpk-dir "~/mpk/" "mpk directory."
  :group 'mpk)

(defcustom server-after-make-frame-hook nil
  "Hook run when the mpk server creates a client frame.
The created frame is selected when the hook is called."
  :type 'hook
  :version "27.1"
  :group 'mpk)

(defcustom server-done-hook nil
  "Hook run when done editing a buffer for the mpk server."
  :type 'hook
  :group 'mpk)

(defvar mpk-data-dir (file-name-as-directory (expand-file-name "data" mpk-dir))
  "mpk data directory.")
(defvar mpk-src-dir (file-name-as-directory (expand-file-name "src" mpk-dir))
  "mpk src directory.")
(defvar mpk-stash-dir (file-name-as-directory (expand-file-name "stash" mpk-dir))
  "mpk stash directory.")
(defvar mpk-store-dir (file-name-as-directory (expand-file-name "store" mpk-dir))
  "mpk store directory.")
(defvar mpk-lab-dir (file-name-as-directory (expand-file-name "lab" mpk-dir))
  "mpk lab directory.")

(defvar mpk-server-process nil
  "The mpk-server process handle.")

(defvar mpk-server-clients nil
  "List of current server clients.
Each element is a process.")

(defvar mpk-cmd-server-port 62824
  "port of the mpk-status broadcaster")

(defvar mpk-cmd-server-clients '() 
  "alist where KEY is a client process and VALUE is the string")

;;;; Bindat
(setq mpk-header-bindat-spec
      '((dest-ip   ip)
        (dest-port u16)
        (src-ip    ip)
        (src-port  u16)))

(setq mpk-body-bindat-spec
      '((type      u8)
        (opcode    u8)
        (length    u16)  ; network byte order
        (id        strz 8) 		; strz?
        (data      vec (length))
        (align     4)))

(setq mpk-packet-bindat-spec
      '((header    struct header-spec)
        (counters  vec 2 u32r)   ; little endian order
        (items     u8)
        (fill      3)
        (item      repeat (items)
                   (struct data-spec))))

(defun mpk-insert-string (string)
  (insert string 0 (make-string (- 3 (% (length string) 4)) 0)))

(defun mpk-insert-int32 (value)
  (let (bytes)
    (dotimes (i 4)
      (push (% value 256) bytes)
      (setq value (/ value 256)))
    (dolist (byte bytes)
      (insert byte))))

(defun mpk-insert-float32 (value)
  (let (s (e 0) f)
    (cond
     ((string= (format "%f" value) (format "%f" -0.0))
      (setq s 1 f 0))
     ((string= (format "%f" value) (format "%f" 0.0))
      (setq s 0 f 0))
     ((= value 1.0e+INF)
      (setq s 0 e 255 f (1- (expt 2 23))))
     ((= value -1.0e+INF)
      (setq s 1 e 255 f (1- (expt 2 23))))
     ((string= (format "%f" value) (format "%f" 0.0e+NaN))
      (setq s 0 e 255 f 1))
     (t
      (setq s (if (>= value 0.0)
		  (progn (setq f value) 0)
		(setq f (* -1 value)) 1))
      (while (>= (* f (expt 2.0 e)) 2.0) (setq e (1- e)))
      (if (= e 0) (while (< (* f (expt 2.0 e)) 1.0) (setq e (1+ e))))
      (setq f (round (* (1- (* f (expt 2.0 e))) (expt 2 23)))
	    e (+ (* -1 e) 127))))
    (insert (+ (lsh s 7) (lsh (logand e #XFE) -1))
	    (+ (lsh (logand e #X01) 7) (lsh (logand f #X7F0000) -16))
	    (lsh (logand f #XFF00) -8)
	    (logand f #XFF))))

(defun mpk-read-string ()
  (let ((pos (point)) string)
    (while (not (= (following-char) 0)) (forward-char 1))
    (setq string (buffer-substring-no-properties pos (point)))
    (forward-char (- 4 (% (length string) 4)))
    string))

(defun mpk-read-int32 ()
  (let ((value 0))
    (dotimes (i 4)
      (setq value (logior (* value 256) (following-char)))
      (forward-char 1))
    value))

(defun mpk-read-float32 ()
  (let ((s (lsh (logand (following-char) #X80) -7))
	(e (+ (lsh (logand (following-char) #X7F) 1)
	      (lsh (logand (progn (forward-char) (following-char)) #X80) -7)))
	(f (+ (lsh (logand (following-char) #X7F) 16)
	      (lsh (progn (forward-char) (following-char)) 8)
	      (prog1 (progn (forward-char) (following-char)) (forward-char)))))
    (cond
     ((and (= e 0) (= f 0))
      (* 0.0 (expt -1 s))
      ((and (= e 255) (or (= f (1- (expt 2 23))) (= f 0)))
       (* 1.0e+INF (expt -1 s)))
      ((and (= e 255) (not (or (= f 0) (= f (1- (expt 2 23))))))
       0.0e+NaN)
      (t
       (* (expt -1 s)
	  (expt 2.0 (- e 127))
	  (1+ (/ f (expt 2.0 23)))))))))

;;;; Network
;;;###autoload
(defun net-check-opts ()
  ;; https://gnu.huihoo.org/emacs/24.4/emacs-lisp/Network-Options.html#Network-Options
  ;; non-blocking
  (featurep 'make-network-process '(:nowait t))
  ;; UNIX socket
					;(featurep 'make-network-process '(:family local))
  ;; UDP
  (featurep 'make-network-process '(:type datagram)))

;;;; Process
(defun mpk-make-client (host port)
  (make-network-process
   :name "mpk-cmd-client"
   :coding 'binary
   :host host
   :service port
   :type 'datagram
   :nowait t))

(defun mpk-cmd-server-sentinel (proc msg)
  (when (string= msg "connection broken by remote peer\n")
    (setq mpk-cmd-server-clients (assq-delete-all proc mpk-cmd-server-clients))
    (mpk-cmd-server-log (format "client %s has quit" proc))))

;;from server.el
;;;###autoload
(defun mpk-cmd-server-log (string &optional client)
  "If a *mpk-cmd-server* buffer exists, write STRING to it for logging purposes."
  (if (get-buffer "*mpk-cmd-server*")
      (with-current-buffer "*mpk-cmd-server*"
        (goto-char (point-max))
        (insert (if client (format "<%s>: " (format-network-address (process-datagram-address client))))
                string)
        (or (bolp) (newline)))))

(defun mpk-cmd-server-start nil
  "start a mpk-cmd-server over udp"
  (interactive)
  (unless (process-status "mpk-cmd-server")
    (make-network-process :name "mpk-cmd-server"
			  :buffer "*mpk-cmd-server*"
			  :family 'ipv4
			  :service mpk-cmd-server-port
			  :type 'datagram
			  :coding 'binary
			  :sentinel 'mpk-cmd-server-sentinel
			  :filter 'mpk-cmd-server-filter
			  :server t
			  :broadcast t) 
    (setq mpk-cmd-server-clients '())

    ;; setup additional filters
    ;; (add-function :after (process-filter (get-process "mpk-cmd-server")) #'mpk-babel-response-filter)
    )
  (message "mpk-cmd-server: ONLINE"))

(defun mpk-cmd-server-stop ()
  "stop a mpk-cmd-server"
  (interactive)
  (while  mpk-cmd-server-clients
    (delete-process (car (car mpk-cmd-server-clients)))
    (setq mpk-cmd-server-clients (cdr mpk-cmd-server-clients)))
  (with-current-buffer "*mpk-cmd-server*"
    (delete-process (get-buffer-process (current-buffer)))
    (set-buffer-modified-p nil)
    (kill-this-buffer)))

(defun mpk-cmd-server-filter (proc string)   
  (let ((pending (assoc proc mpk-cmd-server-clients))
        message
        index)
    ;;create entry if required
    (unless pending
      (setq mpk-cmd-server-clients (cons (cons proc "") mpk-cmd-server-clients))
      (setq pending  (assoc proc mpk-cmd-server-clients)))
    (setq message (concat (cdr pending) string))
    (while (setq index (string-match "\n" message))
      (setq index (1+ index))
;      (process-send-string proc (substring message 0 index))
      (mpk-cmd-server-log  (substring message 0 index) proc)
      (setq message (substring message index)))
    (setcdr pending message)))

(defun mpk-cmd-packet-filter (proc string)
  "process-filter for decoding 'mpk-packet-bindat-spec'"
  (bindat-unpack packet-spec string))

(defun ordinary-insertion-filter (proc string)
  (when (buffer-live-p (process-buffer proc))
    (with-current-buffer (process-buffer proc)
      (let ((moving (= (point) (process-mark proc))))

        (save-excursion
          ;; Insert the text, advancing the process marker.
          (goto-char (process-mark proc))
          (insert string)
          (set-marker (process-mark proc) (point)))
        (if moving (goto-char (process-mark proc)))))))

(defun mpk-babel-response-filter (proc string)
  "match STRING from PROC against 'org-babel-library-of-babel' functions."
  (let ((msg (car (read-from-string string)))
	(status))
    (if (assoc msg org-babel-library-of-babel)
	(progn 
	  (setq status "OK")
	  (mpk-cmd-server-log (format "BABEL_CMD:%s" status) proc)
	  (process-send-string proc (concat "\n" (eval (car (read-from-string
							     (format "(org-sbx %s)" string)))) "\n\n")))
      (progn
	(setq status "ERR")
	(mpk-cmd-server-log (format "BABEL_CMD:%s" status) proc)))))

;;;; Signals
(defun server-shutdown ()
  "Save buffers, Quit, and Shutdown (kill) server"
  (interactive)
  (save-some-buffers)
  (kill-emacs))

(defun signal-restart-server ()
  "Handler for SIGUSR1 signal, to (re)start an emacs server.

Can be tested from within emacs with:
  (signal-process (emacs-pid) 'sigusr1)

or from the command line with:
$ kill -USR1 <emacs-pid>
$ emacsclient -c
"
  (interactive)
  (server-force-delete)
  (server-start)
  )

(define-key special-event-map [sigusr1] 'signal-restart-server)
