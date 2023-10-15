#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::{AbortSignal, AsyncTask},
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  Env, Error, JsFunction,
};
use recover::tasks::{Recover, RecoverBySQL};
use wrappers::SQLiteError;

mod recover;
mod wrappers;

#[napi]
pub fn recover(path: String, recovered: String) -> Option<String> {
  if let Err(err) = recover::recover(&path, &recovered) {
    return Some(err.message);
  }
  None
}

#[napi]
pub fn recover_sql(
  env: Env,
  path: String,
  recovered: String,
  #[napi(ts_arg_type = "(err: Error) => void")] step_callback: JsFunction,
) -> Option<String> {
  if let Err(err) = recover::recover_sql(
    &path,
    &recovered,
    Box::new(move |err: SQLiteError| {
      let _ = step_callback.call(
        None,
        &[env.create_error(Error::from_reason(err.message)).unwrap()],
      );
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
  #[napi(ts_arg_type = "(Error) => void")] step_callback: JsFunction,
  signal: AbortSignal,
) -> AsyncTask<RecoverBySQL> {
  let thread_safe_cb: ThreadsafeFunction<SQLiteError, ErrorStrategy::CalleeHandled> = step_callback
    .create_threadsafe_function(10, |ctx: ThreadSafeCallContext<SQLiteError>| {
      Ok(vec![ctx
        .env
        .create_error(Error::from_reason(ctx.value.message))])
    })
    .unwrap();
  AsyncTask::with_signal(
    RecoverBySQL::new(
      path,
      recovered,
      Box::new(move |err: SQLiteError| {
        thread_safe_cb.call(Ok(err), ThreadsafeFunctionCallMode::Blocking);
      }),
    ),
    signal,
  )
}
