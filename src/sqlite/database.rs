use self::recover::{LostAndFoundOption, Recover, RecoverConfig, StepCallback};

use super::SQLiteError;
use std::ffi::{c_char, c_int, c_void, CStr, CString};

mod recover;

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

  pub fn recover_to(self, recovered_path: &str) -> Result<(), SQLiteError> {
    let mut recover = Recover::init(self, recovered_path);

    recover.configure(RecoverConfig {
      lost_and_found: Some(LostAndFoundOption {
        name: "lost_and_found".to_string(),
        recover_freelist: false,
      }),
      recover_rowids: true,
      slow_indexes: false,
      step_callback: None,
    });

    recover.run()?;
    Ok(())
  }

  pub fn recover_sql_to(
    self,
    recovered_path: &str,
    step_callback: StepCallback,
  ) -> Result<(), SQLiteError> {
    let mut recover = Recover::init_sql(self, recovered_path)?;

    recover.configure(RecoverConfig {
      lost_and_found: Some(LostAndFoundOption {
        name: "lost_and_found".to_string(),
        recover_freelist: false,
      }),
      recover_rowids: true,
      slow_indexes: false,
      step_callback: Some(step_callback),
    });

    recover.run()?;
    Ok(())
  }
}

impl Drop for Database {
  fn drop(&mut self) {
    extern "C" {
      fn sqlite3_close(db: *mut c_void) -> c_int;
    }
    unsafe { sqlite3_close(self.sqlite_db) };
  }
}
