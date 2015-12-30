// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate bldr;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;

use docopt::Docopt;
use std::process;
use ansi_term::Colour::Yellow;
use std::ffi::CString;
use std::ptr;

use bldr::config::{Command, Config};
use bldr::error::{BldrResult, BldrError, ErrorKind};
use bldr::command::*;
use bldr::topology::Topology;

/// Our output key
static LOGKEY: &'static str = "MN";

/// The version number
#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The [docopts](http://burntsushi.net/rustdoc/docopt/index.html) usage
/// string. Determines what options are accepted.
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
static USAGE: &'static str = "
Usage: bldr install <package> -u <url> [-vn]
       bldr start <package> [-u <url>] [--group=<group>] [--topology=<topology>] [--watch=<watch>...] [--gossip-peer=<ip:port>...] [-vnI]
       bldr sh [-v -n]
       bldr bash [-v -n]
       bldr repo [-p <path>] [--port=<port>] [-vn]
       bldr upload <package> -u <url> [-vn]
       bldr generate-user-key <key> <password> <email> [--expire-days=<expire_days>] [-vn]
       bldr generate-service-key <key> [--group=<group>] [--expire-days=<expire_days>] [-vn]
       bldr encrypt --user <userkey> --service <servicekey> --infile <infile> --outfile <outfile> [--password <password>] [--group=<group>] [-vn]
       bldr decrypt --infile <infile> --outfile <outfile> [-vn]
       bldr import-key (--infile <infile> | <key> --u <url>) [-vn]
       bldr export-key (--user <userkey> | --service <servicekey>) --outfile <outfile> [--group=<group>] [-vn]
       bldr download-repo-key <key> [-vn]
       bldr upload-repo-key <key> -u <url> [-vn]
       bldr list-keys [-vn]
       bldr config <package> [-vn]

Options:
    -g, --group=<group>             The service group; shared config and topology [default: default].
    -t, --topology=<topology>       Specify a service topology [default: standalone].
    -p, --path=<path>               The path to use for a repository [default: /opt/bldr/srvc/bldr/data].
    -u, --url=<url>                 Use the specified package repository url.
    -w, --watch=<watch>             One or more service groups to watch for updates.
    -l, --gossip-listen=<ip>        The listen string for gossip [default: 0.0.0.0:9634].
    -P, --gossip-peer=<ip>          The listen string of an initial gossip peer
    -I, --gossip-permanent          If this service is a permanent gossip peer
    -v, --verbose                   Verbose output; shows line numbers.
    -e, --expire-days=<expire>      Number of days before key expires.
    -n, --no-color                  Turn ANSI color off :( .
";

/// The struct that docopts renders options
/// into.
#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_install: bool,
    cmd_start: bool,
    cmd_sh: bool,
    cmd_bash: bool,
    cmd_repo: bool,
    cmd_upload: bool,
    cmd_config: bool,
    cmd_import_key: bool,
    cmd_export_key: bool,
    cmd_upload_repo_key: bool,
    cmd_download_repo_key: bool,
    cmd_generate_user_key: bool,
    cmd_generate_service_key: bool,
    cmd_list_keys: bool,
    cmd_encrypt: bool,
    cmd_decrypt: bool,

    arg_package: Option<String>,
    arg_key: Option<String>,
    arg_password: Option<String>,
    arg_email: Option<String>,
    arg_userkey: Option<String>,
    arg_servicekey: Option<String>,
    arg_infile: Option<String>,
    arg_outfile: Option<String>,

    flag_gossip_listen: String,
    flag_gossip_peer: Vec<String>,
    flag_path: String,
    flag_port: Option<u16>,
    flag_url: Option<String>,
    flag_topology: Option<String>,
    flag_group: String,
    flag_watch: Vec<String>,
    flag_verbose: bool,
    flag_no_color: bool,
    flag_expire_days: Option<u16>,
    flag_gossip_permanent: bool,
}

