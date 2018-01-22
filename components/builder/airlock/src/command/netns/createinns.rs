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

use std::path::Path;
use std::process::Command;
use std::time::Duration;
use std::thread;

use pnet_datalink as pnet;

use Result;
use namespace;
use util;

pub fn run<P: AsRef<Path>>(
    ns_dir: P,
    interface: &str,
    gateway: &str,
    ipv4s: Vec<&str>,
    ipv6s: Vec<&str>,
) -> Result<()> {
    util::check_required_packages(&[util::IP_PKG])?;
    wait_for_interface(interface);

    for ip in ipv4s {
        util::run_cmd(set_address_cmd(interface, ip)?)?;
    }
    for ip in ipv6s {
        util::run_cmd(set_address_cmd(interface, ip)?)?;
    }
    util::run_cmd(interface_up_cmd("lo")?)?;
    util::run_cmd(interface_up_cmd(interface)?)?;
    util::run_cmd(set_default_gateway_cmd(gateway)?)?;

    wait_for_created_file(&ns_dir);

    Ok(())
}

fn wait_for_interface(interface: &str) {
    loop {
        debug!("waiting for, interface={}", interface);
        if interface_present(interface) {
            debug!("found, interface={}", interface);
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn wait_for_created_file<P: AsRef<Path>>(ns_dir: P) {
    let ns_created = namespace::ns_created_file(&ns_dir);
    loop {
        debug!(
            "waiting for, namespace created file={}",
            ns_created.display()
        );
        if ns_created.exists() {
            debug!("found, namespace created file={}", ns_created.display());
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn interface_present(interface: &str) -> bool {
    pnet::interfaces()
        .iter()
        .find(|i| i.name == interface)
        .is_some()
}

fn interface_up_cmd(interface: &str) -> Result<Command> {
    let mut command = util::ip_cmd()?;
    command.arg("link");
    command.arg("set");
    command.arg("dev");
    command.arg(interface);
    command.arg("up");

    Ok(command)
}

fn set_address_cmd(interface: &str, address: &str) -> Result<Command> {
    let mut command = util::ip_cmd()?;
    command.arg("addr");
    command.arg("add");
    command.arg(address);
    command.arg("dev");
    command.arg(interface);

    Ok(command)
}

fn set_default_gateway_cmd(gateway: &str) -> Result<Command> {
    let mut command = util::ip_cmd()?;
    command.arg("route");
    command.arg("add");
    command.arg("default");
    command.arg("via");
    command.arg(gateway);

    Ok(command)
}
