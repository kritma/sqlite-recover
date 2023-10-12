use std::ffi::{c_char, c_int, c_void, CStr, CString};

extern "C" {
  fn sqlite3_recover_init(db: *mut c_void, name: *const c_char, path: *const c_char)
    -> *mut c_void;
  fn sqlite3_recover_run(recover: *mut c_void) -> c_int;
  fn sqlite3_recover_finish(recover: *mut c_void) -> c_int;
  fn sqlite3_recover_errmsg(recover: *mut c_void) -> *const c_char;
  fn sqlite3_open(path: *const c_char, db: *mut *mut c_void) -> c_int;
  fn sqlite3_errmsg(db: *mut c_void) -> *const c_char;
}

pub struct SQLiteError {
  code: i32,
  message: String,
}

pub fn recover_db(db: *mut c_void, recovered_path: &str) -> Result<(), SQLiteError> {
  let recover = unsafe {
    sqlite3_recover_init(
      db,
      CString::new("main").unwrap().as_c_str().as_ptr(),
      CString::new(recovered_path).unwrap().as_c_str().as_ptr(),
    )
  };

  unsafe { sqlite3_recover_run(recover) };

  let err_code = unsafe { sqlite3_recover_finish(recover) };
  if err_code == 0 {
    return Ok(());
  }

  Err(SQLiteError {
    code: err_code,
    message: unsafe {
      CStr::from_ptr(sqlite3_recover_errmsg(recover))
        .to_str()
        .unwrap()
        .to_owned()
    },
  })
}

pub fn open_db(path: &str) -> Result<*mut c_void, SQLiteError> {
  let mut db = std::ptr::null_mut();
  let err_code = unsafe { sqlite3_open(CString::new(path).unwrap().as_c_str().as_ptr(), &mut db) };

  if err_code == 0 {
    return Ok(db);
  }

  Err(SQLiteError {
    code: err_code,
    message: unsafe {
      CStr::from_ptr(sqlite3_errmsg(db))
        .to_str()
        .unwrap()
        .to_owned()
    },
  })
}
