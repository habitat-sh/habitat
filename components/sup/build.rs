// Inline common build behavior
include!("../libbuild.rs");

use prost_build;
use std::{env,
          fs::File,
          io::{self,
               Write},
          path::{Path,
                 PathBuf},
          process::{Command,
                    ExitStatus}};

fn main() {
    habitat::common();
    generate_apidocs();
    generate_event_protobufs();
}

fn generate_apidocs() {
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("api.html");
    match env::var("CARGO_FEATURE_APIDOCS") {
        Ok(_) => {
            let src = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("doc/api.raml");
            let cmd = raml2html_cmd(dst, src).expect("failed to compile html from raml");

            assert!(cmd.success());
        }
        Err(_) => {
            let mut file = File::create(dst).unwrap();
            file.write_all(b"No API docs provided at build").unwrap();
        }
    };
}

fn raml2html_cmd(dst: PathBuf, src: PathBuf) -> io::Result<ExitStatus> {
    let mut cmd = Command::new(if cfg!(windows) { "cmd" } else { "raml2html" });

    let cmd = if cfg!(windows) {
        // One would think we could directly call the .bat file
        // see https://github.com/rust-lang/rust/issues/42791
        // for why this does not work
        cmd.arg("/c").arg("raml2html.bat")
    } else {
        &mut cmd
    };

    cmd.arg("-i").arg(src).arg("-o").arg(dst).status()
}

fn generate_event_protobufs() {
    let mut config = prost_build::Config::new();
    config.compile_protos(&["protocols/event.proto"], &["protocols/"])
          .unwrap()
}
