use crate::{api_client::Client,
            error::Result,
            PRODUCT,
            VERSION};

pub fn start(st: &str, bldr_url: &str, token: Option<&str>) -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None)?;
    let (packages, more) = api_client.search_package(st, token)?;
    match packages.len() {
        0 => println!("No packages found that match '{}'", st),
        _ => {
            for p in &packages {
                if let (&Some(ref version), &Some(ref release)) = (&p.version, &p.release) {
                    println!("{}/{}/{}/{}", p.origin, p.name, version, release);
                } else {
                    println!("{}/{}", p.origin, p.name);
                }
            }
            if more {
                println!("Search returned too many items, only showing the first {}",
                         packages.len());
            }
        }
    }
    Ok(())
}
