;;; mpk-mode.el --- media production kit -*- lexical-binding: t; -*-
;; 
;; Copyright (C) 2021  ellis
;; 
;; Author: ellis <ellis@rwest.io>
;; Keywords: multimedia, hardware, tools
;; 
;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.
;; 
;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.
;; 
;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.
;; 
;;; Commentary:
;; 
;; 
;; 
;;; Code:

(defgroup mpk nil
  "mpk Emacs extensions")

(defcustom mpk-lib-dir "~/mpk/lib"
  "mpk library directory"
  :group 'mpk)

(defcustom mpk-save-session-hook nil
  "Hook run after saving mpk session buffer"
  :type 'hook
  :group 'mpk)

(defcustom mpk-open-session-hook nil
  "Hook run after opening mpk session buffer"
  :type 'hook
  :group 'mpk)

(defvar mpk-db-type nil)

(provide 'mpk-mode)
;;; mpk-mode.el ends here
