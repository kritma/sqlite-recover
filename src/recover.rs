use crate::wrappers::{Database, SQLiteError, StepCallback};

mod tasks;

pub use tasks::{RecoverSQLTask, RecoverTask};

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
