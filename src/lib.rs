#![deny(clippy::all)]

use std::ffi::c_int;

use sqlite::database::{
  recover::{self, Recover},
  Database,
};

#[macro_use]
extern crate napi_derive;

fn print_sql(sql: &str) {
  println!("{}", sql);
}

#[napi]
pub fn recover(path: String, recovered: String) -> String {
  match Database::open(path.as_str()) {
    Ok(db) => match Recover::init(db, recovered.as_str()) {
      Ok(mut recover) => {
        recover.configure({
          recover::RecoverConfig {
            lost_and_found: None,
            recover_rowids: true,
            slow_indexes: false,
            callback: Some(print_sql),
          }
        });
        match recover.run() {
          Ok(()) => "recovered".to_string(),
          Err(err) => "run suck".to_string(),
        }
      }
      Err(err) => "init suck".to_string(),
    },
    Err(err) => "open suck".to_string(),
  }
}

#[napi]
pub fn get() -> i32 {
  extern "C" {
    fn sum(a: c_int, b: c_int) -> c_int;
  }
  unsafe { sum(5, 3) }
}
mod sqlite;
