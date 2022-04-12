;;; mpk-midi.el --- MIDI interop utils -*- lexical-binding: t; -*-

;; Copyright (C) 2022  ellis

;; Author: ellis <ellis@rwest.io>
;; Keywords: convenience

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

;;; Commentary:

;; 

;;; Code:

(defcustom mpk-midi-map
  '((48 . find-file)
    (49 . keyboard-quit)
    (50 . org-agenda)
    (52 . jump-to-bookmark)
    (60 . emms-browser)
    (61 . emms-stream)
    (72 . ibuffer))
  "Mapping of MIDI notes to commands")

(defun mpk-midi-monitor (device)
  "Monitor incoming midi messages from input DEVICE given as an
unsigned integer index. Use `mpk info -m` to list all devices."
  (interactive "nmidi device: ")
  (let ((bufname "*MIDI I/O*")
	(procname "mpk-midi")
	(action (format "mpk run monitor %d" device)))
    (start-process-shell-command procname bufname action)
    (mpk-midi-prep-buffer bufname)))

(defun mpk-midi-prep-buffer (bufname)
  "Prepare the mpk-midi process buffer."
  (save-excursion
    (set-buffer bufname)
    (setq comint-move-point-for-output t)
    (set-process-filter
     (get-process (get-buffer-process bufname)) 'mpk-midi-process-filter)
    (make-local-variable 'mpk-midi-examination-marker)
    (setq mpk-midi-examination-marker (make-marker))
    (set-marker mpk-midi-examination-marker 0)))

(defun mpk-midi-process-key (chan note val)
  "Process a single MIDI message given 3 bytes: CHAN NOTE and VAL"
  (let ((match (alist-get note mpk-midi-map)))
    (if (and match (> val 0))
	(call-interactively match))))

(defun mpk-midi-scan ()
  "Parse lines from mpk-midi process buffer.
`TIME: [CHAN, NOTE, VAL]`"
  (goto-char (marker-position mpk-midi-examination-marker))
  (while (re-search-forward "\\([[:digit:]]*\\): \\[\\([[:digit:]]*\\), \\([[:digit:]]*\\), \\([[:digit:]]*\\)\\]$" nil t)
    (let* ((seq-start (match-beginning 0))
           (seq-end   (match-end 0))
           (sequence  (buffer-substring seq-start seq-end))
	   (time (match-string 1))
           (x (string-to-number (match-string 2)))
           (y (string-to-number (match-string 3)))
	   (z (string-to-number (match-string 4))))
      (set-marker mpk-midi-examination-marker seq-end)
      (mpk-midi-process-key x y z))))

(defun mpk-midi-process-filter (proc str)
  "Process filter for mpk-midi."
  (let ((buffer (process-buffer proc)))
    (with-current-buffer buffer
      (let ((moving (= (point) (marker-position (process-mark proc)))))
	(save-excursion
	  (goto-char (process-mark proc))
	  (insert str)
	  (set-marker (process-mark proc) (point))
	  ;; scan commands
	  (mpk-midi-scan)
	  )
	(comint-postoutput-scroll-to-bottom str)))))

(provide 'mpk-midi)
;;; mpk-midi.el ends here
