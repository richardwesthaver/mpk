use std::ffi::{CStr, OsStr, CString};
use std::os::unix::ffi::OsStrExt;
use std::os::raw::{c_int, c_char};
use std::path::Path;
use mpk_db::Mdb;
use mpk_config::{Config, DbConfig, FsConfig, JackConfig};

#[no_mangle] pub extern fn mpk_config_new(fs: *const FsConfig, db: *const DbConfig, jack: *const JackConfig) -> *mut Config {
  if !fs.is_null() | !db.is_null() | !jack.is_null() {
    unsafe {
      let fs = &*fs;
      let db = &*db;
      let jack = &*jack;
      Box::into_raw(Box::new(Config::new(fs.to_owned(), db.to_owned(), jack.to_owned()).unwrap()))
    }
  } else {
      Box::into_raw(Box::new(Config::default()))
    }
}

#[no_mangle] pub extern fn mpk_config_load(path: *const c_char) -> *mut Config {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

  Box::into_raw(Box::new(Config::load(p).unwrap()))
}

#[no_mangle] pub extern fn mpk_config_write(cfg: *const Config, path: *const c_char) {
  let cstr = unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  };
  let p: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));
  let cfg = unsafe {
    cfg.as_ref().unwrap()
  };
  cfg.write(p).unwrap()
}

#[no_mangle] pub unsafe extern fn mpk_config_build(cfg: *const Config) {
  cfg.as_ref().unwrap().build().unwrap()
}

#[no_mangle]
pub extern fn mpk_fs_config_new(root: *const c_char) -> *mut FsConfig {
  if !root.is_null() {
    let cstr = unsafe {
      CStr::from_ptr(root)
    };
    let r: &Path = Path::new(OsStr::from_bytes(cstr.to_bytes()));

    Box::into_raw(Box::new(FsConfig::new(r).unwrap()))
  } else {
    Box::into_raw(Box::new(FsConfig::default()))
  }
}

#[no_mangle]
pub extern fn mpk_fs_config_get_path(cfg: *const FsConfig, path: *const c_char) -> *const c_char {
  let p = &unsafe {
    assert!(!path.is_null());
    CStr::from_ptr(path)
  }.to_str().unwrap();

  let cfg = unsafe {cfg.as_ref().unwrap()};
  let res = cfg.get_path(p).unwrap();
  CString::new(res.as_os_str().as_bytes()).unwrap().into_raw()
}

#[no_mangle]
pub extern fn mpk_db_config_new() -> *mut DbConfig {
  Box::into_raw(Box::new(DbConfig::default()))
}

#[no_mangle]
pub extern fn mpk_db_config_flags(cfg: *const DbConfig) -> *const c_int {
  let cfg = unsafe {cfg.as_ref().unwrap()};
  &cfg.flags().unwrap()
}

#[no_mangle]
pub extern fn mpk_jack_config_new() -> *mut JackConfig {
  Box::into_raw(Box::new(JackConfig::new().unwrap()))
}

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
  mdb.insert_track(&str).unwrap()
}

#[no_mangle]
pub extern "C" fn mdb_insert_track_tags(db: *const Mdb, 
				   id: i64,
				   artist: *const c_char,
				   title: *const c_char,
				   album: *const c_char,
				   genre: *const c_char,
				   year: i16) {
  let artist = Some(unsafe {CStr::from_ptr(artist).to_str().unwrap().to_string()});
  let title = Some(unsafe {CStr::from_ptr(title).to_str().unwrap().to_string()});
  let album = Some(unsafe {CStr::from_ptr(album).to_str().unwrap().to_string()});
  let genre = Some(unsafe {CStr::from_ptr(genre).to_str().unwrap().to_string()});
  let year = Some(year);
  let mdb = unsafe {
    db.as_ref().unwrap()
  };
  mdb.insert_track_tags(id, artist, title, album, genre, year).unwrap();
}

#[no_mangle]
pub extern "C" fn mdb_exec_batch(db: *const Mdb, sql: *const c_char) {
  let sql = unsafe {
    CStr::from_ptr(sql).to_str().unwrap()
  };

  let mdb = unsafe {
    db.as_ref().unwrap()
  };

  mdb.exec_batch(sql).unwrap()
}
