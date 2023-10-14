mod database;
pub mod recover;

const SQLITE_OK: i32 = 0;

pub struct SQLiteError {
  pub code: i32,
  pub message: String,
}

pub type StepCallback = Box<dyn Fn()>;
