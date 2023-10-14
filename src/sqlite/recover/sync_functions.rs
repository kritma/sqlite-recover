use crate::sqlite::{database::Database, SQLiteError, StepCallback};

pub fn recover_sync(path: &str, recovered: &str) -> Result<(), SQLiteError> {
  Database::open(path)?.recover_to(recovered)
}

pub fn recover_sql_sync(
  path: &str,
  recovered: &str,
  step_callback: StepCallback,
) -> Result<(), SQLiteError> {
  Database::open(path)?.recover_sql_to(recovered, step_callback)
}
