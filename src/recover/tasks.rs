use napi::{bindgen_prelude::Undefined, Env, Error, Result, Task};

use crate::wrappers::SQLiteError;

use super::{recover, recover_sql};

pub struct RecoverTask {
  path: String,
  recovered: String,
}

impl RecoverTask {
  pub fn new(path: String, recovered: String) -> Self {
    Self { path, recovered }
  }
}

#[napi]
impl Task for RecoverTask {
  type Output = ();
  type JsValue = Undefined;

  fn compute(&mut self) -> Result<Self::Output> {
    if let Err(err) = recover(&self.path, &self.recovered) {
      return Err(Error::from_reason(err.message));
    }
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }

  fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
    Err(err)
  }
}

pub struct RecoverSQLTask {
  path: String,
  recovered: String,
  step_callback: Option<Box<dyn Fn(SQLiteError) + Send>>,
}

impl RecoverSQLTask {
  pub fn new(
    path: String,
    recovered: String,
    step_callback: Box<dyn Fn(SQLiteError) + Send>,
  ) -> Self {
    Self {
      path,
      recovered,
      step_callback: Some(step_callback),
    }
  }
}

#[napi]
impl Task for RecoverSQLTask {
  type Output = ();
  type JsValue = Undefined;

  fn compute(&mut self) -> Result<Self::Output> {
    if let Err(err) = recover_sql(
      &self.path,
      &self.recovered,
      self.step_callback.take().unwrap(),
    ) {
      return Err(Error::from_reason(err.message));
    }
    Ok(())
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }

  fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
    Err(err)
  }
}
