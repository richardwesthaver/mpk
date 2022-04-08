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

(defun mpk-midi-init (device)
  (make-process :name "mpk-midi"
		:buffer "mpk-midi"
		:command '("mpk" "run" "monitor"))
  (process-send-string "mpk-midi" (format "%s\n" device)))

(defun mpk-midi-process-filter (process str)
  (let ((buffer (process-buffer process)))
    ))

(provide 'mpk-midi)
;;; mpk-midi.el ends here
