extern crate cc;
extern crate napi_build;

fn main() {
  cc::Build::new()
    .file("./src/sqlite-lib/sqlite3.c")
    .file("./src/sqlite-lib/dbdata.c")
    .file("./src/sqlite-lib/sqlite3recover.c")
    .include("src/sqlite-lib")
    .define("SQLITE_ENABLE_DBPAGE_VTAB", None)
    // .static_flag(true)
    // .shared_flag(true)
    .compile("sqlite_with_recover");

  napi_build::setup();
}
