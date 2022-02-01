use emacs::{defun, Env, Result, Value};
use rocksdb::{DB, Options};
emacs::plugin_is_GPL_compatible!();

#[emacs::module]
fn init(_: &Env) -> Result<()> {

  #[defun(user_ptr)]
  fn make() -> Result<DB> {
    Ok(DB::open_default("mdb_db").unwrap())
  }

  #[defun]
  fn get(db: &DB, key: String) -> Result<Option<String>> {
    let res = match db.get(&key).unwrap() {
      Some(val) => Some(String::from_utf8(val).unwrap()),
      None => None,
    };
    Ok(res)
  }

  #[defun]
  fn set(db: &DB, key: String, val: String) -> Result<()> {
    Ok(db.put(key,val).unwrap())
  }

  let _ = DB::destroy(&Options::default(), "mdb_db");
  Ok(())
}
