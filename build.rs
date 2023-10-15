extern crate cc;
extern crate napi_build;

fn main() {
  cc::Build::new()
    .include("./src/sqlite-amalgamation-3430200")
    .file("./src/sqlite-amalgamation-3430200/sqlite3.c")
    .file("./src/sqlite-amalgamation-3430200/shell.c")
    .define("SQLITE_ENABLE_DBPAGE_VTAB", None)
    // .static_flag(true)
    // .shared_flag(true)
    .compile("sqlite_with_recover");

  napi_build::setup();
}
