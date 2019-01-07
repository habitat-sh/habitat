// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! hab command line interface analytics module.
//!
//! The `hab` command-line tool will optionally send anonymous usage data to Habitat's Google
//! Analytics account. This is a strictly opt-in activity and no tracking will occur unless you
//! respond affirmatively to the question during `hab setup`. If you do not use `hab setup`, no
//! data will ever be sent.
//!
//! We collect this data to help improve Habitat's user experience: for example, to know what tasks
//! users are performing, and which ones they are having trouble with (e.g. mistyping command line
//! arguments).
//!
//! By _anonymous_ we mean that all identifying information about you is removed before we send the
//! data. This includes the removal of any information about what packages you are building, or
//! what origins you are using. For example, if you were building the package `yourname/yourapp`,
//! and you typed `hab pkg build -k yourkey yourname/yourapp`, the fact that you were performing
//! the `pkg build` operation would be transmitted. Neither the name of the specific package you
//! are building, nor the fact that you are using the `yourkey` key to sign that package would be
//! transmitted.
//!
//! Please do not hesitate to [contact us](mailto:support@habitat.sh) if you have questions or
//! concerns about the use of Google Analytics within the Habitat product.
//!
//! Note that this module is highly documented, even inside functions with the intent of guiding a
//! user through the implementation who may not necessarily be familiar with Rust code. Given the
//! "must-not-impact-the-user" nature of this code, it tends to be much more explicit than regular,
//! idiomatic Rust code. But at least you can see a lot of `match` expressions in action :)
//!
//! # Subcommand Invocations
//!
//! The following is a complete list of all pre-selected commands which are reported:
//!
//! * `apply`
//! * `pkg upload`
//! * `cli setup`
//! * `config apply`
//! * `file upload`
//! * `origin key generate`
//! * `pkg build`
//! * `ring key generate`
//! * `service key generate`
//! * `setup`
//! * `studio build`
//! * `studio enter`
//! * `user key generate`
//!
//! # Subcommands Which Attempt Submission of Events
//!
//! The only time events will be sent to the Google Analytics API is when certain subcommands are
//! invoked which require network access. These subcommands are:
//!
//! * `apply`
//! * `pkg upload`
//! * `cli setup`
//! * `config apply`
//! * `file upload`
//! * `install`
//! * `origin key upload`
//! * `pkg install`
//! * `setup`
//!
//! For all other subcommands, even those which report events, the event payload is saved to a
//! cached file under the analytics cache directory (`/hab/cache/analytics` for a root user and
//! `$HOME/.hab/cache/analytics` for a non-root user).
//!
//! # Event Data Breakdown
//!
//! For each event that is reported, a set of information is bundled into the payload. Here is a
//! breakdown of each key/value entry:
//!
//! ## `v=1`
//!
//! The [Protocol Version][pv] which is currently only ever the integer value of `1`.
//!
//! [pv]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#v
//!
//! ## `tid=UA-XXXXXXX-X`
//!
//! The [Tracking ID][ti]
//! which represents this product and is currently hard coded as `"UA-6369228-7"`.
//!
//! [ti]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#tid
//!
//! ## `cid=f673faaf-6ba1-4e60-b819-e2d51e4ad6f1`
//!
//! The [Client ID][ci] which is a randomly generated [UUID v4][uuid] and written into the system
//! or user's analytics cache (`/hab/cache/analytics/CLIENT_ID` when the program is invoked as the
//! root user and `$HOME/.hab/analytics/CLIENT_ID` when invoked by a non-root user). This is not
//! intended to track individual users or systems, but rather show patterns of usage in aggregate.
//! For example: "In general, users who generally start with `hab studio enter` tend to migrate to
//! using `hab pkg build` over time".
//!
//! [ci]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#cid
//! [uuid]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_.28random.29
//!
//! ## `t=event`
//!
//! The [Hit Type][ht].  This value is hard coded as `"event"` as it is a required Google Analytics
//! field for all Hit Events.
//!
//! [ht]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#t
//!
//! ## `aip=1`
//!
//! Enables [Anonymize IP][ai].  This entry ensures that the sender's IP address will not be
//! captured and will be anonymized.  The value is hard coded as the integer `1`.
//!
//! [ai]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#aip
//!
//! ## `an=hab`
//!
//! The [Application Name][an] of the program sending the event. For this program the value is
//! currently hard coded as `"hab"`.
//!
//! [an]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#an
//!
//! ## `av=0.6.0%2F20160604180457`
//!
//! The [Application Version][av] of the program sending the event. This version string will be the
//! same value as reported when asking for the program's version on the command line. Note that
//! this field may contain characters that must be percent encoded.
//!
//! [av]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#av
//!
//! ## `ds=cli--hab`
//!
//! The [Data Source][ds] which represents the program which generated the event data. For this
//! program the value is currently hardcoded as `"cli--hab"`.
//!
//! [ds]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#ds
//!
//! ## `ec=invoke`
//!
//! The [Event Category][ec] which corresponds to the type of event being sent. Currently there are
//! only 2 possible values: `"invoke"` for subcommand invocations and `"clierror"` for CLI errors.
//!
//! [ec]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#ec
//!
//! ## `ea=hab--pkg--build`
//!
//! The [Event Action][ea] which breaks down differently depending on the type of event. For
//! subcommand invocations (where `"ec=invoke"`), the value is the subcommand invoked with no
//! further arguments, options, or flags. Any spaces are replaced with a doubledash, as in:
//! `"hab--studio--enter"` or `"hab--artifact--upload"`. For CLI errors (where `"ec=clierror"`),
//! the value is the type of CLI error followed by a double dash and terminated with the subcommand
//! which was invoked (also containing no further arguments, options, or flags). As before any
//! spaces in the subcommand are replaced with a double dash, as in:
//! `"InvalidSubcommand--hab-whoops"`.
//!
//! [ea]: https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#ea
//!
//! # User-Agent HTTP Header
//!
//! A user agent string is also included in the HTTP/POST to the Google Analytics API it is of the
//! form:
//!
//! ```text
//! <PRODUCT>/<VERSION> (<TARGET>; <KERNEL_RELEASE>)
//! ```
//!
//! where:
//!
//! * `<PRODUCT>`: is the provided product name. For this program the value is currently hard coded
//!   as `"hab"`.
//! * `<VERSION>`: is the provided version string which may also include a release number. This is
//!   the same version obtained when running the help or version subcommands.
//! * `<TARGET>`: is the machine architecture and the kernel separated by a dash in lower case.
//! * `<KERNEL_RELEASE>`: is the kernel release string from `uname`.
//!
//! For example:
//!
//! ```text
//! hab/0.6.0/20160606153031 (x86_64-darwin; 14.5.0)
//! ```

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use clap;
use crate::common::ui::{Status, UIWriter, UI};
use crate::hcore;
use crate::http_client::ApiClient;
use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use uuid::Uuid;

