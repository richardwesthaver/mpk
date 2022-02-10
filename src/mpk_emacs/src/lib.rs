use emacs::{defun, Env, Result, Value};

emacs::plugin_is_GPL_compatible!();

#[emacs::module(name="mpk")]
fn init(_: &Env) -> Result<()> {Ok(())}
