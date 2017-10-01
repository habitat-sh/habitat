// Inline common build protocols behavior
include!("../libbuild-protocols.rs");

fn main() {
    protocols::generate_if_feature_enabled();
}