/// Creates a [Config](config/struct.Config.html) from the [Args](/Args)
/// struct.
fn config_from_args(args: &Args, command: Command) -> BldrResult<Config> {
    let mut config = Config::new();
    config.set_command(command);
    if let Some(ref package) = args.arg_package {
        let (deriv, name, version, release) = try!(split_package_arg(package));
        config.set_deriv(deriv);
        config.set_package(name);
        if let Some(ver) = version {
            config.set_version(ver);
        }
        if let Some(rel) = release {
            config.set_release(rel);
        }
    }
    if let Some(ref arg_key) = args.arg_key {
        config.set_key(arg_key.clone());
    }
    if let Some(ref arg_password) = args.arg_password {
        config.set_password(arg_password.clone());
    }
    if let Some(ref arg_email) = args.arg_email {
        config.set_email(arg_email.clone());
    }
    if let Some(ref arg_userkey) = args.arg_userkey {
        config.set_user_key(arg_userkey.clone());
    }
    if let Some(ref arg_servicekey) = args.arg_servicekey {
        config.set_service_key(arg_servicekey.clone());
    }
    if let Some(ref arg_infile) = args.arg_infile {
        config.set_infile(arg_infile.clone());
    }
    if let Some(ref arg_outfile) = args.arg_outfile {
        config.set_outfile(arg_outfile.clone());
    }
    if let Some(ref topology) = args.flag_topology {
        match topology.as_ref() {
            "standalone" => {
                config.set_topology(Topology::Standalone);
            }
            "leader" => {
                config.set_topology(Topology::Leader);
            }
            "initializer" => {
                config.set_topology(Topology::Initializer);
            }
            t => return Err(bldr_error!(ErrorKind::UnknownTopology(String::from(t)))),
        }
    }
    if let Some(port) = args.flag_port {
        config.set_port(port);
    }
    if let Some(expire_days) = args.flag_expire_days {
        config.set_expire_days(expire_days);
    }
    if let Some(ref url) = args.flag_url {
        config.set_url(url.clone());
    }
    config.set_group(args.flag_group.clone());
    config.set_watch(args.flag_watch.clone());
    config.set_path(args.flag_path.clone());
    config.set_gossip_listen(args.flag_gossip_listen.clone());
    config.set_gossip_peer(args.flag_gossip_peer.clone());
    config.set_gossip_permanent(args.flag_gossip_permanent);
    if args.flag_verbose {
        bldr::output::set_verbose(true);
    }
    if args.flag_no_color {
        bldr::output::set_no_color(true);
    }
    debug!("Config:\n{:?}", config);
    Ok(config)
}

