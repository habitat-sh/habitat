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

use protocol::sessionsrv;
use sessionsrv::data_store::DataStore;

#[test]
fn create_account() {
    let ds = datastore_test!(DataStore);
    let mut sc = sessionsrv::SessionCreate::new();
    sc.set_token(String::from("hail2theking"));
    sc.set_extern_id(64);
    sc.set_email(String::from("bobo@chef.io"));
    sc.set_name(String::from("Bobo T. Clown"));
    sc.set_provider(sessionsrv::OAuthProvider::GitHub);

    let session = ds.find_or_create_account_via_session(&sc, true, false, false)
        .expect("Should create account");
    assert!(session.get_id() != 0, "Created account has an ID");
    assert_eq!(session.get_email(), "bobo@chef.io");
    assert_eq!(session.get_name(), "Bobo T. Clown");

    let session2 = ds.find_or_create_account_via_session(&sc, true, false, false)
        .expect("Should return account");
    assert_eq!(session.get_id(), session2.get_id());
    assert_eq!(session.get_email(), session2.get_email());
    assert_eq!(session.get_name(), session2.get_name());
}

fn create_bobo_account(ds: &DataStore) -> sessionsrv::Session {
    let mut sc = sessionsrv::SessionCreate::new();
    sc.set_token(String::from("hail2theking"));
    sc.set_extern_id(64);
    sc.set_email(String::from("bobo@chef.io"));
    sc.set_name(String::from("Bobo T. Clown"));
    sc.set_provider(sessionsrv::OAuthProvider::GitHub);
    ds.find_or_create_account_via_session(&sc, true, false, false)
        .expect("Should create account")
}

#[test]
fn get_account() {
    let ds = datastore_test!(DataStore);
    let bobo = create_bobo_account(&ds);

    let mut ag = sessionsrv::AccountGet::new();
    ag.set_name(bobo.get_name().to_string());
    let bobo2 = ds.get_account(&ag)
        .expect("Should run without error")
        .expect("Bobo should exist");

    assert_eq!(bobo.get_id(), bobo2.get_id());
    assert_eq!(bobo.get_email(), bobo2.get_email());
    assert_eq!(bobo.get_name(), bobo2.get_name());
}

#[test]
fn get_account_by_id() {
    let ds = datastore_test!(DataStore);
    let bobo = create_bobo_account(&ds);

    let mut ag = sessionsrv::AccountGetId::new();
    ag.set_id(bobo.get_id());
    let bobo2 = ds.get_account_by_id(&ag)
        .expect("Should run without error")
        .expect("Bobo should exist");

    assert_eq!(bobo.get_id(), bobo2.get_id());
    assert_eq!(bobo.get_email(), bobo2.get_email());
    assert_eq!(bobo.get_name(), bobo2.get_name());
}

#[test]
fn get_session() {
    let ds = datastore_test!(DataStore);
    let bobo = create_bobo_account(&ds);

    let mut sg = sessionsrv::SessionGet::new();
    sg.set_name(String::from(bobo.get_name()));
    sg.set_token(String::from(bobo.get_token()));

    let session = ds.get_session(&sg)
        .expect("Should run without error")
        .expect("Session should exist");
    assert_eq!(bobo, session);

    // Should expire sessions that are more than a day old
    {
        let conn = ds.pool.get(&sg).expect("get the connection back");
        conn.execute(
            "UPDATE account_sessions SET expires_at = now() - interval '2 day' WHERE token = $1",
            &[&sg.get_token()],
        ).expect("Execute successfully");
    }
    assert_eq!(None, ds.get_session(&sg).expect("Should run without error"));
}
