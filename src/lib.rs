#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask},
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
  JsFunction,
};
use sqlite::recover::{
  sync_functions::{recover_sql_sync, recover_sync},
  tasks::{Recover, RecoverBySQL},
};

mod sqlite;

#[napi]
pub fn recover(path: String, recovered: String) -> Option<String> {
  if let Err(err) = recover_sync(&path, &recovered) {
    return Some(err.message);
  }
  None
}

#[napi]
pub fn recover_sql(path: String, recovered: String, step_callback: JsFunction) -> Option<String> {
  if let Err(err) = recover_sql_sync(
    &path,
    &recovered,
    Box::new(move || {
      let _ = step_callback.call_without_args(None);
    }),
  ) {
    return Some(err.message);
  }
  None
}

#[napi]
pub fn recover_async(path: String, recovered: String, signal: AbortSignal) -> AsyncTask<Recover> {
  AsyncTask::with_signal(Recover::new(path, recovered), signal)
}

#[napi]
pub fn recover_sql_async(
  path: String,
  recovered: String,
  step_callback: JsFunction,
  signal: AbortSignal,
) -> AsyncTask<RecoverBySQL> {
  let thread_safe_cb: ThreadsafeFunction<(), ErrorStrategy::CalleeHandled> = step_callback
    .create_threadsafe_function(0, |_| Ok(vec![()]))
    .unwrap();
  AsyncTask::with_signal(
    RecoverBySQL::new(
      path,
      recovered,
      Box::new(move || {
        thread_safe_cb.call(Ok(()), ThreadsafeFunctionCallMode::Blocking);
      }),
    ),
    signal,
  )
}
