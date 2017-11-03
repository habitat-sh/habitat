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

use std::env;
use std::fs::File;
use std::io::Read;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{self, Command};
use std::time::Duration;
use std::thread;

use pnet_datalink as pnet;
use pnet_datalink::NetworkInterface;

use {Error, Result};
use coreutils::touch;
use mount::{self, Mount};
use namespace;
use user;
use util;

pub fn run<P: AsRef<Path>>(ns_dir: P, user: &str, interface: &str, gateway: &str) -> Result<()> {
    user::check_running_user_is_root()?;

    let netif = find_network_interface(interface)?;

    let mut command = command_as_user(util::proc_exe()?, user)?;
    command.arg("netns");
    command.arg("createasuser");
    command.arg("--ns-dir");
    command.arg(ns_dir.as_ref());
    command.arg("--interface");
    command.arg(interface);
    command.arg("--gateway");
    command.arg(gateway);
    for ip in ipv4_addrs_on(&netif)? {
        command.arg("--ipv4addr");
        command.arg(ip);
    }
    for ip in ipv6_addrs_on(&netif) {
        command.arg("--ipv6addr");
        command.arg(ip);
    }

    debug!("running as, user={}, command={:?}", user, command);
    let mut child = command.spawn()?;

    let ns_pid = wait_for_ns_pid(&ns_dir)?;
    util::run_cmd(assign_interface_cmd(interface, ns_pid)?)?;
    mount_ns_files(&ns_dir, ns_pid)?;
    touch(namespace::ns_created_file(&ns_dir))?;

    let exit_status = child.wait()?;

    println!(
        "Network namespace created: userns={}, netns={}",
        namespace::userns_file(&ns_dir).display(),
        namespace::netns_file(&ns_dir).display(),
    );

    process::exit(exit_status.code().unwrap_or(127));
}

fn wait_for_ns_pid<P: AsRef<Path>>(ns_dir: P) -> Result<u32> {
    let ns_pid = namespace::ns_pid_file(&ns_dir);
    loop {
        debug!("waiting for, namespace pid file={}", ns_pid.display());
        if ns_pid.exists() {
            debug!("found, namespace pid file={}", ns_pid.display());
            let mut buffer = String::new();
            let mut file = File::open(&ns_pid)?;
            file.read_to_string(&mut buffer)?;
            return Ok(buffer.trim().parse().expect("poop"));
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn mount_ns_files<P: AsRef<Path>>(ns_dir: P, ns_pid: u32) -> Result<()> {
    let userns_file = namespace::userns_file(&ns_dir);
    let netns_file = namespace::netns_file(&ns_dir);

    loop {
        debug!("waiting for, user namespace file={}", userns_file.display());
        if userns_file.exists() {
            debug!("found, user namespace file={}", userns_file.display());
            mount::bind(
                Path::new("/proc")
                    .join(ns_pid.to_string())
                    .join("ns")
                    .join("user"),
                &userns_file,
                Mount::Nonrecursive,
                None,
            )?;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    loop {
        debug!("waiting for, net namespace file={}", netns_file.display());
        if netns_file.exists() {
            debug!("found, net namespace file={}", netns_file.display());
            mount::bind(
                Path::new("/proc")
                    .join(ns_pid.to_string())
                    .join("ns")
                    .join("net"),
                &netns_file,
                Mount::Nonrecursive,
                None,
            )?;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn command_as_user<P: AsRef<Path>>(program: P, username: &str) -> Result<Command> {
    let mut command = Command::new(program.as_ref());
    command.uid(user::uid_for_username(username)?);
    command.gid(user::primary_gid_for_username(username)?);
    command.env_clear();
    command.env("USER", username);
    command.env("HOME", user::home_dir_for_username(username)?);
    command.env("PATH", env::var("PATH").unwrap_or(String::new()));
    for var in util::DEBUG_ENVVARS {
        if let Ok(val) = env::var(var) {
            command.env(var, val);
        }
    }

    Ok(command)
}

fn assign_interface_cmd(interface: &str, pid: u32) -> Result<Command> {
    let mut command = util::ip_cmd()?;
    command.arg("link");
    command.arg("set");
    command.arg(interface);
    command.arg("netns");
    command.arg(pid.to_string());

    Ok(command)
}

fn find_network_interface(interface: &str) -> Result<NetworkInterface> {
    match pnet::interfaces().iter().find(|i| i.name == interface) {
        Some(i) => Ok(i.clone()),
        None => Err(Error::InterfaceNotFound(String::from(interface))),
    }
}

fn ipv4_addrs_on(netif: &NetworkInterface) -> Result<Vec<String>> {
    let addrs: Vec<_> = netif
        .ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .map(|ip| ip.to_string())
        .collect();

    // We require at least one IPv4 address before going any further
    if addrs.is_empty() {
        Err(Error::IpAddressNotFound(netif.name.clone()))
    } else {
        Ok(addrs)
    }
}

fn ipv6_addrs_on(netif: &NetworkInterface) -> Vec<String> {
    let addrs: Vec<_> = netif
        .ips
        .iter()
        .filter(|ip| ip.is_ipv6())
        .map(|ip| ip.to_string())
        .collect();

    addrs
}
