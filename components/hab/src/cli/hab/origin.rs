use super::util::{AuthToken,
                  BldrUrl,
                  CacheKeyPath,
                  ConfigOptAuthToken,
                  ConfigOptBldrUrl,
                  ConfigOptCacheKeyPath};
use crate::cli::valid_origin;
use configopt::ConfigOpt;
use habitat_core::crypto::keys::PairType;
use std::path::PathBuf;
use structopt::{clap::ArgGroup,
                StructOpt};

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat Builder origins
pub enum Origin {
    /// Creates a new Builder origin
    Create {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        /// The origin to be created
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:     String,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Removes an unused/empty origin
    Delete {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:     String,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Departs membership from selected origin
    Depart {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:     String,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Displays general information about an origin
    Info {
        /// The origin name to be queried
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:     String,
        /// Output will be rendered in json
        #[structopt(name = "TO_JSON", short = "j", long = "json")]
        to_json:    bool,
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    Invitations(Invitations),
    Key(Key),
    Secret(Secret),
    /// Transfers ownership of an origin to another member of that origin
    Transfer {
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:            String,
        #[structopt(flatten)]
        bldr_url:          BldrUrl,
        #[structopt(flatten)]
        auth_token:        AuthToken,
        /// The account name of the new origin owner
        #[structopt(name = "NEW_OWNER_ACCOUNT")]
        new_owner_account: String,
    },
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Manage origin member invitations
pub enum Invitations {
    /// Accept an origin member invitation
    Accept {
        /// The origin name the invitation applies to
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:        String,
        /// The id of the invitation to accept
        #[structopt(name = "INVITATION_ID")]
        invitation_id: u64,
        #[structopt(flatten)]
        bldr_url:      BldrUrl,
        #[structopt(flatten)]
        auth_token:    AuthToken,
    },
    /// Ignore an origin member invitation
    Ignore {
        /// The origin name the invitation applies to
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:        String,
        /// The id of the invitation to ignore
        #[structopt(name = "INVITATION_ID")]
        invitation_id: u64,
        #[structopt(flatten)]
        bldr_url:      BldrUrl,
        #[structopt(flatten)]
        auth_token:    AuthToken,
    },
    /// List origin invitations sent to your account
    List {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// List pending invitations for a particular origin. Requires that you are the origin owner
    Pending {
        /// The name of the origin you wish to list invitations for
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:     String,
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        auth_token: AuthToken,
    },
    /// Rescind an existing origin member invitation
    Rescind {
        /// The origin name the invitation applies to
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:        String,
        /// The id of the invitation to rescind
        #[structopt(name = "INVITATION_ID")]
        invitation_id: u64,
        #[structopt(flatten)]
        bldr_url:      BldrUrl,
        #[structopt(flatten)]
        auth_token:    AuthToken,
    },
    /// Send an origin member invitation
    Send {
        /// The origin name the invitation applies to
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:          String,
        /// The account name to invite into the origin
        #[structopt(name = "INVITEE_ACCOUNT")]
        invitee_account: String,
        #[structopt(flatten)]
        bldr_url:        BldrUrl,
        #[structopt(flatten)]
        auth_token:      AuthToken,
    },
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat origin key maintenance
pub enum Key {
    /// Download origin key(s)
    Download {
        #[structopt(flatten)]
        cache_key_path:  CacheKeyPath,
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:          String,
        /// The origin key revision
        #[structopt(name = "REVISION")]
        revision:        Option<String>,
        #[structopt(flatten)]
        bldr_url:        BldrUrl,
        /// Download origin private key instead of origin public key
        #[structopt(name = "WITH_SECRET", short = "s", long = "secret")]
        with_secret:     bool,
        /// Download public encryption key instead of origin public key
        #[structopt(name = "WITH_ENCRYPTION", short = "e", long = "encryption")]
        with_encryption: bool,
        /// Authentication token for Builder (required for downloading origin private keys)
        #[structopt(name = "AUTH_TOKEN", short = "z", long = "auth")]
        auth_token:      Option<String>,
    },
    /// Outputs the latest origin key contents to stdout
    Export {
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:         String,
        /// Export either the 'public' or 'secret' key. The 'secret' key is the origin private key
        #[structopt(name = "PAIR_TYPE", short = "t", long = "type")]
        pair_type:      Option<PairType>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Generates a Habitat origin key pair
    Generate {
        /// The origin name
        #[structopt(name = "ORIGIN", validator = valid_origin)]
        origin:         Option<String>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Reads a stdin stream containing a public or private origin key contents and writes the key
    /// to disk
    Import {
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
    /// Upload origin keys to Builder
    Upload {
        #[structopt(flatten)]
        upload:         UploadGroup,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
        /// Upload origin private key in addition to the public key
        #[structopt(name = "WITH_SECRET",
                    short = "s",
                    long = "secret",
                    conflicts_with = "PUBLIC_FILE")]
        with_secret:    bool,
        /// Path to a local origin private key file on disk
        #[structopt(name = "SECRET_FILE", long = "secfile", conflicts_with = "ORIGIN")]
        secret_file:    Option<PathBuf>,
        #[structopt(flatten)]
        bldr_url:       BldrUrl,
        #[structopt(flatten)]
        auth_token:     AuthToken,
    },
}

#[derive(ConfigOpt, StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("upload").required(true), no_version)]
pub struct UploadGroup {
    /// The origin name
    #[structopt(name = "ORIGIN", validator = valid_origin, group = "upload")]
    origin:      Option<String>,
    /// Path to a local public origin key file on disk
    #[structopt(name = "PUBLIC_FILE", long = "pubfile", group = "upload")]
    public_file: Option<PathBuf>,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands related to secret management
pub enum Secret {
    /// Delete a secret for your origin
    Delete {
        /// The name of the variable key to be injected into the studio
        #[structopt(name = "KEY_NAME")]
        key_name:   String,
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        auth_token: AuthToken,
        /// The origin for which the secret will be deleted. Default is from 'HAB_ORIGIN' or
        /// cli.toml
        #[structopt(name = "ORIGIN", short = "o", long = "origin", validator = valid_origin)]
        origin:     Option<String>,
    },
    /// List all secrets for your origin
    List {
        #[structopt(flatten)]
        bldr_url:   BldrUrl,
        #[structopt(flatten)]
        auth_token: AuthToken,
        /// The origin for which secrets will be listed. Default is from 'HAB_ORIGIN' or cli.toml
        #[structopt(name = "ORIGIN", short = "o", long = "origin", validator = valid_origin)]
        origin:     Option<String>,
    },
    /// Create and upload a secret for your origin
    Upload {
        /// The name of the variable key to be injected into the studio. Ex: KEY="some_value"
        #[structopt(name = "KEY_NAME")]
        key_name:       String,
        /// The contents of the variable to be injected into the studio
        #[structopt(name = "SECRET")]
        secret:         String,
        #[structopt(flatten)]
        bldr_url:       BldrUrl,
        #[structopt(flatten)]
        auth_token:     AuthToken,
        /// The origin for which the secret will be uploaded. Default is from 'HAB_ORIGIN' or
        /// cli.toml
        #[structopt(name = "ORIGIN", short = "o", long = "origin", validator = valid_origin)]
        origin:         Option<String>,
        #[structopt(flatten)]
        cache_key_path: CacheKeyPath,
    },
}
