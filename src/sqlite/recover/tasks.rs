use napi::{bindgen_prelude::Undefined, Env, Error, Result, Task};

use crate::sqlite::database::Database;

use super::sync_functions::recover_sync;

pub struct Recover {
  path: String,
  recovered: String,
}

impl Recover {
  pub fn new(path: String, recovered: String) -> Self {
    Self { path, recovered }
  }
}

#[napi]
impl Task for Recover {
  type Output = ();
  type JsValue = Undefined;

  fn compute(&mut self) -> Result<Self::Output> {
    if let Err(err) = recover_sync(&self.path, &self.recovered) {
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

pub struct RecoverBySQL {
  path: String,
  recovered: String,
  step_callback: Vec<Box<dyn Fn() + Send>>,
}

impl RecoverBySQL {
  pub fn new(path: String, recovered: String, step_callback: Box<dyn Fn() + Send>) -> Self {
    Self {
      path,
      recovered,
      step_callback: vec![step_callback],
    }
  }
}

#[napi]
impl Task for RecoverBySQL {
  type Output = ();
  type JsValue = Undefined;

  fn compute(&mut self) -> Result<Self::Output> {
    match Database::open(&self.path) {
      Ok(db) => match db.recover_sql_to(&self.recovered, self.step_callback.pop().unwrap()) {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::from_reason(err.message)),
      },
      Err(err) => Err(Error::from_reason(err.message)),
    }
  }

  fn resolve(&mut self, _env: Env, _output: Self::Output) -> Result<Self::JsValue> {
    Ok(())
  }

  fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
    Err(err)
  }
}