use crate::error::Result;

/// The Google Analytics [Tracking
/// ID](https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#tid)
/// which represents this product.
const GOOGLE_ANALYTICS_ID: &'static str = "UA-6369228-7";
/// The Google Analytics [URL endpoint][g].
///
/// [g]: https://developers.google.com/analytics/devguides/collection/protocol/v1/reference#endpoint
const GOOGLE_ANALYTICS_URL: &'static str = "https://www.google-analytics.com/collect";
/// The product name for this application.
const PRODUCT: &'static str = "hab";
/// A representation of the source of the analytics data.
const DATA_SOURCE: &'static str = "cli--hab";
/// The filename containing a randomly generated [Client
/// ID](https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#cid).
const CLIENT_ID_METAFILE: &'static str = "CLIENT_ID";
/// The filename which represents a program which has opted in to analytics
const OPTED_IN_METAFILE: &'static str = "OPTED_IN";
/// The filename which represents a program which has explicitly opted out to analytics. Note that
/// the default is opted out.
const OPTED_OUT_METAFILE: &'static str = "OPTED_OUT";

/// Different kinds of analytic events.
enum Event {
    /// Occurs when a CLI error occurs, such as when a required argument is missing, an invalid
    /// subcommand is invoked, etc.
    CliError,
    /// Occurs when a pre-selected subcommand will be invoked, representing a user's intention to
    /// use this subcommand.
    Subcommand,
}

