// This build script is distinct from the standard build.rs as it is only used
// for the BSDs which run a stripped down version. The `all.c` file is assumed
// to have been already generated for this build script.

extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("all.c")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Werror")
        .flag("-Wno-type-limits")
        .compile("liball.a");
}
