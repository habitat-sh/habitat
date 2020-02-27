use prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".butterfly.newscast.Rumor.payload",
                          "#[allow(clippy::large_enum_variant)]");
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_protos(&["protocols/common.proto",
                            "protocols/newscast.proto",
                            "protocols/swim.proto"],
                          &["protocols/"])
          .expect("Couldn't compile protobufs!");
    for file in generated_files() {
        fs::copy(&file,
                 // NB: src/generated is presumed to exist; if you delete
                 // it, this'll fail.
                 format!("src/generated/{}",
                         file.file_name().unwrap().to_string_lossy())).expect("Could not copy \
                                                                               generated code to \
                                                                               src/generated");
    }
}

fn generated_files() -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(env::var("OUT_DIR").unwrap()).unwrap() {
        let file = entry.unwrap();
        if file.file_name().to_str().unwrap().ends_with(".rs") && file.metadata().unwrap().is_file()
        {
            files.push(file.path());
        }
    }
    files
}
