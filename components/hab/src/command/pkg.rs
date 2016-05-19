// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

// Looking for `pkg::install`? That's in the `common` crate. You're welcome :)

pub mod exec {
    use std::env;
    use std::ffi::OsString;

    use hcore::package::{PackageIdent, PackageInstall};
    use hcore::fs::find_command;

    use error::{Error, Result};
    use exec;

    pub fn start(ident: &PackageIdent, command: &str, args: Vec<OsString>) -> Result<()> {
        let pkg_install = try!(PackageInstall::load(&ident, None));
        let env_path = try!(pkg_install.runtime_path());
        info!("Setting: PATH='{}'", &env_path);
        env::set_var("PATH", env_path);
        let command = match find_command(command) {
            Some(path) => path,
            None => return Err(Error::ExecCommandNotFound(command.to_string())),
        };
        let mut display_args = command.to_string_lossy().into_owned();
        for arg in &args {
            display_args.push(' ');
            display_args.push_str(arg.to_string_lossy().as_ref());
        }
        info!("Running: {}", display_args);
        exec::exec_command(command, args)
    }
}

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
