use crate::wrappers::{database::Database, SQLiteError, StepCallback};

pub mod tasks;

pub fn recover(path: &str, recovered: &str) -> Result<(), SQLiteError> {
  Database::open(path)?.recover_to(recovered)
}

pub fn recover_sql(
  path: &str,
  recovered: &str,
  step_callback: StepCallback,
) -> Result<(), SQLiteError> {
  Database::open(path)?.recover_sql_to(recovered, step_callback)
}