/// Selects a known pre-selected of subcommands and reports on the event of their invocation with
/// no environment, arguments, or parameters being captured.
///
/// The set of pre-selected subcommands help the project and the maintainers better confirm
/// workflow hypotheses, have feel for adoption of subsystems, and generally have a sense about
/// what is important to users in aggregate.
pub fn instrument_subcommand() {
    // If analytics are not enabled, return early, we're done.
    if !analytics_enabled() {
        return;
    }

    // Get the first 3 program arguments after the program name argument in order to determine the
    // subcommand which will be invoked. If there aren't that many program arguments, a default of
    // an empty string slice is used.
    let mut args = env::args();
    let arg1 = args.nth(1).unwrap_or_default();
    let arg2 = args.next().unwrap_or_default();
    let arg3 = args.next().unwrap_or_default();

    // Use a pattern match against a tuple of the first 3 program arguments, or as many as are
    // relevant.
    match (arg1.as_str(), arg2.as_str(), arg3.as_str()) {
        // Match against any pre-selected subcommands that are 1 level deep and ignore any
        // potential arguments, options, or flags to that subcommand--these extras will not be
        // reported.
        ("apply", _, _) => record_event(Event::Subcommand, &format!("{}--{}", PRODUCT, arg1)),
        // Match against any pre-selected subcommands that are 2 levels deep and ignore any
        // potential arguments, options, or flags to that subcommand--these extras will not be
        // reported.
        ("config", "apply", _)
        | ("file", "upload", _)
        | ("pkg", "build", _)
        | ("pkg", "upload", _)
        | ("studio", "build", _)
        | ("studio", "enter", _) => record_event(
            Event::Subcommand,
            &format!("{}--{}--{}", PRODUCT, arg1, arg2),
        ),
        // Match against any pre-selected subcommands that are 3 levels deep. Since there are no
        // more positional matches left, we ignore all further arguments, options, or flags to that
        // subcommand.
        ("origin", "key", "generate")
        | ("ring", "key", "generate")
        | ("svc", "key", "generate")
        | ("user", "key", "generate") => record_event(
            Event::Subcommand,
            &format!("{}--{}--{}--{}", PRODUCT, arg1, arg2, arg3),
        ),
        // If the subcommand to be invoked doesn't match any of the above arms, then it has not
        // been pre-selected and we can return early, all done.
        _ => (),
    };

    // If the subcommand to be invoked makes network calls, then attempt to send any pending events
    if should_send() {
        send_pending();
    }
}

/// Determines the type of CLI error and reports on the occurrence of an error given the subcommand
/// which was invoked with no environment, arguments, or parameters being captured.
///
/// The generic error types tied to the subcommand attempted help the project and the maintainers
/// to better understand common stumbling blocks, under-documented subcommands, and confirm user
/// experience hypotheses.
pub fn instrument_clap_error(err: &clap::Error) {
    // If analytics are not enabled, return early, we're done.
    if !analytics_enabled() {
        return;
    }

    // Get the first 3 program arguments after the program name in order to determine the
    // subcommand which was invoked leading to the CLI error. If there aren't that many program
    // arguments, a default of an empty string slice is used.
    let mut args = env::args();
    let arg1 = args.nth(1).unwrap_or_default();
    let arg2 = args.next().unwrap_or_default();
    let arg3 = args.next().unwrap_or_default();

    // Use a pattern match against the first program argument.
    match arg1.as_str() {
        // Match against subcommands which are 2 levels deep.
        "config" | "file" | "pkg" => record_event(
            Event::CliError,
            &format!("{:?}--{}--{}--{}", err.kind, PRODUCT, arg1, arg2),
        ),
        // Match against subcommands which are 3 levels deep.
        "origin" | "ring" | "svc" | "user" => record_event(
            Event::CliError,
            &format!("{:?}--{}--{}--{}--{}", err.kind, PRODUCT, arg1, arg2, arg3),
        ),
        // Match against subcommands which are 1 levels deep or anything else remaining. This match
        // arm appears last because it "slurps" up all remaining cases with `_`.
        "apply" | "install" | "setup" | _ => record_event(
            Event::CliError,
            &format!("{:?}--{}--{}", err.kind, PRODUCT, arg1),
        ),
    }
}

