extern crate cc;
extern crate napi_build;

fn main() {
  cc::Build::new()
    .file("./src/sqlite-lib/sqlite3.c")
    .file("./src/sqlite-lib/shell.c")
    .static_flag(true)
    .compile("sqlite");
  napi_build::setup();
}
