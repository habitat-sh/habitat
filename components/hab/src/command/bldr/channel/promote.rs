use crate::{PRODUCT,
            VERSION,
            api_client::{self,
                         Client,
                         builder::{ChannelCompatibilityError,
                                   ChannelSnapshot}},
            common::ui::{Status,
                         UI,
                         UIWriter},
            error::{Error,
                    Result}};
use habitat_core::{ChannelIdent,
                   origin::Origin};
use reqwest::StatusCode;

#[derive(Clone, Copy, Debug)]
pub(crate) struct PromoteOptions {
    pub check:    bool,
    pub snapshot: bool,
    pub json:     bool,
}

pub(crate) async fn start(ui: &mut UI,
                          bldr_url: &str,
                          token: &str,
                          origin: &Origin,
                          source_channel: &ChannelIdent,
                          target_channel: &ChannelIdent,
                          options: PromoteOptions)
                          -> Result<()> {
    let PromoteOptions { check,
                         snapshot,
                         json, } = options;
    let api_client = Client::new(bldr_url, PRODUCT, VERSION, None).map_err(Error::APIClient)?;

    if !json {
        ui.status(Status::Promoting,
                  format!("Packages from channel {} to {}.",
                          source_channel, target_channel))?;
    }

    // Preserve the original request and response handling for unflagged promotion.
    if !check && !snapshot {
        api_client.promote_channel_packages(origin, token, source_channel, target_channel)
                  .await
                  .map_err(Error::APIClient)?;
        if !json {
            ui.status(Status::Promoted,
                      format!("packages from channel {} to {}.",
                              source_channel, target_channel))?;
        }
        return Ok(());
    }

    let response = match api_client.promote_channel_packages_with_options(origin,
                                                                          token,
                                                                          source_channel,
                                                                          target_channel,
                                                                          check,
                                                                          snapshot)
                                   .await
    {
        Ok(response) => response,
        Err(api_client::Error::APIError(StatusCode::CONFLICT, body)) if check => {
            match serde_json::from_str::<ChannelCompatibilityError>(&body) {
                Ok(conflict) if conflict.error == "compatibility_check_failed" => {
                    if json {
                        println!("{body}");
                    } else {
                        println!("{}", format_conflict(&conflict));
                    }
                    return Err(Error::ArgumentError("Compatibility check failed; no \
                                                     packages were promoted."
                                                                             .into()));
                }
                _ => {
                    return Err(Error::APIClient(api_client::Error::APIError(StatusCode::CONFLICT,
                                                                            body)));
                }
            }
        }
        Err(error) => return Err(Error::APIClient(error)),
    };

    if !json {
        ui.status(Status::Promoted,
                  format!("packages from channel {} to {}.",
                          source_channel, target_channel))?;
    }

    if check && response.snapshot.is_some() && !json {
        println!("Compatibility check passed.");
    }

    if snapshot {
        match response.snapshot.as_ref() {
            Some(snapshot_response) => {
                if json {
                    println!("{}",
                             response.body
                                     .as_ref()
                                     .expect("snapshot response has a body"));
                } else {
                    println!("{}", format_snapshot(snapshot_response));
                }
            }
            None if response.body.is_none() => {
                eprintln!("No snapshot was created: Builder returned an empty response and may \
                           have ignored --snapshot.");
            }
            None => {
                eprintln!("Builder returned JSON without a snapshot channel; no snapshot was \
                           confirmed.")
            }
        }
    }

    Ok(())
}

fn format_conflict(conflict: &ChannelCompatibilityError) -> String {
    let mut lines = vec!["Compatibility check failed:".to_string()];
    for (target, packages) in &conflict.details {
        lines.push(format!("  Target {target}:"));
        for (package, detail) in packages {
            lines.push(format!("    {package}:"));
            for ident in &detail.idents {
                lines.push(format!("      {}", ident.ident));
                for carrier in &ident.introduced_by.source.examples {
                    lines.push(format!("        source: {} ({})",
                                       carrier.package, carrier.relation));
                }
                for carrier in &ident.introduced_by.target_only.packages {
                    lines.push(format!("        target: {} ({})",
                                       carrier.package, carrier.relation));
                }
                if ident.introduced_by.target_only.truncated {
                    lines.push(format!("        target carriers shown: {} of {}",
                                       ident.introduced_by.target_only.packages.len(),
                                       ident.introduced_by.target_only.total_count));
                }
            }
        }
    }
    if conflict.conflict_within_source {
        lines.push("The source channel also contains a compatibility conflict.".to_string());
    }
    lines.join("\n")
}

fn format_snapshot(snapshot: &ChannelSnapshot) -> String {
    let mut lines = vec![format!("Snapshot channel: {}", snapshot.snapshot_channel),
                         "Packages (including transitive dependencies):".to_string()];
    for (target, origins) in &snapshot.packages {
        lines.push(format!("  Target {target}:"));
        let mut idents = Vec::new();
        for packages in origins.values() {
            for entries in packages.values() {
                idents.extend(entries.iter().map(|entry| entry.ident.as_str()));
            }
        }
        idents.sort_unstable();
        for ident in idents {
            lines.push(format!("    {ident}"));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_conflict_with_carriers_and_source_note() {
        let body = r#"{"error":"compatibility_check_failed","conflicts":{"x86_64-linux":{"core/openssl":["core/openssl/1/1","core/openssl/2/1"]}},"conflict_within_source":true,"details":{"x86_64-linux":{"core/openssl":{"summary":"openssl has 2 versions","idents":[{"ident":"core/openssl/1/1","introduced_by":{"source":{"total_count":1,"examples":[{"package":"core/app/1/1","relation":"tdep"}]},"target_only":{"total_count":0,"truncated":false,"packages":[]}}},{"ident":"core/openssl/2/1","introduced_by":{"source":{"total_count":0,"examples":[]},"target_only":{"total_count":1,"truncated":false,"packages":[{"package":"core/other/1/1","relation":"head"}]}}}]}}}}"#;
        let conflict: ChannelCompatibilityError = serde_json::from_str(body).unwrap();
        let text = format_conflict(&conflict);
        for expected in ["Target x86_64-linux",
                         "core/openssl/1/1",
                         "core/openssl/2/1",
                         "source: core/app/1/1 (tdep)",
                         "target: core/other/1/1 (head)",
                         "source channel also contains"]
        {
            assert!(text.contains(expected), "missing {expected}: {text}");
        }
    }

    #[test]
    fn renders_snapshot_channel_and_package_list() {
        let body = r#"{"snapshot_channel":"stable_SS_example","packages":{"x86_64-linux":{"core":{"app":[{"ident":"core/app/1/1"}],"openssl":[{"ident":"core/openssl/3/1"}]}}}}"#;
        let snapshot: ChannelSnapshot = serde_json::from_str(body).unwrap();
        let text = format_snapshot(&snapshot);
        assert!(text.contains("Snapshot channel: stable_SS_example"));
        assert!(text.contains("Target x86_64-linux"));
        assert!(text.contains("core/app/1/1"));
        assert!(text.contains("core/openssl/3/1"));
    }
}
