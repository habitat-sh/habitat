use crate::{PRODUCT,
            VERSION,
            api_client::Client,
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::{ChannelIdent,
                   origin::Origin};

pub async fn start(ui: &mut UI,
                   bldr_url: &str,
                   origin: &Origin,
                   channel: &ChannelIdent,
                   token: Option<&str>,
                   limit: usize,
                   json: bool)
                   -> Result<()> {
    if limit == 0 {
        return Err(Error::ArgumentError("--limit must be greater than zero".into()));
    }
    let client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;
    if !json {
        ui.status(Status::Determining,
                  format!("packages in {}/{}.", origin, channel))?;
    }
    let packages = client.list_channel_packages(origin, channel, limit, token)
                         .await
                         .map_err(Error::APIClient)?;
    let idents: Vec<String> = packages.iter().map(ToString::to_string).collect();
    if json {
        println!("{}", serde_json::to_string(&idents)?);
    } else if idents.is_empty() {
        ui.para(&empty_channel_message(origin, channel))?;
    } else {
        for ident in idents {
            println!("{}", ident);
        }
    }
    Ok(())
}

fn empty_channel_message(origin: &Origin, channel: &ChannelIdent) -> String {
    format!("No packages found in {} channel for {} origin.",
            channel, origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_channel_message_names_channel_and_origin() {
        let origin: Origin = "chef".parse().unwrap();
        let channel: ChannelIdent = "build-20261104-221233-41923200607-built".parse().unwrap();
        assert_eq!(empty_channel_message(&origin, &channel),
                   "No packages found in build-20261104-221233-41923200607-built channel for \
                    chef origin.");
    }
}
