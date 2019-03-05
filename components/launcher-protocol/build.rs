use prost_build;

/// Automatically generate Rust code from our protobuf definitions at
/// compile time.
///
/// Generated code is deposited in `OUT_DIR` and automatically
/// `include!`-ed in our Rust modules, per standard Prost practice.

fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_protos(&["protocols/error.proto",
                            "protocols/launcher.proto",
                            "protocols/net.proto",
                            "protocols/supervisor.proto"],
                          &["protocols/"])
          .unwrap()
}
