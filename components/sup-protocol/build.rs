use prost_build;

/// Automatically generate Rust code from our protobuf definitions at
/// compile time.
///
/// Generated code is deposited in `OUT_DIR` and automatically
/// `include!`-ed in our Rust modules, per standard Prost practice.

fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"kebab-case\")]");
    config.compile_protos(&["protocols/ctl.proto",
                            "protocols/net.proto",
                            "protocols/types.proto"],
                          &["protocols/"])
          .expect("Couldn't compile protobufs!");
}
