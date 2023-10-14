pub mod database;

pub struct SQLiteError {
  pub code: i32,
  pub message: String,
}
