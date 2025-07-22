#[cfg(feature = "v2")]
mod main_v2;

mod main_v4;

fn main() {
    if cfg!(feature = "v4") {
        main_v4::main_v4()
    } else {
        #[cfg(feature = "v2")]
        main_v2::main_v2()
    }
}
