;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((nil . ((eval . (progn
		   (defvar mpk-dev-mode-map (make-sparse-keymap)
		     "Keymap for MPK Development")
		   (define-minor-mode mpk-dev-mode
		     "Temporary minor-mode for MPK Development"
		     :lighter "MPK-DEV"
		     :keymap mpk-dev-mode-map)
		   (mpk-dev-mode 1)
		   (keymap-set mpk-dev-mode-map
			       "C-c c c" (lambda () (interactive) (eshell-command "nim build")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c v" (lambda () (interactive) (eshell-command "nim status")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c p" (lambda () (interactive) (eshell-command "nim ci")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c f" (lambda () (interactive) (eshell-command "nim pull")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c m" (lambda () (interactive) (eshell-command "nim mirror")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c o" (lambda () (interactive) (eshell-command "nim ox")))
		   (keymap-set mpk-dev-mode-map
			       "C-c c t" (lambda () (interactive) (eshell-command "nim test")))))))
 (org-mode . ((time-stamp-pattern . "4/#\\+DATE: %Y-%02m-%02d$")
	      (eval . (add-hook 'before-save-hook 'time-stamp nil t)))))
