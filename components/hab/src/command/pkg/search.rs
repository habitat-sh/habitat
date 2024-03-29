use crate::{api_client::Client,
            error::Result,
            PRODUCT,
            VERSION};

pub async fn start(st: &str, bldr_url: &str, limit: usize, token: Option<&str>) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    let (packages, total) = api_client.search_package(st, limit, token).await?;
    match packages.len() {
        0 => eprintln!("No packages found that match '{}'", st),
        _ => {
            for p in &packages {
                if let (Some(version), Some(release)) = (&p.version, &p.release) {
                    println!("{}/{}/{}/{}", p.origin, p.name, version, release);
                } else {
                    println!("{}/{}", p.origin, p.name);
                }
            }
            if packages.len() < total {
                eprintln!("Search returned too many items, only showing the first {} of {}",
                          packages.len(),
                          total);
            }
        }
    }
    Ok(())
}