/// The primary loop for bldr.
///
/// * Set up the logger
/// * Pull in the arguments from the Command Line, push through Docopts
/// * Dispatch to a function that handles that action called
/// * Exit cleanly, or if we return an `Error`, call `exit_with(E, 1)`
#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    debug!("Docopt Args: {:?}", args);

    let result = match args {
        Args{cmd_install: true, ..} => {
            match config_from_args(&args, Command::Install) {
                Ok(config) => install(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_start: true, ..} => {
            match config_from_args(&args, Command::Start) {
                Ok(config) => start(&config),
                Err(e) => Err(e),
            }
        }

        // -----------------------------------------------------
        // start security stuff
        // -----------------------------------------------------
        Args{cmd_import_key: true, ..} => {
            match config_from_args(&args, Command::ImportKey) {
                Ok(config) => import_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_export_key: true, ..} => {
            match config_from_args(&args, Command::ExportKey) {
                Ok(config) => export_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_upload_repo_key: true, ..} => {
            match config_from_args(&args, Command::UploadRepoKey) {
                Ok(config) => upload_repo_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_download_repo_key: true, ..} => {
            match config_from_args(&args, Command::DownloadRepoKey) {
                Ok(config) => download_repo_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_generate_user_key: true, ..} => {
            match config_from_args(&args, Command::GenerateUserKey) {
                Ok(config) => generate_user_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_generate_service_key: true, ..} => {
            match config_from_args(&args, Command::GenerateServiceKey) {
                Ok(config) => generate_service_key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_list_keys: true, ..} => {
            match config_from_args(&args, Command::ListKeys) {
                Ok(config) => list_keys(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_encrypt: true, ..} => {
            match config_from_args(&args, Command::Encrypt) {
                Ok(config) => encrypt(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_decrypt: true, ..} => {
            match config_from_args(&args, Command::Decrypt) {
                Ok(config) => decrypt(&config),
                Err(e) => Err(e),
            }
        }
        // -----------------------------------------------------
        // end security stuff
        // -----------------------------------------------------
        Args{cmd_sh: true, ..} => {
            match config_from_args(&args, Command::Shell) {
                Ok(config) => shell(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_bash: true, ..} => {
            match config_from_args(&args, Command::Shell) {
                Ok(config) => shell(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_repo: true, ..} => {
            match config_from_args(&args, Command::Repo) {
                Ok(config) => repo(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_upload: true, ..} => {
            match config_from_args(&args, Command::Upload) {
                Ok(config) => upload(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_config: true, ..} => {
            match config_from_args(&args, Command::Configuration) {
                Ok(config) => configure(&config),
                Err(e) => Err(e),
            }
        }
        _ => Err(bldr_error!(ErrorKind::CommandNotImplemented)),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

/// Start a shell
#[allow(dead_code)]
fn shell(_config: &Config) -> BldrResult<()> {
    outputln!("Starting your shell; enjoy!");
    let shell_arg = try!(CString::new("sh"));
    let mut argv = [shell_arg.as_ptr(), ptr::null()];
    // Yeah, you don't know any better.. but we aren't coming back from
    // what happens next.
    unsafe {
        libc::execvp(shell_arg.as_ptr(), argv.as_mut_ptr());
    }
    Ok(())
}

/// Show the configuration options for a service
#[allow(dead_code)]
fn configure(config: &Config) -> BldrResult<()> {
    try!(configure::display(config));
    Ok(())
}

/// Install a package
#[allow(dead_code)]
fn install(config: &Config) -> BldrResult<()> {
    outputln!("Installing {}", Yellow.bold().paint(config.package_id()));
    try!(install::from_url(&config.url().as_ref().unwrap(),
                           config.deriv(),
                           config.package(),
                           config.version().clone(),
                           config.release().clone()));
    Ok(())
}

/// Start a service
#[allow(dead_code)]
fn start(config: &Config) -> BldrResult<()> {
    outputln!("Starting {}", Yellow.bold().paint(config.package_id()));
    try!(start::package(config));
    outputln!("Finished with {}", Yellow.bold().paint(config.package_id()));
    Ok(())
}

/// Run a package repo
#[allow(dead_code)]
fn repo(config: &Config) -> BldrResult<()> {
    outputln!("Starting Bldr Repository at {}",
              Yellow.bold().paint(config.path()));
    try!(repo::start(&config));
    outputln!("Finished with {}", Yellow.bold().paint(config.package_id()));
    Ok(())
}

/// Upload a package
#[allow(dead_code)]
fn upload(config: &Config) -> BldrResult<()> {
    outputln!("Upload Bldr Package {}",
              Yellow.bold().paint(config.package()));
    try!(upload::package(&config));
    outputln!("Finished with {}", Yellow.bold().paint(config.package_id()));
    Ok(())
}

/// Import a key
fn import_key(config: &Config) -> BldrResult<()> {
    outputln!("Importing key {}", Yellow.bold().paint(config.key()));
    try!(key::import(&config));
    outputln!("Finished importing key");
    Ok(())
}

/// Export a key
fn export_key(config: &Config) -> BldrResult<()> {
    outputln!("Exporting key {}", Yellow.bold().paint(config.key()));
    try!(key::export(&config));
    outputln!("Finished exporting key");
    Ok(())
}

/// Upload a key to a repo
fn upload_repo_key(config: &Config) -> BldrResult<()> {
    outputln!("Upload Bldr key {}", Yellow.bold().paint(config.key()));
    try!(key::upload(&config));
    outputln!("Finished with {}", Yellow.bold().paint(config.key()));
    Ok(())
}

/// Download a key from a repo
fn download_repo_key(_config: &Config) -> BldrResult<()> {
    panic!("Not implemented");
}

/// Generate a key for a user
fn generate_user_key(config: &Config) -> BldrResult<()> {
    outputln!("Generate user key for {}",
              Yellow.bold().paint(config.key()));
    try!(key::generate_user_key(&config));
    outputln!("Finished generating user key for {}",
              Yellow.bold().paint(config.key()));
    Ok(())
}

/// Generate a key for a service
fn generate_service_key(config: &Config) -> BldrResult<()> {
    outputln!("Generate service key for {}",
              Yellow.bold().paint(config.key()));
    try!(key::generate_service_key(&config));
    outputln!("Finished generating service key for {}",
              Yellow.bold().paint(config.key()));
    Ok(())
}

/// List bldr managed gpg keys
fn list_keys(config: &Config) -> BldrResult<()> {
    outputln!("Listing keys");
    try!(key::list(&config));
    outputln!("Finished listing keys");
    Ok(())
}

/// Encrypt a file
fn encrypt(config: &Config) -> BldrResult<()> {
    outputln!("Encrypting");
    try!(key::encrypt_and_sign(&config));
    outputln!("Finished encrypting");
    Ok(())
}

/// Decrypt a file
fn decrypt(config: &Config) -> BldrResult<()> {
    outputln!("Decrypting");
    try!(key::decrypt_and_verify(&config));
    outputln!("Finished decrypting");
    Ok(())
}



/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: BldrError, code: i32) {
    println!("{}", e);
    process::exit(code)
}

fn split_package_arg(arg: &str) -> BldrResult<(String, String, Option<String>, Option<String>)> {
    let items: Vec<&str> = arg.split("/").collect();
    match items.len() {
        2 => Ok((items[0].to_string(), items[1].to_string(), None, None)),
        3 => {
            Ok((items[0].to_string(),
                items[1].to_string(),
                Some(items[2].to_string()),
                None))
        }
        4 => {
            Ok((items[0].to_string(),
                items[1].to_string(),
                Some(items[2].to_string()),
                Some(items[3].to_string())))
        }
        _ => Err(bldr_error!(ErrorKind::InvalidPackageIdent(arg.to_string()))),
    }
}
