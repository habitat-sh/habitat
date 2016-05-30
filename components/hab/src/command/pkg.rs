// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

// Looking for `pkg::install`? That's in the `common` crate. You're welcome :)

pub mod binlink {
    use std::fs;
    use std::path::Path;
    use std::os::unix;

    use ansi_term::Colour::{Blue, Green, Yellow};
    use hcore::package::{PackageIdent, PackageInstall};

    use error::{Error, Result};
    use exec::find_command_in_pkg;

    pub fn start(ident: &PackageIdent,
                 binary: &str,
                 dest_path: &Path,
                 fs_root_path: &Path)
                 -> Result<()> {
        let dst_path = fs_root_path.join(try!(dest_path.strip_prefix("/")));
        let dst = dst_path.join(&binary);
        println!("{}",
                 Yellow.bold().paint(format!("» Symlinking {} from {} into {}",
                                             &binary,
                                             &ident,
                                             dst_path.display())));
        let pkg_install = try!(PackageInstall::load(&ident, Some(fs_root_path)));
        let src = match try!(find_command_in_pkg(binary, &pkg_install, fs_root_path)) {
            Some(c) => c,
            None => {
                return Err(Error::CommandNotFoundInPkg((pkg_install.ident().to_string(),
                                                        binary.to_string())))
            }
        };
        if !dst_path.is_dir() {
            println!("{} parent directory {}",
                     Green.paint("Ω Creating"),
                     dst_path.display());
            try!(fs::create_dir_all(&dst_path))
        }
        match fs::read_link(&dst) {
            Ok(path) => {
                if path != src {
                    try!(fs::remove_file(&dst));
                    try!(unix::fs::symlink(&src, &dst));
                }
            }
            Err(_) => try!(unix::fs::symlink(&src, &dst)),
        }
        println!("{}",
                 Blue.paint(format!("★ Binary {} from {} symlinked to {}",
                                    &binary,
                                    &pkg_install.ident(),
                                    &dst.display())));
        Ok(())
    }
}

pub mod build {
    use std::ffi::OsString;

    use error::Result;
    use command::studio;

    pub fn start(plan_context: &str,
                 root: Option<&str>,
                 src: Option<&str>,
                 keys: Option<&str>,
                 reuse: bool)
                 -> Result<()> {
        let mut args: Vec<OsString> = Vec::new();
        if let Some(root) = root {
            args.push("-r".into());
            args.push(root.into());
        }
        if let Some(src) = src {
            args.push("-s".into());
            args.push(src.into());
        }
        if let Some(keys) = keys {
            args.push("-k".into());
            args.push(keys.into());
        }
        args.push("build".into());
        if cfg!(target_os = "linux") && reuse {
            args.push("-R".into());
        }
        args.push(plan_context.into());
        studio::start(args)
    }
}

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
