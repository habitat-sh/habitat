use super::util::{AuthToken,
                  BldrOrigin,
                  BldrUrl,
                  CacheKeyPath,
                  ConfigOptAuthToken,
                  ConfigOptBldrOrigin,
                  ConfigOptBldrUrl,
                  ConfigOptCacheKeyPath};
use crate::{cli::valid_origin,
            key_type::KeyType};
use configopt::ConfigOpt;
use habitat_core::origin::OriginMemberRole;
use serde::Serialize;
use std::path::PathBuf;
use structopt::{clap::ArgGroup,
                StructOpt};

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat Builder origins
pub enum Origin {
    Create(OriginCreate),
    Delete(OriginDelete),
    Depart(OriginDepart),
    Info(OriginInfo),
    Invitations(Invitations),
    Key(Key),
    /// Role Based Access Control for origin members
    Rbac(Rbac),
    Secret(Secret),
    Transfer(OriginTransfer),
}

/// Creates a new Builder origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "create", no_version)]
pub struct OriginCreate {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    /// The origin to be created
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:     String,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Removes an unused/empty origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "delete", no_version)]
pub struct OriginDelete {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    /// The origin name
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:     String,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Departs membership from selected origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "depart", no_version)]
pub struct OriginDepart {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    /// The origin name
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:     String,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Displays general information about an origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "info", no_version)]
pub struct OriginInfo {
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
}

/// Transfers ownership of an origin to another member of that origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "transfer", no_version)]
pub struct OriginTransfer {
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
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Manage origin member invitations
pub enum Invitations {
    Accept(InvitationsAccept),
    Ignore(InvitationsIgnore),
    List(InvitationsList),
    Pending(InvitationsPending),
    Rescind(InvitationsRescind),
    Send(InvitationsSend),
}

/// Accept an origin member invitation
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "accept", no_version)]
pub struct InvitationsAccept {
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
}

/// Ignore an origin member invitation
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "ignore", no_version)]
pub struct InvitationsIgnore {
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
}
/// List origin invitations sent to your account
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "list", no_version)]
pub struct InvitationsList {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// List pending invitations for a particular origin. Requires that you are the origin owner
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "pending", no_version)]
pub struct InvitationsPending {
    /// The name of the origin you wish to list invitations for
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:     String,
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    auth_token: AuthToken,
}

/// Rescind an existing origin member invitation
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "rescind", no_version)]
pub struct InvitationsRescind {
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
}

/// Send an origin member invitation
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "send", no_version)]
pub struct InvitationsSend {
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
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat origin key maintenance
pub enum Key {
    Download(OriginKeyDownload),
    Export(OriginKeyExport),
    Generate(OriginKeyGenerate),
    Import(OriginKeyImport),
    Upload(OriginKeyUpload),
}

/// Outputs the latest origin key contents to stdout
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "export", no_version)]
pub struct OriginKeyExport {
    /// The origin name
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:         String,
    /// Export either the 'public' or 'secret' key. The 'secret' key is the origin private key
    #[structopt(name = "KEY_TYPE", short = "t", long = "type")]
    key_type:       Option<KeyType>,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Reads a stdin stream containing a public or private origin key contents and writes the key
/// to disk
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "import", no_version)]
pub struct OriginKeyImport {
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Upload origin keys to Builder
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "upload", no_version)]
pub struct OriginKeyUpload {
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
}

/// Generates a Habitat origin key pair
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "generate", no_version)]
pub struct OriginKeyGenerate {
    /// The origin name
    #[structopt(name = "ORIGIN", validator = valid_origin)]
    origin:         Option<String>,
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Download origin key(s)
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "download", no_version)]
pub struct OriginKeyDownload {
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
}

#[derive(ConfigOpt, StructOpt, Debug)]
#[configopt(derive(Serialize))]
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
#[structopt(no_version, rename_all = "screamingsnake")]
pub struct RbacShow {
    #[structopt(flatten)]
    pub origin:         BldrOrigin,
    /// The account name of the role to display
    pub member_account: String,
    #[structopt(flatten)]
    pub bldr_url:       BldrUrl,
    #[structopt(flatten)]
    pub auth_token:     AuthToken,
    /// Output will be rendered in json
    #[structopt(short = "j", long = "json")]
    pub to_json:        bool,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, rename_all = "screamingsnake")]
pub struct RbacSet {
    #[structopt(flatten)]
    pub origin:         BldrOrigin,
    /// The account name whose role will be changed
    pub member_account: String,
    /// The role name to enforce for the member account
    #[structopt(possible_values = &[OriginMemberRole::READONLY_MEMBER, OriginMemberRole::MEMBER, OriginMemberRole::MAINTAINER, OriginMemberRole::ADMINISTRATOR, OriginMemberRole::OWNER])]
    pub role:           OriginMemberRole,
    #[structopt(flatten)]
    pub bldr_url:       BldrUrl,
    #[structopt(flatten)]
    pub auth_token:     AuthToken,
    /// Do not prompt for confirmation
    #[structopt(short = "n", long = "no-prompt")]
    pub no_prompt:      bool,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "rbac", no_version)]
/// Role Based Access Control for origin members
pub enum Rbac {
    /// Display an origin member's current role
    Show(RbacShow),
    /// Change an origin member's role
    Set(RbacSet),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands related to secret management
pub enum Secret {
    Delete(SecretDelete),
    List(SecretList),
    Upload(SecretUpload),
}

/// Delete a secret for your origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "delete", no_version)]
pub struct SecretDelete {
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
}

/// List all secrets for your origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "list", no_version)]
pub struct SecretList {
    #[structopt(flatten)]
    bldr_url:   BldrUrl,
    #[structopt(flatten)]
    auth_token: AuthToken,
    /// The origin for which secrets will be listed. Default is from 'HAB_ORIGIN' or cli.toml
    #[structopt(name = "ORIGIN", short = "o", long = "origin", validator = valid_origin)]
    origin:     Option<String>,
}

/// Create and upload a secret for your origin
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "upload", no_version)]
pub struct SecretUpload {
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
}
