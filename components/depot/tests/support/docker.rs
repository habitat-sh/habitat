// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
    docker_cmd(&["run", "-d", "--cap-add=NET_ADMIN", "--expose=9632", image, "depot"])
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
            if let None = env::var_os("BLDR_DOCKER_KEEP") {
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
