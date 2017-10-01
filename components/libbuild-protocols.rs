mod protocols {
    extern crate pkg_config;

    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    pub fn generate_if_feature_enabled() {
        if env::var("CARGO_FEATURE_PROTOCOLS").is_ok() {
            generate_protocols();
        }
    }

    fn generate_protocols() {
        let prefix = match env::var("PROTOBUF_PREFIX").ok() {
            Some(prefix) => prefix,
            None => {
                match pkg_config::get_variable("protobuf", "prefix") {
                    Ok(prefix) => prefix,
                    Err(msg) => panic!("Unable to locate protobuf, err={:?}", msg),
                }
            }
        };

        let out_dir = r"src/message";
        let cmd = Command::new(format!("{}/bin/protoc", prefix))
            .arg("--rust_out")
            .arg(out_dir)
            .args(&protocol_files())
            .output();
        match cmd {
            Ok(out) => {
                if !out.status.success() {
                    panic!("{:?}", out)
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn protocol_files() -> Vec<PathBuf> {
        let mut files = vec![];
        for entry in fs::read_dir("protocols").unwrap() {
            let file = entry.unwrap();
            // skip vim temp files
            if file.file_name().to_str().unwrap().starts_with(".") {
                continue;
            }
            if file.metadata().unwrap().is_file() {
                files.push(file.path());
            }
        }
        files
    }
}