/// Explicitly opts in to reporting analytics.
///
/// This function is designed to be triggered from another subcommand and can be a blocking call.
/// That is to say that this doesn't have to work on a separate thread of execution. As a result,
/// we can use more Rust idioms such as `try!` and `Result` to make for a much more terse and
/// direct function body.
///
/// # Errors
///
/// * If the parent directory cannot be created
/// * If an opt-out if exists but cannot be deleted
/// * If an opt-in file cannot be created
pub fn opt_in(ui: &mut UI, analytics_path: &Path, origin_generated: bool) -> Result<()> {
    ui.begin("Opting in to analytics")?;
    // Create the parent directory which will contain the opt-in file
    fs::create_dir_all(analytics_path)?;
    // Get the path to the opt-out file
    let opt_out_path = analytics_path.join(OPTED_OUT_METAFILE);
    // If the opt-out file exists, delete it from disk
    if opt_out_path.exists() {
        ui.status(Status::Deleting, opt_out_path.display())?;
        fs::remove_file(&opt_out_path)?;
    }
    // Get the path to the opt-in file
    let opt_in_path = analytics_path.join(OPTED_IN_METAFILE);
    ui.status(Status::Creating, opt_in_path.display())?;
    // Create the opt-in file
    let _ = File::create(opt_in_path)?;
    ui.end("Analytics opted in, thank you!")?;
    // Record an event that the setup subcommand was invoked
    record_event(
        Event::Subcommand,
        &format!("{}--{}--{}", PRODUCT, "cli", "setup"),
    );
    // If an origin key was generated in the setup subcommand, record an event as well
    if origin_generated {
        record_event(
            Event::Subcommand,
            &format!("{}--{}--{}--{}", PRODUCT, "origin", "key", "generate"),
        );
    }
    // Send any pending events to the Google Analytics API
    send_pending();
    // Return an empty Ok, representing a successful operation
    Ok(())
}

/// Explicitly opts out of reporting analytics.
///
/// This function is designed to be triggered from another subcommand and can be a blocking call.
/// That is to say that this doesn't have to work on a separate thread of execution. As a result,
/// we can use more Rust idioms such as `try!` and `Result` to make for a much more terse and
/// direct function body.
///
/// # Errors
///
/// * If the parent directory cannot be created
/// * If an opt-in if exists but cannot be deleted
/// * If an opt-out file cannot be created
pub fn opt_out(ui: &mut UI, analytics_path: &Path) -> Result<()> {
    ui.begin("Opting out of analytics")?;
    // Create the parent directory which will contain the opt-in file
    fs::create_dir_all(analytics_path)?;
    // Get the path to the opt-in file
    let opt_in_path = analytics_path.join(OPTED_IN_METAFILE);
    // If the opt-in file exists, delete it from disk
    if opt_in_path.exists() {
        ui.status(Status::Deleting, opt_in_path.display())?;
        fs::remove_file(&opt_in_path)?;
    }
    // Get the path to the opt-out file
    let opt_out_path = analytics_path.join(OPTED_OUT_METAFILE);
    ui.status(Status::Creating, opt_out_path.display())?;
    // Create the opt-out file
    let _ = File::create(opt_out_path)?;
    ui.end("Analytics opted out, we salute you just the same!")?;
    // Return an empty Ok, representing a successful operation
    Ok(())
}

/// Returns whether or not analytics are explicitly opted in, opted out, or are so far unset.
pub fn is_opted_in<T>(analytics_path: T) -> Option<bool>
where
    T: AsRef<Path>,
{
    if analytics_path.as_ref().join(OPTED_OUT_METAFILE).exists() {
        // If an explicit opt-out file exists, the return false
        Some(false)
    } else if analytics_path.as_ref().join(OPTED_IN_METAFILE).exists() {
        // If an opt-in file exists, return true
        Some(true)
    } else {
        // In all other cases, return a None as there is no explicit yes or no answer
        None
    }
}

/// Returns true if analytics are enabled and false otherwise.
fn analytics_enabled() -> bool {
    match is_opted_in(hcore::fs::cache_analytics_path(None::<String>)) {
        // If the value is explicitly true or false, return the unwrapped value
        Some(val) => val,
        // In all other cases, return false which enforces the default opt-out behavior
        None => false,
    }
}

