use std::ffi::CStr;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use libc::c_char;
use std::path::Path;
use mpk_db::Mdb;

#[no_mangle]
pub extern fn mdb_new(path: *const c_char) -> *mut Mdb {
  let mdb: Mdb = if path.is_null() {
    Mdb::new(None).unwrap()
  } else {
    let cstr = unsafe {CStr::from_ptr(path)};
    let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
    Mdb::new(Some(p)).unwrap()
  };

  let mdb_box: Box<Mdb> = Box::new(mdb);

  Box::into_raw(mdb_box)
}

#[no_mangle]
pub unsafe extern "C" fn mdb_init(db: *const Mdb) {
  db.as_ref().unwrap().init().unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_insert_track(db: *const Mdb, path: *const c_char) -> i64 {
    let c_str = unsafe {
        assert!(!path.is_null());

        CStr::from_ptr(path)
    };
  let str = c_str.to_str().unwrap();
  let mdb = unsafe {
    db.as_ref().unwrap()
  };
  mdb.insert_track(&str).unwrap();
  mdb.last_insert_rowid()
}
