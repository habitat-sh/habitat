// limitations under the License.

use std::str;
use std::path::Path;
use std::io::{self, Read};
use std::fs::File;
use std::thread;
use std::time;

use butterfly::client::Client;
use common::ui::{Status, UI};
use hcore::crypto::{SymKey, BoxKeyPair};
use hcore::service::ServiceGroup;
use toml;

use error::{Error, Result};

pub fn run(
    ui: &mut UI,
    member_id: String,
    peers: &Vec<String>,
    ring_key: Option<&SymKey>,
) -> Result<()> {
    try!(ui.begin(
        format!("Permanently marking {} as departed", member_id),
    ));

    try!(ui.status(
        Status::Creating,
        format!("service configuration"),
    ));

    for peer in peers.iter() {
        try!(ui.status(Status::Applying, format!("to peer {}", peer)));
        let mut client = try!(Client::new(peer, ring_key.map(|k| k.clone())).map_err(
            |e| {
                Error::ButterflyError(format!("{}", e))
            },
        ));
        try!(client.send_departure(member_id.clone()).map_err(|e| {
            Error::ButterflyError(format!("{}", e))
        }));

        // please take a moment to weep over the following line
        // of code. We must sleep to allow messages to be sent
        // before freeing the socket to prevent loss.
        // see https://github.com/zeromq/libzmq/issues/1264
        thread::sleep(time::Duration::from_millis(100));
    }
    try!(ui.end("Departure recorded."));
    Ok(())
}
