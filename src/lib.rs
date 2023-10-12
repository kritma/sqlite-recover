#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn recover(path: String, recovered: String) -> String {
  if let Ok(db) = sqlite::open_db(path.as_str()) {
    if let Ok(()) = sqlite::recover_db(db, recovered.as_str()) {
      return "recovered successfully".to_string();
    }
  }
  return "cant recover".to_string();
}

mod sqlite;
