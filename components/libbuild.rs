mod habitat {
    use std::{env,
              fs::{self,
                   File},
              io::Write,
              path::Path};

    /// Writes version information to `$OUT_DIR/VERSION` during
    /// compilation, in order to be picked up by `include!` macros
    /// elsewhere in the compiling code.
    pub fn common() { write_out_dir_file("VERSION", version()); }

    /// Reads from $PLAN_VERSION in a `hab pkg build` run, but from
    /// the `VERSION` file in a plain `cargo build` run.
    fn version() -> String { env::var("PLAN_VERSION").unwrap_or_else(|_| read_common_version()) }

    /// Reads the contents of the `VERSION` file at the repository root.
    fn read_common_version() -> String {
        let version_file = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).parent()
                                                                              .unwrap()
                                                                              .parent()
                                                                              .unwrap()
                                                                              .join("VERSION");
        fs::read_to_string(version_file).expect("Couldn't read VERSION file!")
    }

    /// Write the given `content` to `$OUT_DIR/filename`.
    fn write_out_dir_file(filename: &str, content: String) {
        let mut f = File::create(
            Path::new(&env::var("OUT_DIR").unwrap())
                .join(filename),
        )
        .expect("Failed to create OUT_DIR file");
        f.write_all(content.trim().as_bytes())
         .expect("Failed to write to OUT_DIR file");
    }
}
