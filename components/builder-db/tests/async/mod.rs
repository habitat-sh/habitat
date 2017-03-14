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

use std::time::Duration;

use db::pool::Pool;
use db::test::{postgres, init, SHARD_COUNT};
use db::async;
use db::error::Result;

#[test]
fn finishes() {
    postgres::start();
    init::create_database();
    let p = Pool::new("postgresql://hab@127.0.0.1/builder_db_test",
                      5,
                      300,
                      Duration::from_secs(3600),
                      (0..SHARD_COUNT).collect())
            .expect("Failed to create pool");
    let server = async::AsyncServer::new(p);
    server.register("event1".to_string(), will_finish);
    assert!(server
                .dispatch
                .read()
                .expect("Dispatch lock poisoned")
                .contains_key("event1"));
    assert!(server
                .retry
                .read()
                .expect("Retry lock poisoned")
                .contains_key("event1"));

    server.run_event("event1".to_string(), will_finish);
    assert_eq!(server
                   .retry
                   .read()
                   .expect("Dispatch lock poisoned")
                   .contains_key("event1"),
               false);
}

#[test]
fn retries() {
    postgres::start();
    init::create_database();
    let p = Pool::new("postgresql://hab@127.0.0.1/builder_db_test",
                      5,
                      300,
                      Duration::from_secs(3600),
                      (0..SHARD_COUNT).collect())
            .expect("Failed to create pool");
    let server = async::AsyncServer::new(p);
    server.register("event1".to_string(), will_retry);
    server.run_event("event1".to_string(), will_retry);
    assert_eq!(server
                   .retry
                   .read()
                   .expect("Dispatch lock poisoned")
                   .contains_key("event1"),
               true);
    assert_eq!(server
                   .failure_count
                   .read()
                   .expect("Dispatch lock poisoned")
                   .get("event1")
                   .unwrap(),
               &1);
    server.run_event("event1".to_string(), will_retry);
    assert_eq!(server
                   .failure_count
                   .read()
                   .expect("Dispatch lock poisoned")
                   .get("event1")
                   .unwrap(),
               &2);
}

fn will_finish(pool: Pool) -> Result<async::EventOutcome> {
    Ok(async::EventOutcome::Finished)
}

fn will_retry(pool: Pool) -> Result<async::EventOutcome> {
    Ok(async::EventOutcome::Retry)
}

// #[test]
// fn creation() {
//     postgres::start();
//     init::create_database();
//     let p = Pool::new("postgresql://hab@127.0.0.1/builder_db_test",
//                       5,
//                       300,
//                       Duration::from_secs(3600),
//                       (0..SHARD_COUNT).collect())
//             .expect("Failed to create pool");
//     let mut ae = AsyncEvent::new(p);
//     ae.register(String::from("sloop"), String::from("create"), sloopy);
//     let dk = DispatchKey::new(0, String::from("sloop"), String::from("create"));
//     ae.run_event(&dk, &String::from("fatal flaw"));
//     assert!(false);
// }
//
// fn sloopy(pool: &Pool, dispatch_key: &DispatchKey, payload: &Payload) -> Result<EventOutcome> {
//     println!("Oh yes, motherfucker");
//     Ok(EventOutcome::Finished)
// }
