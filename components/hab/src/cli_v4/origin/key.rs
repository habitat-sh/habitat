// Implemenatation of `hab origin key`

use clap_v4 as clap;

use crate::{cli_v4::utils::{BldrUrl,
                            AuthToken,
                            CacheKeyPath,
                            UploadGroup,
                            valid_origin},
            error::{Result as HabResult,
                    Error},
            command::origin::key,
            key_type::KeyType};
use clap::Parser;

use habitat_common::ui::UI;

use std::{io::{self, Read},
        path::PathBuf};

use habitat_core::{crypto::keys::KeyCache,
                    origin::Origin};

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) enum OriginKeyCommand {
    /// Download origin key(s)
    Download {
        #[command(flatten)]
        cache_key_path: CacheKeyPath,

        /// The origin name
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin:     String,

        /// The origin key revision
        #[arg(name = "REVISION")]
        revision:        Option<String>,

        #[command(flatten)]
        bldr_url:   BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,

        /// Download origin private key instead of origin public key
        #[arg(name = "WITH_SECRET", short = 's', long = "secret")]
        with_secret:     bool,

        /// Download public encryption key instead of origin public key
        #[arg(name = "WITH_ENCRYPTION", short = 'e', long = "encryption")]
        with_encryption: bool,
    },

    /// Outputs the latest origin key contents to stdout
    Export {
        /// The origin name
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin:     String,

        /// Export either the 'public' or 'secret' key. The 'secret' key is the origin private key
        #[arg(name = "KEY_TYPE", short = 't', long = "type")]
        key_type:       Option<KeyType>,

        #[command(flatten)]
        cache_key_path: CacheKeyPath,
    },

    /// Generates a Habitat origin key pair
    Generate {
        /// The origin name 
        #[arg(name = "ORIGIN", value_parser = valid_origin)]
        origin:     Option<String>,

        #[command(flatten)]
        cache_key_path: CacheKeyPath,
    },

    /// Reads a stdin stream containing a public or private origin key contents and writes the key
    /// to disk
    Import {
        #[command(flatten)]
        cache_key_path: CacheKeyPath,
    },

    /// Upload origin keys to Builder
    Upload {
        #[command(flatten)]
        upload:         UploadGroup,

        #[command(flatten)]
        cache_key_path: CacheKeyPath,

        /// Upload origin private key in addition to the public key
        #[arg(name = "WITH_SECRET", short = 's', long = "secret", conflicts_with = "public_file")]
        with_secret:    bool,

        /// Path to a local origin private key file on disk
        #[arg(name = "SECRET_FILE", long = "secfile", conflicts_with = "origin")]
        secret_file:    Option<PathBuf>,

        #[command(flatten)]
        bldr_url:   BldrUrl,

        #[command(flatten)]
        auth_token: AuthToken,
    },
}

impl OriginKeyCommand {
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        match self {
            OriginKeyCommand::Download {
                cache_key_path,
                origin,
                revision,
                bldr_url,
                auth_token,
                with_secret,
                with_encryption,
            } => {
                let endpoint = bldr_url.to_string();
                let token = auth_token.try_from_cli_or_config();
                let origin_obj: Origin = origin.parse()
                                        .map_err(Error::from)?;
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;
                key::download::start(
                    ui,
                    &endpoint,
                    &origin_obj,
                    revision.as_deref(),
                    *with_secret,
                    *with_encryption,
                    token.as_deref(),
                    &key_cache,
                )
                .await
            }

            OriginKeyCommand::Export {origin, key_type, cache_key_path} => {
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;
                let kt = key_type.clone().unwrap_or(KeyType::Public);
                let origin_obj: Origin = origin.parse()
                                        .map_err(Error::from)?;
                key::export::start(&origin_obj, kt, &key_cache)?;
                Ok(())
            }

            OriginKeyCommand::Generate {origin, cache_key_path} => {
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;
                // Make sure we actually got an origin on the CLI:
                let origin_str: &str = origin
                    .as_deref()
                    .ok_or_else(|| Error::ArgumentError("ORIGIN is required".into()))?;

                // Parse it into your Origin newtype
                let origin_obj: Origin = origin_str
                    .parse()
                    .map_err(Error::from)?;

                key::generate::start(ui, &origin_obj, &key_cache)?;
                Ok(())
            }

            OriginKeyCommand::Import { cache_key_path } => {
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;
                let mut content = String::new();
                io::stdin().read_to_string(&mut content)?;
                key::import::start(ui, content.trim(), &key_cache)?;
                Ok(())
            }

            OriginKeyCommand::Upload {
                upload,
                cache_key_path,
                with_secret,
                secret_file,
                bldr_url,
                auth_token,
            } => {
                let endpoint = bldr_url.to_string();
                let token = auth_token.from_cli_or_config().map_err(|e| Error::ArgumentError(e.to_string()))?;
                let cache_dir: PathBuf = cache_key_path.cache_key_path.clone();
                let key_cache = KeyCache::new(cache_dir);
                key_cache.setup().map_err(Error::from)?;

                if let Some(orig) = &upload.origin {
                    let origin_obj: Origin = orig
                                            .parse()
                                            .map_err(Error::from)?;
                    // upload latest
                    key::upload_latest::start(
                        ui,
                        &endpoint,
                        &token,
                        &origin_obj,
                        *with_secret,
                        &key_cache,
                    )
                    .await
                } else {
                    // upload specific files
                    let pub_path = upload
                        .public_file
                        .as_ref()
                        .expect("PUBLIC_FILE or ORIGIN is required");
                    key::upload::start(
                        ui,
                        &endpoint,
                        &token,
                        pub_path.as_path(),
                        secret_file.as_deref(),
                    )
                    .await
                }
            }
        }
    }
}