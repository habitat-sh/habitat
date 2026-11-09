use crate::{PRODUCT,
            VERSION,
            api_client::Client,
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::origin::Origin;

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   origin: &Origin,
                   include_sandbox_channels: bool,
                   filter: Option<&str>)
                   -> Result<()> {
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    ui.status(Status::Determining, format!("channels for {}.", origin))?;

    match api_client.list_channels(origin, include_sandbox_channels)
                    .await
    {
        Ok(channels) => {
            let channels = filter_channels(channels, filter);
            match filter {
                Some(f) if channels.is_empty() => println!("No channels found matching '{}'", f),
                _ => println!("{}", channels.join("\n")),
            }
            Ok(())
        }
        Err(e) => Err(Error::APIClient(e)),
    }
}

fn filter_channels(channels: Vec<String>, filter: Option<&str>) -> Vec<String> {
    match filter {
        Some(f) => channels.into_iter().filter(|c| c.contains(f)).collect(),
        None => channels,
    }
}

#[cfg(test)]
mod tests {
    use super::filter_channels;

    fn chans() -> Vec<String> {
        ["stable", "unstable", "Stable-x", "dev"].iter()
                                                 .map(ToString::to_string)
                                                 .collect()
    }

    #[test]
    fn no_filter_returns_all() {
        assert_eq!(filter_channels(chans(), None), chans());
    }

    #[test]
    fn filter_is_case_sensitive_substring() {
        assert_eq!(filter_channels(chans(), Some("stable")),
                   vec!["stable", "unstable"]);
    }

    #[test]
    fn no_match_is_empty() {
        assert!(filter_channels(chans(), Some("zzz")).is_empty());
    }
}
