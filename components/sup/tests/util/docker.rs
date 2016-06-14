// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

//! Start a docker container, store the instance id
//! Get the logs
//! Stop the container
//! Get the logs
//! Remove the container on drop

use util::command;
use time;
use regex::Regex;
use std::thread;
use std::env;
use std::time::Duration;

#[derive(Debug)]
pub struct Docker {
    pub container_id: String,
}

fn docker_cmd(args: &[&str]) -> Docker {
    println!("{}: Starting docker with {:?}",
             thread::current().name().unwrap_or("main"),
             args);
    let mut cmd = command::run("docker", args).unwrap_or_else(|x| panic!("{:?}", x));
    cmd.wait_with_output();
    let mut id = String::from(cmd.stdout());
    id.pop();
    println!("{}: Docker exited with: {:?}, stdout: {}, stderr: {}",
             thread::current().name().unwrap_or("main"),
             cmd.status().code(),
             cmd.stdout(),
             cmd.stderr());
    println!("{}: Docker container: {}",
             id,
             thread::current().name().unwrap_or("main"));
    Docker { container_id: String::from(id) }
}

pub fn run(image: &str) -> Docker {
    docker_cmd(&["run", "-d", "--cap-add=NET_ADMIN", image])
}

pub fn run_with_topology(image: &str, topology: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--topology={}", topology)])
}

pub fn depot(image: &str) -> Docker {
    docker_cmd(&["run", "-d", "--cap-add=NET_ADMIN", "--expose=9632", image, "depot"])
}

pub fn run_with_env(image: &str, env: &str) -> Docker {
    docker_cmd(&["run", "-d", "--cap-add=NET_ADMIN", &format!("-e={}", env), image])
}

pub fn run_with_peer(image: &str, peer: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--peer={}", peer),
                 &format!("--group={}", thread::current().name().unwrap_or("main"))])
}

pub fn run_with_peer_permanent(image: &str, peer: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--peer={}", peer),
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--permanent-peer")])
}

pub fn run_with_permanent(image: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--permanent-peer")])
}

pub fn run_with_peer_topology(image: &str, peer: &str, topology: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--topology={}", topology),
                 &format!("--peer={}", peer),
                 &format!("--group={}", thread::current().name().unwrap_or("main"))])
}

pub fn run_with_peer_permanent_topology(image: &str, peer: &str, topology: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--topology={}", topology),
                 &format!("--peer={}", peer),
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--permanent-peer")])
}

pub fn run_with_permanent_topology(image: &str, topology: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 image,
                 "start",
                 image,
                 &format!("--topology={}", topology),
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--permanent-peer")])
}


pub fn run_with_etcd(image: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--link=bldr_etcd_1:etcd",
                 "-e=BLDR_CONFIG_ETCD=http://etcd:4001",
                 image,
                 "start",
                 "test/simple_service",
                 &format!("--group={}", thread::current().name().unwrap_or("main"))])
}

pub fn run_with_etcd_watch(image: &str, watch: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--link=bldr_etcd_1:etcd",
                 "-e=BLDR_CONFIG_ETCD=http://etcd:4001",
                 image,
                 "start",
                 "test/simple_service",
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--watch={}", watch)])
}

pub fn run_with_etcd_topology(image: &str, topology: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--link=bldr_etcd_1:etcd",
                 "-e=BLDR_CONFIG_ETCD=http://etcd:4001",
                 image,
                 "start",
                 "test/simple_service",
                 &format!("--group={}", thread::current().name().unwrap_or("main")),
                 &format!("--topology={}", topology)])
}

impl Docker {
    pub fn ipaddress(&self) -> String {
        let mut cmd = command::run("sh",
                                   &["-c",
                                     &format!("docker inspect --format='{}' {}",
                                              "{{range .NetworkSettings.Networks}}{{.\
                                               IPAddress}}{{end}}",
                                              &self.container_id)])
            .unwrap_or_else(|x| panic!("{:?}", x));
        cmd.wait_with_output();
        let ipaddress = String::from(cmd.stdout().trim());
        println!("I have ipaddress {}", &ipaddress);
        ipaddress
    }

    pub fn logs(&self) -> String {
        loop {
            // Because docker sometimes returns you a container, but the log endpoint fails.
            thread::sleep(Duration::from_millis(500));
            let mut cmd = command::run("docker", &["logs", &self.container_id])
                .unwrap_or_else(|x| panic!("{:?}", x));
            cmd.wait_with_output();
            let output = String::from(cmd.stdout());
            let error = String::from(cmd.stderr());
            let re = Regex::new(r"i/o timeout").unwrap();
            if re.is_match(&output) {
                println!("{}: An i/o error failed, retrying",
                         thread::current().name().unwrap_or("main"));
                continue;
            }
            if re.is_match(&error) {
                println!("{}: An i/o error failed, retrying",
                         thread::current().name().unwrap_or("main"));
                continue;
            }
            println!("{}: {}", thread::current().name().unwrap_or("main"), output);
            return output;
        }
    }

    pub fn wait_until(&self, ready_regex: &str) -> bool {
        let wait_duration = time::Duration::seconds(30);
        let current_time = time::now_utc().to_timespec();
        let stop_time = current_time + wait_duration;
        while time::now_utc().to_timespec() < stop_time {
            let re = Regex::new(ready_regex).unwrap();
            if re.is_match(&self.logs()) {
                return true;
            }
        }
        println!("Container not ready after 30 seconds, looking for {}",
                 ready_regex);
        false
    }

    pub fn stop(&self) {
        let mut cmd = command::run("docker", &["stop", &self.container_id])
            .unwrap_or_else(|x| panic!("{:?}", x));
        cmd.wait_with_output();
    }

    pub fn exec(&self, args: &[&str]) {
        let mut real_args = vec!["exec", &self.container_id];
        for arg in args.iter() {
            real_args.push(arg)
        }
        let mut cmd = command::run("docker", &real_args).unwrap_or_else(|x| panic!("{:?}", x));
        cmd.wait_with_output();
        if !cmd.status.unwrap().success() {
            panic!("Failed to run `docker {:#?}`: STDOUT - {:?} STDERR - {:?}",
                   real_args,
                   cmd.stdout,
                   cmd.stderr);
        }
    }
}

impl Drop for Docker {
    fn drop(&mut self) {
        if thread::panicking() {
            if let None = env::var_os("HAB_DOCKER_KEEP") {
                let mut cmd = command::run("docker", &["rm", "-f", &self.container_id])
                    .unwrap_or_else(|x| panic!("{:?}", x));
                cmd.wait_with_output();
            }
        } else {
            let mut cmd = command::run("docker", &["rm", "-f", &self.container_id])
                .unwrap_or_else(|x| panic!("{:?}", x));
            cmd.wait_with_output();
        }
    }
}
