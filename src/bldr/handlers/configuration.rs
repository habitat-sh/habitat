//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
// use std::sync::Arc;
// use std::sync::RwLock;
// use std::thread;
// 
// use error::BldrResult;
// use pkg::Package;
// 
// pub enum ConfigEvent {
//     Start,
//     Stop,
//     Configure,
//     WriteToml(String, String)
// }
// 
// struct ConfigUpdater {
//     package: Arc<RwLock<Package>>,
//     rx: Receiver<ConfigEvent>
// }
// 
// struct ConfigHandler {
//     config_updater_thread: thread::JoinHandle<Result<(), BldrError>>,
//     tx: Sender<ConfigEvent>,
// }
// 
// impl ConfigHandler {
//     fn new(pkg: Package) {
// 
//     }
// 
//     fn spawn_config_updater(pkg: Package) -> BldrResult<()> {
//         let pkg = Arc::new
//         try!(thread::Builder::new()
//              .name(String::from("configuration"))
//              .spawn(move || -> BldrResult<()> {
// 
// 
//                 }));
//     }
// }
