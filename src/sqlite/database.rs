use super::SQLiteError;
use std::ffi::{c_char, c_int, c_void, CStr, CString};

pub mod recover;

pub struct Database {
  sqlite_db: *mut c_void,
}

impl Database {
  pub fn open(path: &str) -> Result<Self, SQLiteError> {
    extern "C" {
      fn sqlite3_open(path: *const c_char, db: *mut *mut c_void) -> c_int;
      fn sqlite3_errmsg(db: *mut c_void) -> *const c_char;
    }

    let mut db = std::ptr::null_mut();

    let err_code =
      unsafe { sqlite3_open(CString::new(path).unwrap().as_c_str().as_ptr(), &mut db) };

    if err_code != 0 {
      return Err(SQLiteError {
        code: err_code,
        message: unsafe {
          CStr::from_ptr(sqlite3_errmsg(db))
            .to_str()
            .unwrap()
            .to_owned()
        },
      });
    }

    Ok(Database { sqlite_db: db })
  }

  // pub fn recover(self, recovered_path: &str, cb: extern "C" fn() -> c_int) {}
}

impl Drop for Database {
  fn drop(&mut self) {
    extern "C" {
      fn sqlite3_close(db: *mut c_void) -> c_int;
    }
    unsafe { sqlite3_close(self.sqlite_db) };
  }
}
