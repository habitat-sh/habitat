extern crate gcc;

fn main() {
    gcc::compile_library("libsid.a", &["./src/obtain_sid.c"]);
}
