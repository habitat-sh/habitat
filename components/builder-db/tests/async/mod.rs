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

use db::async;
use db::error::Result;
use db::pool::Pool;

#[test]
fn finishes() {
    with_pool!(pool, {
        let server = async::AsyncServer::new(pool);
        server.register("event1".to_string(), will_finish);
        assert!(
            server
                .dispatch
                .read()
                .expect("Dispatch lock poisoned")
                .contains_key("event1")
        );
        assert!(
            server
                .retry
                .read()
                .expect("Retry lock poisoned")
                .contains_key("event1")
        );
        server.run_event("event1".to_string(), will_finish);
        assert_eq!(
            server
                .retry
                .read()
                .expect("Dispatch lock poisoned")
                .contains_key("event1"),
            false
        );
    });
}

#[test]
fn retries() {
    with_pool!(pool, {
        let server = async::AsyncServer::new(pool);
        server.register("event1".to_string(), will_retry);
        server.run_event("event1".to_string(), will_retry);
        assert_eq!(
            server
                .retry
                .read()
                .expect("Dispatch lock poisoned")
                .contains_key("event1"),
            true
        );
        assert_eq!(
            server
                .failure_count
                .read()
                .expect("Dispatch lock poisoned")
                .get("event1")
                .unwrap(),
            &1
        );
        server.run_event("event1".to_string(), will_retry);
        assert_eq!(
            server
                .failure_count
                .read()
                .expect("Dispatch lock poisoned")
                .get("event1")
                .unwrap(),
            &2
        );
    });
}

fn will_finish(_pool: Pool) -> Result<async::EventOutcome> {
    Ok(async::EventOutcome::Finished)
}

fn will_retry(_pool: Pool) -> Result<async::EventOutcome> {
    Ok(async::EventOutcome::Retry)
}
