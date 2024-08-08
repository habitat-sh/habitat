#[cfg(feature = "v2")]
mod main_v2;

mod main_v4;

#[tokio::main]
async fn main() {
    if cfg!(feature = "v4") {
        main_v4::main_v4().await
    } else {
        #[cfg(feature = "v2")]
        main_v2::main_v2().await
    }
}
