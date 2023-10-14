use super::Database;
use crate::sqlite::{SQLiteError, SQLITE_OK};
use std::ffi::{c_char, c_int, c_void, CStr, CString};

const SQLITE_RECOVER_LOST_AND_FOUND: i32 = 1;
const SQLITE_RECOVER_FREELIST_CORRUPT: i32 = 2;
const SQLITE_RECOVER_ROWIDS: i32 = 3;
const SQLITE_RECOVER_SLOWINDEXES: i32 = 4;

pub struct LostAndFoundOption {
  pub name: String,
  pub recover_freelist: bool,
}

pub type StepCallback = fn(&str);

pub struct RecoverConfig {
  pub lost_and_found: Option<LostAndFoundOption>,
  pub recover_rowids: bool,
  pub slow_indexes: bool,
  pub step_callback: Option<StepCallback>,
}

pub struct Recover {
  db_to_recover: Database,
  recovered_db: Option<Database>,
  sqlite_recover: *mut c_void,
  step_callback: Option<StepCallback>,
}

impl Recover {
  pub fn init_sql(db_to_recover: Database, path_to_recovered: &str) -> Result<Self, SQLiteError> {
    extern "C" {
      fn sqlite3_recover_init_sql(
        db: *mut c_void,
        name: *const c_char,
        cb: extern "C" fn(ctx: *mut c_void, sql: *const c_char) -> c_int,
        ctx: *mut c_void,
      ) -> *mut c_void;
    }

    let recovered = Database::open(path_to_recovered)?;
    let mut recover = Self {
      db_to_recover,
      recovered_db: Some(recovered),
      sqlite_recover: std::ptr::null_mut(),
      step_callback: None,
    };

    recover.sqlite_recover = unsafe {
      sqlite3_recover_init_sql(
        recover.db_to_recover.sqlite_db,
        CString::new("main").unwrap().as_c_str().as_ptr(),
        Recover::recover_step,
        &mut recover as *mut Recover as *mut c_void,
      )
    };

    Ok(recover)
  }

  pub fn init(db_to_recover: Database, path_to_recovered: &str) -> Self {
    extern "C" {
      fn sqlite3_recover_init(
        db: *mut c_void,
        name: *const c_char,
        path_to_recovered: *const c_char,
      ) -> *mut c_void;
    }

    let mut recover = Self {
      db_to_recover,
      recovered_db: None,
      sqlite_recover: std::ptr::null_mut(),
      step_callback: None,
    };

    recover.sqlite_recover = unsafe {
      sqlite3_recover_init(
        recover.db_to_recover.sqlite_db,
        CString::new("main").unwrap().as_c_str().as_ptr(),
        CString::new(path_to_recovered).unwrap().as_c_str().as_ptr(),
      )
    };

    recover
  }

  extern "C" fn recover_step(db: *mut c_void, sql: *const c_char) -> c_int {
    let db = db as *mut Recover;
    unsafe {
      if let Some(callback) = (*db).step_callback {
        callback(CStr::from_ptr(sql).to_str().unwrap());
      }
    }
    SQLITE_OK
  }

  pub fn configure(&mut self, config: RecoverConfig) {
    extern "C" {
      fn sqlite3_recover_config(recover: *mut c_void, op: c_int, arg: *mut c_void);
    }
    self.step_callback = config.step_callback;

    // default is 0
    if config.recover_rowids {
      unsafe {
        sqlite3_recover_config(
          self.sqlite_recover,
          SQLITE_RECOVER_ROWIDS,
          &mut 1 as *mut i32 as *mut c_void,
        );
      }
    }

    // default is 1
    if !config.slow_indexes {
      unsafe {
        sqlite3_recover_config(
          self.sqlite_recover,
          SQLITE_RECOVER_SLOWINDEXES,
          &mut 0 as *mut i32 as *mut c_void,
        );
      }
    }

    unsafe {
      if let Some(lost_and_found) = config.lost_and_found {
        sqlite3_recover_config(
          self.sqlite_recover,
          SQLITE_RECOVER_LOST_AND_FOUND,
          CString::new(lost_and_found.name.as_str())
            .unwrap()
            .as_c_str()
            .as_ptr() as *mut c_void,
        );

        // default is 0
        if lost_and_found.recover_freelist {
          sqlite3_recover_config(
            self.sqlite_recover,
            SQLITE_RECOVER_FREELIST_CORRUPT,
            &mut 1 as *mut i32 as *mut c_void,
          );
        }
      } else {
        sqlite3_recover_config(
          self.sqlite_recover,
          SQLITE_RECOVER_LOST_AND_FOUND,
          std::ptr::null_mut(),
        )
      }
    };
  }

  pub fn run(&self) -> Result<(), SQLiteError> {
    extern "C" {
      fn sqlite3_recover_run(recover: *mut c_void) -> c_int;
      fn sqlite3_recover_errmsg(recover: *mut c_void) -> *const c_char;
      fn sqlite3_recover_errcode(recover: *mut c_void) -> c_int;
    }

    let err_code = unsafe {
      sqlite3_recover_run(self.sqlite_recover);
      sqlite3_recover_errcode(self.sqlite_recover)
    };

    if err_code != 0 {
      return Err(SQLiteError {
        code: err_code,
        message: unsafe {
          CStr::from_ptr(sqlite3_recover_errmsg(self.sqlite_recover))
            .to_str()
            .unwrap()
            .to_string()
        },
      });
    }

    Ok(())
  }
}

impl Drop for Recover {
  fn drop(&mut self) {
    extern "C" {
      fn sqlite3_recover_finish(recover: *mut c_void) -> c_int;
    }
    unsafe { sqlite3_recover_finish(self.sqlite_recover) };
  }
}