/// Creates a representation of the analytic event and sends it to the Google Analytics API.
fn record_event(kind: Event, action: &str) {
    // Determine a suitable category value given the type of event we are capturing. We are using
    // pattern matching over a type which means that the Rust compiler will guarantee that no enum
    // types are missing in the future.
    let category = match kind {
        Event::CliError => "clierror",
        Event::Subcommand => "invoke",
    };
    // Craft the Google Analytics payload body which resembles a URL query string, even requiring
    // all values to be percent encoded. For more details about the payload data format see:
    // https://developers.google.com/analytics/devguides/collection/protocol/v1/reference#payload
    //
    // For more details about the chosen variables, values, and their meanings, see the above
    // module documentation.
    let event = format!(
        "v=1&tid={}&cid={}&t=event&aip=1&an={}&av={}&ds={}&ec={}&ea={}",
        utf8_percent_encode(GOOGLE_ANALYTICS_ID, PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(&client_id(), PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(PRODUCT, PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(super::VERSION, PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(DATA_SOURCE, PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(category, PATH_SEGMENT_ENCODE_SET),
        utf8_percent_encode(action, PATH_SEGMENT_ENCODE_SET)
    );
    debug!("Event: {}", event);
    // Save the event to disk--there might not be enough time to hit the network
    save_event(&event);
}

/// Determines whether or not to send an event now or save it for later.
///
/// If the subcommand to be invoked requires network access then this is a reasonable time to
/// attempt submitting events, otherwise we should honor the spirit of the subcommand and not hit
/// the network if it is an "offline" operation.
fn should_send() -> bool {
    let mut args = env::args();

    // Use a pattern match against a tuple of the first 3 program arguments after the program name
    // in order to determine whether or not the subcommand to be invoked is going to hit the
    // network. If it will, return true and otherwise return false.
    match (
        args.nth(1).unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
    ) {
        ("apply", _, _)
        | ("config", "apply", _)
        | ("file", "upload", _)
        | ("install", _, _)
        | ("origin", "key", "upload")
        | ("pkg", "install", _)
        | ("pkg", "upload", _) => true,
        _ => false,
    }
}

/// Sends the event to Google Analytics via an HTTP/POST.
///
/// This function returns true if the event was sent and false if an error occurred along the way.
/// The presence of a `false` return value is enough to assume that we want to save this event to
/// disk for later retry.
fn send_event(payload: &str) -> bool {
    // Create a new Hyper HTTP client, using a helper from the `habitat_http_client` crate. This
    // function is responsible for setting up the SSL context, finding suitable SSL root certificate
    // files, etc. The `None` reference is for a more advanced use case which is the Rust way of
    // saying: "I'm giving you nothing for this value, as opposed to something".
    //
    // The `ApiClient::new` function returns a `Result` structure which can either be `Ok`, or
    // can contain an error (`Err`). The `Ok` pattern matching arm will return the actual
    // "unwrapped" client from the expression and setting the client variable binding. The `Err`
    // matching arm is when something (or anything) goes wrong. In this case we absolutely do not
    // want to crash, panic this thread, or otherwise impact the real operation potentially running
    // concurrently. So, the strategy here is to report and early return.
    let client = match ApiClient::new(GOOGLE_ANALYTICS_URL, PRODUCT, super::VERSION, None) {
        Ok(c) => c,
        Err(e) => {
            debug!("Unable to create HTTP client for analytics: {}", e);
            return false;
        }
    };
    // Build up an HTTP/POST request to the Google Analytics API endpoint with the event payload as
    // the body of the request.
    let request = client.post("").body(payload);
    // Send the request on the wire. As before we will unwrap the successful operation or report
    // and early return if anything goes wrong.
    let response = match request.send() {
        Ok(r) => r,
        Err(e) => {
            debug!("Error posting payload to {}: {}", GOOGLE_ANALYTICS_URL, e);
            return false;
        }
    };
    // Report if the posting was successful or not successful.
    if response.status.is_success() {
        debug!(
            "Event posted successfully: {}",
            response.status.canonical_reason().unwrap_or_default()
        );
        true
    } else {
        debug!(
            "Response indicated not successful: {}",
            response.status.canonical_reason().unwrap_or_default()
        );
        false
    }
}

/// Save the event to a timestamped file under the analytics cache directory.
///
/// A later execution of this program may batch up pending events and attempt to submit them.
fn save_event(payload: &str) {
    // Compute a timestamp as number of seconds and nanoseconds since the Unix epoch.
    let (secs, subsec_nanos) = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => (duration.as_secs(), duration.subsec_nanos()),
        Err(e) => {
            debug!("Cannot generate system time: {}", e);
            return;
        }
    };
    // Determine the parent directory for the cached event file.
    let cache_dir = hcore::fs::cache_analytics_path(None::<String>);
    // Determine the full path to the cached event file.
    let cached_event = cache_dir.join(format!("event-{}.{}.txt", secs, subsec_nanos));
    // Write the file with the payload contents to disk.
    write_file(&cache_dir, &cached_event, payload);
}

/// Attempts to send any pending events on disk in the analytics cache.
fn send_pending() {
    // Determine the path to the analytics cache directory.
    let cache_dir = hcore::fs::cache_analytics_path(None::<String>);
    // Get an iterator to all file and directory entries under the cache directory. If an error
    // occurs, report and return early.
    let entries = match cache_dir.read_dir() {
        Ok(rd) => rd,
        Err(e) => {
            debug!(
                "Cannot read directory entries in {}: {}",
                cache_dir.display(),
                e
            );
            return;
        }
    };
    // Iterate over each directory entry.
    for entry in entries {
        // Ensure that the directory entry is readable and if not, proceed to the next entry.
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                debug!("Error reading directory entry: {}", e);
                continue;
            }
        };
        // Get the entry's metadata and if there are any errors, proceed to the next entry.
        let metadata = match entry.metadata() {
            Ok(md) => md,
            Err(e) => {
                debug!("Error reading entry metadata: {}", e);
                continue;
            }
        };
        // If the directory entry is a file and the base file name starts with `event-`, then this
        // is a cached event. Otherwise proceed to the next entry.
        if metadata.is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .as_ref()
                .starts_with("event-")
        {
            let file_path = entry.path();
            // Send the event, but if not successful report and proceed to the next entry.
            if send_event(&read_file(&file_path)) {
                // If the event was successfully sent, then remove the cached file. If there is an
                // error removing the file, report and proceed to the next entry.
                if let Err(e) = fs::remove_file(&file_path) {
                    debug!("Error removing {}: {}", file_path.display(), e);
                    continue;
                }
            } else {
                debug!("Error sending pending event, keeping for a retry next attempt");
                continue;
            }
        }
    }
}

/// Returns a previous randomly generated [Client
/// ID](https://developers.google.com/analytics/devguides/collection/protocol/v1/parameters#cid) or
/// creates, saves, and return a new Client ID.
fn client_id() -> String {
    // Get a path to the location containing a file with the randomly generated Client ID, using a
    // helper function from the `habitat_core` crate. The `None` tells the function that there is
    // no custom file system root prefix path.
    let metadir = hcore::fs::cache_analytics_path(None::<String>);
    // Get the path to the metadata file.
    let metafile = metadir.join(CLIENT_ID_METAFILE);
    if metafile.exists() {
        // Return the contents of the file which is the Client ID
        read_file(&metafile)
    } else {
        // Generate a new, random UUID for the Client ID.
        let uuid = Uuid::new_v4().to_hyphenated_ref().to_string();
        write_file(&metadir, &metafile, &uuid);
        // Finally, return the Client ID String
        uuid
    }
}

/// Reads a file from disk and returns its contents as an owned String.
fn read_file(file_path: &Path) -> String {
    // Set up a mutable heap-allocated buffer to read into from the file.
    let mut content = String::new();
    // If the file exists, then open it to get a file handle for reading. As before, this could
    // fail so we will unwrap and use the success and report and early return on any failure.
    // As this function is guaranteeing that a String will be returned, and given that we can't
    // open the file with our answer (we hope), then we'll return an empty String.
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            debug!("Error opening file {}: {}", file_path.display(), e);
            return content;
        }
    };
    // Read the file contents into the above mutable String buffer. This "if let" construct
    // will only report and early return if a failure is returned in the `Result`.
    if let Err(e) = file.read_to_string(&mut content) {
        debug!("Error reading file {}: {}", file_path.display(), e);
        return content;
    }
    // Finally, return the content as a String.
    content
}

/// Writes the content to a file while also ensuring that the parent directory exists.
fn write_file(parent_dir: &Path, file_path: &Path, content: &str) {
    debug!("Creating directory {}", parent_dir.display());
    // Create the parent directory of the file to ensure that it exists. If an error
    // is returned during directory creation, we'll report the error and return.
    if let Err(e) = fs::create_dir_all(parent_dir) {
        debug!("Error creating directory {}: {}", parent_dir.display(), e);
        return;
    };
    // Create an empty file and return a file handle for writing. Same error handling logic as
    // above.
    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            debug!("Error creating file {}: {}", file_path.display(), e);
            return;
        }
    };
    debug!("Creating file {}", file_path.display());
    // Write out the content to the file. Same error handling logic as above.
    if let Err(e) = file.write_all(content.as_bytes()) {
        debug!("Error writing to file {}: {}", file_path.display(), e);
        return;
    }
}
