#[macro_use]
extern crate napi_derive;

use napi::{Callback, Error, JsFunction};
use sqlite::database::Database;

mod sqlite;

#[napi]
pub fn recover(path: String, recovered: String) -> String {
  match Database::open(path.as_str()) {
    Ok(db) => match db.recover_to(&recovered) {
      Ok(()) => "recovered".to_string(),
      Err(err) => "recover suck:".to_string() + err.message.as_str(),
    },
    Err(err) => "open suck: ".to_string() + err.message.as_str(),
  }
}

#[napi]
pub fn recover_sql(path: String, recovered: String, step_callback: JsFunction) -> String {
  match Database::open(path.as_str()) {
    Ok(db) => match db.recover_sql_to(
      &recovered,
      Box::new(move || {
        step_callback.call_without_args(None);
      }),
    ) {
      Ok(()) => "recovered".to_string(),
      Err(err) => "recover suck:".to_string() + err.message.as_str(),
    },
    Err(err) => "open suck: ".to_string() + err.message.as_str(),
  }
}
