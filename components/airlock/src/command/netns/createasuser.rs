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

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;

use unshare::{self, Namespace};

use Result;
use coreutils::{mkdir_p, touch};
use namespace;
use user;
use util;

pub fn run<P: AsRef<Path>>(
    ns_dir: P,
    interface: &str,
    gateway: &str,
    ipv4s: Vec<&str>,
    ipv6s: Vec<&str>,
) -> Result<()> {
    util::check_user_group_membership(&user::my_username()?)?;

    mkdir_p(&ns_dir)?;

    let mut command = network_unshare_command(util::proc_exe()?)?;
    command.arg("netns");
    command.arg("createinns");
    command.arg("--ns-dir");
    command.arg(ns_dir.as_ref());
    command.arg("--interface");
    command.arg(interface);
    command.arg("--gateway");
    command.arg(gateway);
    for ip in ipv4s {
        command.arg("--ipv4addr");
        command.arg(ip);
    }
    for ip in ipv6s {
        command.arg("--ipv6addr");
        command.arg(ip);
    }

    debug!("running, command={:?}", command);
    let mut child = command.spawn()?;

    write_ns_pid(&ns_dir, child.id())?;
    touch(namespace::userns_file(&ns_dir))?;
    touch(namespace::netns_file(&ns_dir))?;

    let exit_status = child.wait()?;
    process::exit(exit_status.code().unwrap_or(127));
}

fn write_ns_pid<P: AsRef<Path>>(ns_dir: P, pid: u32) -> Result<()> {
    let mut file = File::create(namespace::ns_pid_file(ns_dir))?;
    file.write_all(pid.to_string().as_bytes())?;
    Ok(())
}

fn network_unshare_command<P: AsRef<Path>>(program: P) -> Result<unshare::Command> {
    let namespaces = vec![Namespace::User, Namespace::Net];

    let mut command = unshare::Command::new(program.as_ref());
    command.uid(0);
    command.gid(0);
    command.unshare(namespaces.iter().cloned());
    command.set_id_maps(
        namespace::uid_maps(&user::my_username()?)?,
        namespace::gid_maps(&user::my_groupname()?)?,
    );
    command.set_id_map_commands(
        util::find_command("newuidmap")?,
        util::find_command("newgidmap")?,
    );

    Ok(command)
}
