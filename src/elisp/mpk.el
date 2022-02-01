;;; mpk-mode.el --- media production kit -*- lexical-binding: t; -*-

;; Copyright (C) 2021  ellis

;; Author: ellis <ellis@rwest.io>
;; Keywords: multimedia, hardware, tools

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
;; 

;;; Code:

(defvar mpk-version "0.1.0"
  "Mpk version string."
  :group 'multimedia)

;;; Customization
(defgroup mpk nil
  "Mpk Emacs extensions.")

(defcustom mpk-lib-dir (expand-file-name "~/mpk/lib")
  "Mpk library directory."
  :group 'mpk)

(defcustom mpk-save-session-hook nil
  "Hook run after saving mpk session buffer."
  :type 'hook
  :group 'mpk)

(defcustom mpk-open-session-hook nil
  "Hook run after opening mpk session buffer."
  :type 'hook
  :group 'mpk)

(defcustom mpk-db-type 'sqlite
  "Storage backend for mpk resources."
  :type 'symbol
  :group 'mpk)

(defvar mpk-mode-buffer-name "*mpk*"
  "Mpk buffer name.")

(defun mpk-mode-buffer ()
  (or (get-buffer mpk-mode-buffer-name)
      (with-current-buffer (get-buffer-create mpk-mode-buffer-name)
	(setq major-mode 'mpk-mode
	      mode-name "MPK"
	      mode-line-format (copy-tree mode-line-format))
	(current-buffer))))

(provide 'mpk-mode)
;;; mpk-mode.el ends here
