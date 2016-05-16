// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

pub mod key {
    pub mod generate {
        use std::path::Path;

        use ansi_term::Colour::{Blue, Yellow};
        use hcore::crypto::BoxKeyPair;

        use error::Result;

        pub fn start(user: &str, cache: &Path) -> Result<()> {
            println!("{}",
                     Yellow.bold().paint(format!("» Generating user key for {}", &user)));
            let pair = try!(BoxKeyPair::generate_pair_for_user(user, cache));
            println!("{}",
                     Blue.paint(format!("★ Generated user key pair {}.", &pair.name_with_rev())));
            Ok(())
        }
    }
}
