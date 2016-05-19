// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

// Looking for `pkg::install`? That's in the `common` crate. You're welcome :)

pub mod path {
    use std::path::Path;

    use hcore::package::{PackageIdent, PackageInstall};

    use error::Result;

    pub fn start(ident: &PackageIdent, fs_root_path: &Path) -> Result<()> {
        let pkg_install = try!(PackageInstall::load(ident, Some(fs_root_path)));
        println!("{}", pkg_install.installed_path().display());
        Ok(())
    }
}
