#[macro_use]
extern crate napi_derive;

use sqlite::database::Database;

mod sqlite;

fn print_sql(sql: &str) {
  println!("{}", sql);
}

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
