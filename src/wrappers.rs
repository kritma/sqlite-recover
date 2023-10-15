pub mod database;

const SQLITE_OK: i32 = 0;
const SQLITE_RECOVER_LOST_AND_FOUND: i32 = 1;
const SQLITE_RECOVER_FREELIST_CORRUPT: i32 = 2;
const SQLITE_RECOVER_ROWIDS: i32 = 3;
const SQLITE_RECOVER_SLOWINDEXES: i32 = 4;

pub struct SQLiteError {
  pub code: i32,
  pub message: String,
}

pub type StepCallback = Box<dyn Fn(SQLiteError)>;
