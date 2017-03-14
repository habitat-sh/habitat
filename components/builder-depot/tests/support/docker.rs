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

//! Start a docker container, store the instance id
//! Get the logs
//! Stop the container
//! Get the logs
//! Remove the container on drop

use super::command;
use std::thread;
use std::env;

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

pub fn depot(image: &str) -> Docker {
    docker_cmd(&["run",
                 "-d",
                 "--cap-add=NET_ADMIN",
                 "--expose=9632",
                 image,
                 "depot"])
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
