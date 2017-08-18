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

use protobuf;
use protocol::originsrv;
use originsrv::data_store::DataStore;

use std::str::FromStr;

#[test]
fn create_origin_poop() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    // Create new database connection
    let conn = ds.pool.get(&origin).expect(
        "Cannot get connection from pool",
    );

    let rows = conn.query("SELECT COUNT(*) FROM origin_channels", &[])
        .expect("Failed to query database for number of channels");
    let count: i64 = rows.iter().nth(0).unwrap().get(0);
    assert_eq!(count, 2); // note: count of 2 is the 'unstable' and 'stable' channels
}

#[test]
fn create_origin_handles_unique_constraint_violations_correctly() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let mut origin2 = originsrv::OriginCreate::new();
    origin2.set_name(String::from("neurosis"));
    origin2.set_owner_id(1);
    origin2.set_owner_name(String::from("scottkelly"));
    let resp2 = ds.create_origin(&origin2);

    assert!(resp2.is_err(), "Insertion should've triggered an error");
}

#[test]
fn get_origin_by_name() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let new_origin = ds.get_origin_by_name("neurosis").expect(
        "Could not get the origin",
    );
    assert!(new_origin.is_some(), "Origin did not exist");
    let fg = new_origin.unwrap();
    assert_eq!(fg.get_name(), "neurosis");
    assert_eq!(fg.get_owner_id(), 1);
    assert_eq!(fg.get_private_key_name(), "");
}

#[test]
fn create_origin_secret_key() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin secret key
    let mut oskc = originsrv::OriginSecretKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_secret").into_bytes());
    ds.create_origin_secret_key(&oskc).expect(
        "Failed to create origin secret key",
    );

    // Origin secret keys get returned with the origin
    let first = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(first.get_private_key_name(), "neurosis-20160612031944");

    // They are also sorted based on the latest key if there is more than one
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect(
        "Failed to create origin secret key",
    );
    let second = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(second.get_private_key_name(), "neurosis-20160612031945");
}

#[test]
fn create_origin_secret_key_handles_unique_constraint_violations_correctly() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin secret key
    let mut oskc = originsrv::OriginSecretKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_secret").into_bytes());
    ds.create_origin_secret_key(&oskc).expect(
        "Failed to create origin secret key",
    );

    // Create a duplicate origin secret key, which should fail
    let mut oskc2 = originsrv::OriginSecretKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_revision(String::from("20160612031944"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_body(String::from("very_secret").into_bytes());
    let resp = ds.create_origin_secret_key(&oskc2);

    assert!(resp.is_err(), "Insertion should've triggered an error");
}

#[test]
fn get_origin_secret_key() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin secret key
    let mut oskc = originsrv::OriginSecretKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_secret").into_bytes());
    ds.create_origin_secret_key(&oskc).expect(
        "Failed to create origin secret key",
    );
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect(
        "Failed to create origin secret key",
    );

    let mut osk_get = originsrv::OriginSecretKeyGet::new();
    osk_get.set_origin(String::from("neurosis"));
    osk_get.set_owner_id(1);
    let neurosis_key = ds.get_origin_secret_key(&osk_get)
        .expect("Failed to get origin secret key from database")
        .expect("No origin secret key found in database");
    assert_eq!(neurosis_key.get_name(), "neurosis");
    assert_eq!(neurosis_key.get_revision(), "20160612031945");
    assert_eq!(neurosis_key.get_origin_id(), neurosis.get_id());
    assert_eq!(neurosis_key.get_body(), oskc.get_body());
    assert_eq!(neurosis_key.get_owner_id(), oskc.get_owner_id());
}

#[test]
fn create_origin_public_key() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin public key
    let mut oskc = originsrv::OriginPublicKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_public").into_bytes());
    ds.create_origin_public_key(&oskc).expect(
        "Failed to create origin public key",
    );
    let mut oskc2 = originsrv::OriginPublicKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_revision(String::from("20160612031945"));
    oskc2.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc2).expect(
        "Failed to create origin public key",
    );

    let mut oskg = originsrv::OriginPublicKeyGet::new();
    oskg.set_owner_id(1);
    oskg.set_origin(oskc.get_name().to_string());
    oskg.set_revision(String::from("20160612031944"));
    let key = ds.get_origin_public_key(&oskg)
        .expect("Could not get the key")
        .expect("key did not exist");
    assert_eq!(key.get_body(), oskc.get_body());
}

#[test]
fn create_origin_public_key_handles_unique_constraint_violations_correctly() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin public key
    let mut oskc = originsrv::OriginPublicKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_public").into_bytes());
    ds.create_origin_public_key(&oskc).expect(
        "Failed to create origin public key",
    );
    let mut oskc2 = originsrv::OriginPublicKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_revision(String::from("20160612031945"));
    oskc2.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc2).expect(
        "Failed to create origin public key",
    );

    // Create a duplicate origin public key, which should fail
    let mut opkc = originsrv::OriginPublicKeyCreate::new();
    opkc.set_name(String::from("neurosis"));
    opkc.set_revision(String::from("20160612031944"));
    opkc.set_origin_id(neurosis.get_id());
    opkc.set_owner_id(1);
    opkc.set_body(String::from("very_public").into_bytes());
    let resp = ds.create_origin_public_key(&opkc);

    assert!(resp.is_err(), "Insertion should've triggered an error");
}

#[test]
fn get_origin_public_key_latest() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin public key
    let mut oskc = originsrv::OriginPublicKeyCreate::new();
    let body = String::from("very_public").into_bytes();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(body);
    ds.create_origin_public_key(&oskc).expect(
        "Failed to create origin public key",
    );
    oskc.set_revision(String::from("20160612031945"));
    oskc.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc).expect(
        "Failed to create origin public key",
    );

    let mut osk_get = originsrv::OriginPublicKeyLatestGet::new();
    osk_get.set_origin(String::from("neurosis"));
    osk_get.set_owner_id(1);
    let neurosis_key = ds.get_origin_public_key_latest(&osk_get)
        .expect("Failed to get origin public key from database")
        .expect("No origin public key found in database");
    assert_eq!(neurosis_key.get_name(), "neurosis");
    assert_eq!(neurosis_key.get_revision(), "20160612031945");
    assert_eq!(neurosis_key.get_origin_id(), neurosis.get_id());
    assert_eq!(neurosis_key.get_body(), oskc.get_body());
    assert_eq!(neurosis_key.get_owner_id(), oskc.get_owner_id());
}

#[test]
fn list_origin_public_key() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin public key
    let mut oskc = originsrv::OriginPublicKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_public").into_bytes());
    ds.create_origin_public_key(&oskc).expect(
        "Failed to create origin public key",
    );
    let mut oskc2 = originsrv::OriginPublicKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_revision(String::from("20160612031945"));
    oskc2.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc2).expect(
        "Failed to create origin public key",
    );

    let mut oskl = originsrv::OriginPublicKeyListRequest::new();
    oskl.set_origin_id(neurosis.get_id());
    let keys = ds.list_origin_public_keys_for_origin(&oskl).expect(
        "Could not get the keys from the database",
    );
    assert_eq!(keys.get_keys().len(), 2);
    let key1 = keys.get_keys().iter().nth(0).unwrap();
    assert_eq!(key1.get_revision(), "20160612031945");
    let key2 = keys.get_keys().iter().nth(1).unwrap();
    assert_eq!(key2.get_revision(), "20160612031944");
}

#[test]
fn create_origin_invitation() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut oic = originsrv::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(2);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation",
    );
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation again, which should be a no-op",
    );

    oic.set_owner_id(5);
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation again, which should be a no-op",
    );

    // We should never create an invitation for the same person and org
    let conn = ds.pool.get(&oic).expect("Cannot get connection from pool");
    let rows = conn.query("SELECT COUNT(*) FROM origin_invitations", &[])
        .expect("Failed to query database for number of invitations");
    let count: i64 = rows.iter().nth(0).unwrap().get(0);
    assert_eq!(count, 1);
}

#[test]
fn list_origin_invitations_for_origin() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut oilr = originsrv::OriginInvitationListRequest::new();
    oilr.set_origin_id(neurosis.get_id());
    let no_invites = ds.list_origin_invitations_for_origin(&oilr).expect(
        "Failed to get origin list from database",
    );
    assert_eq!(
        no_invites.get_invitations().len(),
        0,
        "We have invitations when we should have none"
    );

    let mut oic = originsrv::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(2);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation",
    );
    oic.set_account_id(3);
    oic.set_account_name(String::from("maynard_james_keenan"));
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation",
    );
    oic.set_account_id(4);
    oic.set_account_name(String::from("danny_cary"));
    ds.create_origin_invitation(&oic).expect(
        "Failed to create the origin invitation",
    );

    // List comes back in alphabetical order by origin
    let oi_list = ds.list_origin_invitations_for_origin(&oilr).expect(
        "Could not get origin invitation list from database",
    );
    assert_eq!(oi_list.get_invitations().len(), 3);
    let danny = oi_list.get_invitations().iter().nth(0).unwrap();
    assert_eq!(danny.get_account_id(), 4);
    let maynard = oi_list.get_invitations().iter().nth(1).unwrap();
    assert_eq!(maynard.get_account_id(), 3);
    let noel = oi_list.get_invitations().iter().nth(2).unwrap();
    assert_eq!(noel.get_account_id(), 2);
}

#[test]
fn check_account_in_origin() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut coar = originsrv::CheckOriginAccessRequest::new();
    coar.set_origin_name(String::from("neurosis"));
    coar.set_account_id(1);

    assert!(
        ds.check_account_in_origin(&coar).expect(
            "failed to check membership in the database",
        ),
        "Member should have been in the origin"
    );
}

#[test]
fn create_origin_project() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let neurosis = ds.create_origin(&origin)
        .expect("Should create origin")
        .expect("Should return the origin");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from(neurosis.get_name()));
    op.set_origin_id(neurosis.get_id());
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    ds.create_origin_project(&opc).expect(
        "Failed to create origin project",
    );
}

#[test]
fn create_origin_project_handles_unique_constraint_violations_correctly() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let neurosis = ds.create_origin(&origin)
        .expect("Should create origin")
        .expect("Should return the origin");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from(neurosis.get_name()));
    op.set_origin_id(neurosis.get_id());
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op.clone());

    ds.create_origin_project(&opc).expect(
        "Failed to create origin project",
    );

    // Now let's insert a duplicate, which should throw an error
    let mut opc2 = originsrv::OriginProjectCreate::new();
    opc2.set_project(op);

    let resp = ds.create_origin_project(&opc2);
    assert!(resp.is_err(), "Insertion should've triggered an error");
}

#[test]
fn get_origin_project_by_name() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let neurosis = ds.create_origin(&origin)
        .expect("Should create origin")
        .expect("Should return the origin");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from(neurosis.get_name()));
    op.set_origin_id(neurosis.get_id());
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    let _ = ds.create_origin_project(&opc);

    let project = ds.get_origin_project_by_name("neurosis/zeal")
        .expect("Error getting project form database")
        .expect("Project does not exist");
    assert!(project.get_id() != 0, "Should have a real ID");
    assert_eq!(
        project.get_origin_id(),
        neurosis.get_id(),
        "Should have the right origin id"
    );
    assert_eq!(
        project.get_origin_name(),
        "neurosis",
        "Should have the right origin name"
    );
    assert_eq!(
        project.get_package_name(),
        "zeal",
        "Should have zeal as the package name"
    );
    assert_eq!(
        project.get_name(),
        "neurosis/zeal",
        "Should have neurosis/zeal as the project name"
    );
    assert_eq!(
        project.get_plan_path(),
        "foo",
        "Should have foo as the plan path"
    );
    assert_eq!(project.get_owner_id(), 1, "Should have the right owner id");
    assert_eq!(
        project.get_vcs_type(),
        "git",
        "Should have the right vcs type"
    );
    assert_eq!(
        project.get_vcs_data(),
        "git://github.com/habitat-sh/core-plans",
        "Should have the right vcs data"
    );
}

#[test]
fn delete_origin_project_by_name() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_id(neurosis.get_id());
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    ds.create_origin_project(&opc).expect(
        "Failed to create project in database",
    );
    assert!(
        ds.delete_origin_project_by_name("neurosis/zeal").is_ok(),
        "Should delete existing project"
    );

    assert!(
        ds.get_origin_project_by_name("neurosis/zeal")
            .expect("Error getting project form database")
            .is_none(),
        "Project should not exist"
    );
}

#[test]
fn update_origin_project() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_id(neurosis.get_id());
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op.clone());

    ds.create_origin_project(&opc).expect(
        "Failed to create project in database",
    );

    let mut project = ds.get_origin_project_by_name("neurosis/zeal")
        .expect("Error getting project form database")
        .expect("Project does not exist");

    project.set_package_name(String::from("sepultura"));
    // This is
    project.set_name(String::from("no/nope"));
    project.set_plan_path(String::from("bar"));
    project.set_vcs_type(String::from("svn"));
    project.set_vcs_data(String::from("svn://github.com/habitat-sh/core-plans"));
    project.set_owner_id(2);

    let mut opu = originsrv::OriginProjectUpdate::new();
    let updated_project = project.clone();
    opu.set_project(project);

    ds.update_origin_project(&opu).expect(
        "Failed to update project in database",
    );

    let sepultura = ds.get_origin_project_by_name("neurosis/sepultura")
        .expect("Error getting project form database")
        .expect("Project does not exist");
    assert_eq!(
        updated_project.get_id(),
        sepultura.get_id(),
        "Should have a the same ID"
    );
    assert_eq!(
        updated_project.get_origin_id(),
        sepultura.get_origin_id(),
        "Should have the same origin id"
    );
    assert_eq!(
        updated_project.get_origin_name(),
        sepultura.get_origin_name(),
        "Should have the same origin name"
    );
    assert_eq!(
        sepultura.get_package_name(),
        "sepultura",
        "Should have the correct package name"
    );
    assert_eq!(
        sepultura.get_name(),
        "neurosis/sepultura",
        "Should have neurosis/sepultura as the project name"
    );
    assert_eq!(
        updated_project.get_plan_path(),
        sepultura.get_plan_path(),
        "Should have the same plan path"
    );
    assert_eq!(
        updated_project.get_owner_id(),
        sepultura.get_owner_id(),
        "Should have the same owner id"
    );
    assert_eq!(
        updated_project.get_vcs_type(),
        sepultura.get_vcs_type(),
        "Should have the updated vcs type"
    );
    assert_eq!(
        updated_project.get_vcs_data(),
        sepultura.get_vcs_data(),
        "Should have the same vcs data"
    );
}

#[test]
fn create_origin_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident = originsrv::OriginPackageIdent::new();
    ident.set_origin("core".to_string());
    ident.set_name("cacerts".to_string());
    ident.set_version("2017.01.17".to_string());
    ident.set_release("20170209064044".to_string());

    let mut dep_idents = protobuf::RepeatedField::new();
    let mut dep_ident = originsrv::OriginPackageIdent::new();
    dep_ident.set_origin("dep_org".to_string());
    dep_ident.set_name("dep_name".to_string());
    dep_ident.set_version("1.1.1-dep".to_string());
    dep_ident.set_release("20170101010102".to_string());
    dep_idents.push(dep_ident);
    let mut dep_ident2 = originsrv::OriginPackageIdent::new();
    dep_ident2.set_origin("dep_org2".to_string());
    dep_ident2.set_name("dep_name2".to_string());
    dep_ident2.set_version("1.1.1-dep2".to_string());
    dep_ident2.set_release("20170101010122".to_string());
    dep_idents.push(dep_ident2);

    let mut tdep_idents = protobuf::RepeatedField::new();
    let mut tdep_ident = originsrv::OriginPackageIdent::new();
    tdep_ident.set_origin("tdep_org".to_string());
    tdep_ident.set_name("tdep_name".to_string());
    tdep_ident.set_version("1.1.1-tdep".to_string());
    tdep_ident.set_release("20170101010103".to_string());
    tdep_idents.push(tdep_ident);
    let mut tdep_ident2 = originsrv::OriginPackageIdent::new();
    tdep_ident2.set_origin("tdep_org2".to_string());
    tdep_ident2.set_name("tdep_name2".to_string());
    tdep_ident2.set_version("1.1.1-tdep2".to_string());
    tdep_ident2.set_release("20170101010123".to_string());
    tdep_idents.push(tdep_ident2);

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident);
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_deps(dep_idents);
    package.set_tdeps(tdep_idents);
    package.set_config("config".to_string());
    package.set_target("x86_64-linux".to_string());
    package.set_exposes(vec![1, 2]);

    ds.create_origin_package(&package).expect(
        "Failed to create origin package",
    );
}

#[test]
fn get_origin_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident = originsrv::OriginPackageIdent::new();
    ident.set_origin("core".to_string());
    ident.set_name("cacerts".to_string());
    ident.set_version("2017.01.17".to_string());
    ident.set_release("20170209064044".to_string());

    let mut dep_idents = protobuf::RepeatedField::new();
    let mut dep_ident = originsrv::OriginPackageIdent::new();
    dep_ident.set_origin("dep_org".to_string());
    dep_ident.set_name("dep_name".to_string());
    dep_ident.set_version("1.1.1-dep".to_string());
    dep_ident.set_release("20170101010102".to_string());
    dep_idents.push(dep_ident);
    let mut dep_ident2 = originsrv::OriginPackageIdent::new();
    dep_ident2.set_origin("dep_org2".to_string());
    dep_ident2.set_name("dep_name2".to_string());
    dep_ident2.set_version("1.1.1-dep2".to_string());
    dep_ident2.set_release("20170101010122".to_string());
    dep_idents.push(dep_ident2);

    let mut tdep_idents = protobuf::RepeatedField::new();
    let mut tdep_ident = originsrv::OriginPackageIdent::new();
    tdep_ident.set_origin("tdep_org".to_string());
    tdep_ident.set_name("tdep_name".to_string());
    tdep_ident.set_version("1.1.1-tdep".to_string());
    tdep_ident.set_release("20170101010103".to_string());
    tdep_idents.push(tdep_ident);
    let mut tdep_ident2 = originsrv::OriginPackageIdent::new();
    tdep_ident2.set_origin("tdep_org2".to_string());
    tdep_ident2.set_name("tdep_name2".to_string());
    tdep_ident2.set_version("1.1.1-tdep2".to_string());
    tdep_ident2.set_release("20170101010123".to_string());
    tdep_idents.push(tdep_ident2);

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_deps(dep_idents.clone());
    package.set_tdeps(tdep_idents.clone());
    package.set_config("config".to_string());
    package.set_target("x86_64-linux".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package).expect(
        "Failed to create origin package",
    );

    let mut package_get = originsrv::OriginPackageGet::new();
    package_get.set_ident(ident.clone());
    let result = ds.get_origin_package(&package_get)
        .expect("Failed to get origin package")
        .unwrap();

    assert_eq!(result.get_owner_id(), 1);
    assert_eq!(result.get_origin_id(), origin.get_id());
    assert_eq!(result.get_ident().to_string(), ident.to_string());
    assert_eq!(result.get_checksum(), "checksum".to_string());
    assert_eq!(result.get_manifest(), "manifest".to_string());
    assert_eq!(result.get_config(), "config".to_string());
    assert_eq!(result.get_target(), "x86_64-linux".to_string());
    assert_eq!(result.get_exposes().to_vec(), vec![1, 2]);
    assert_eq!(result.get_deps().to_vec(), dep_idents.to_vec());
    assert_eq!(result.get_tdeps().to_vec(), tdep_idents.to_vec());
}

#[test]
fn get_latest_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    package.set_target("x86_64-linux".to_string());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    package.set_target("x86_64-linux".to_string());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut package_get = originsrv::OriginPackageLatestGet::new();
    let mut search_ident = originsrv::OriginPackageIdent::new();
    search_ident.set_origin("core".to_string());
    search_ident.set_name("cacerts".to_string());
    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-windows".to_string());
    let result1 = ds.get_origin_package_latest(&package_get.clone()).unwrap();

    search_ident.set_version("2017.01.18".to_string());
    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-linux".to_string());
    let result2 = ds.get_origin_package_latest(&package_get.clone()).unwrap();

    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-windows".to_string());
    let result3 = ds.get_origin_package_latest(&package_get.clone()).unwrap();

    assert_eq!(result1.unwrap().to_string(), ident1.to_string());
    assert_eq!(result2.unwrap().to_string(), ident3.to_string());
    assert!(result3.is_none());
}

#[test]
fn list_origin_package_versions_for_origin() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut ident4 = originsrv::OriginPackageIdent::new();
    ident4.set_origin("core".to_string());
    ident4.set_name("cacerts2".to_string());
    ident4.set_version("2017.01.19".to_string());
    ident4.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident4.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opvl = originsrv::OriginPackageVersionListRequest::new();
    opvl.set_origin("core".to_string());
    opvl.set_name("cacerts".to_string());

    let result = ds.list_origin_package_versions_for_origin(&opvl.clone())
        .expect("Could not get the packages from the database");
    assert_eq!(result.get_versions().len(), 2);
    let v1 = result.get_versions().iter().nth(0).unwrap();
    assert_eq!(v1.get_version(), "2017.01.18");
    assert_eq!(v1.get_release_count(), 2);
    assert_eq!(v1.get_latest(), "20170209064045");
    let v2 = result.get_versions().iter().nth(1).unwrap();
    assert_eq!(v2.get_version(), "2017.01.17");
    assert_eq!(v2.get_release_count(), 1);
    assert_eq!(v2.get_latest(), "20170209064044");

    opvl.set_name("crazy".to_string());
    let result2 = ds.list_origin_package_versions_for_origin(&opvl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result2.get_versions().len(), 0);
}

#[test]
fn list_origin_package_for_origin() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut ident4 = originsrv::OriginPackageIdent::new();
    ident4.set_origin("core".to_string());
    ident4.set_name("cacerts2".to_string());
    ident4.set_version("2017.01.19".to_string());
    ident4.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident4.clone());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opl = originsrv::OriginPackageListRequest::new();
    opl.set_ident(
        originsrv::OriginPackageIdent::from_str("core/cacerts").unwrap(),
    );
    opl.set_start(1);
    opl.set_stop(2);
    let result = ds.list_origin_package_for_origin(&opl.clone()).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result.get_idents().len(), 2);
    assert_eq!(result.get_start(), 1);
    assert_eq!(result.get_stop(), 2);
    assert_eq!(result.get_count(), 3);
    let pkg1 = result.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident2.to_string());
    let pkg2 = result.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), ident1.to_string());

    opl.set_start(1);
    opl.set_stop(20);
    let result2 = ds.list_origin_package_for_origin(&opl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result2.get_idents().len(), 2);
    assert_eq!(result2.get_start(), 1);
    assert_eq!(result2.get_stop(), 2);
    assert_eq!(result2.get_count(), 3);

    opl.set_ident(
        originsrv::OriginPackageIdent::from_str("core/crazy").unwrap(),
    );
    opl.set_start(0);
    opl.set_stop(20);
    let result3 = ds.list_origin_package_for_origin(&opl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result3.get_idents().len(), 0);
    assert_eq!(result3.get_start(), 0);
    assert_eq!(result3.get_stop(), 20);
    assert_eq!(result3.get_count(), 0);

    opl.set_ident(
        originsrv::OriginPackageIdent::from_str("core/cacerts").unwrap(),
    );
    opl.set_distinct(true);
    opl.set_start(0);
    opl.set_stop(20);
    let result4 = ds.list_origin_package_for_origin(&opl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result4.get_idents().len(), 1);
    assert_eq!(result4.get_start(), 0);
    assert_eq!(result4.get_stop(), 0);
    assert_eq!(result4.get_count(), 1);
    let pkg3 = result4.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg3.to_string(), "core/cacerts");
}

#[test]
fn list_origin_package_for_origin_unique() {
    let ds = datastore_test!(DataStore);

    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin1 = ds.create_origin(&origin.clone())
        .expect("Should create origin")
        .unwrap();

    origin.set_name(String::from("core2"));
    let origin2 = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core2".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut ident4 = originsrv::OriginPackageIdent::new();
    ident4.set_origin("core".to_string());
    ident4.set_name("cacerts2".to_string());
    ident4.set_version("2017.01.19".to_string());
    ident4.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin1.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    package.set_origin_id(origin1.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    package.set_origin_id(origin2.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident4.clone());
    package.set_origin_id(origin1.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opl = originsrv::OriginPackageUniqueListRequest::new();
    opl.set_origin("core".to_string());
    opl.set_start(0);
    opl.set_stop(2);
    let result = ds.list_origin_package_unique_for_origin(&opl.clone())
        .expect("Could not get the packages from the database");
    assert_eq!(result.get_idents().len(), 2);
    assert_eq!(result.get_start(), 0);
    assert_eq!(result.get_stop(), 1);
    assert_eq!(result.get_count(), 2);
    let pkg1 = result.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), "core/cacerts".to_string());
    let pkg2 = result.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), "core/cacerts2".to_string());

    opl.set_origin("core2".to_string());
    opl.set_start(0);
    opl.set_stop(20);
    let result2 = ds.list_origin_package_unique_for_origin(&opl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result2.get_idents().len(), 1);
    assert_eq!(result2.get_start(), 0);
    assert_eq!(result2.get_stop(), 0);
    assert_eq!(result2.get_count(), 1);
    let pkg1 = result2.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), "core2/cacerts".to_string());
}

#[test]
fn search_origin_package_for_origin() {
    let ds = datastore_test!(DataStore);

    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin1 = ds.create_origin(&origin.clone())
        .expect("Should create origin")
        .unwrap();

    origin.set_name(String::from("core2"));
    let origin2 = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    origin.set_name(String::from("josh"));
    let origin3 = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    origin.set_name(String::from("ace"));
    let origin4 = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("red".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("red".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core2".to_string());
    ident3.set_name("red".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut ident4 = originsrv::OriginPackageIdent::new();
    ident4.set_origin("core".to_string());
    ident4.set_name("red_dog".to_string());
    ident4.set_version("2017.01.19".to_string());
    ident4.set_release("20170209064045".to_string());

    let mut ident5 = originsrv::OriginPackageIdent::new();
    ident5.set_origin("josh".to_string());
    ident5.set_name("red_dog".to_string());
    ident5.set_version("2017.01.19".to_string());
    ident5.set_release("20170209064045".to_string());

    let mut ident6 = originsrv::OriginPackageIdent::new();
    ident6.set_origin("ace".to_string());
    ident6.set_name("red_dog".to_string());
    ident6.set_version("2017.01.19".to_string());
    ident6.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin1.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    package.set_origin_id(origin1.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    package.set_origin_id(origin2.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident4.clone());
    package.set_origin_id(origin1.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident5.clone());
    package.set_origin_id(origin3.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident6.clone());
    package.set_origin_id(origin4.get_id());
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut ops = originsrv::OriginPackageSearchRequest::new();
    ops.set_origin("core".to_string());
    ops.set_query("red_".to_string());
    ops.set_start(0);
    ops.set_stop(2);
    let result = ds.search_origin_package_for_origin(&ops.clone()).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result.get_idents().len(), 1);
    assert_eq!(result.get_start(), 0);
    assert_eq!(result.get_stop(), 0);
    assert_eq!(result.get_count(), 1);
    let pkg1 = result.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident4.to_string());

    ops.set_query("red".to_string());
    ops.set_start(1);
    ops.set_stop(20);
    let result2 = ds.search_origin_package_for_origin(&ops).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result2.get_idents().len(), 2);
    assert_eq!(result2.get_start(), 1);
    assert_eq!(result2.get_stop(), 2);
    assert_eq!(result2.get_count(), 3);
    let pkg1 = result2.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident2.to_string());
    let pkg2 = result2.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), ident4.to_string());

    ops.set_query("do".to_string());
    ops.set_start(0);
    ops.set_stop(2);
    let result3 = ds.search_origin_package_for_origin(&ops).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result3.get_idents().len(), 1);
    assert_eq!(result3.get_start(), 0);
    assert_eq!(result3.get_stop(), 0);
    assert_eq!(result3.get_count(), 1);
    let pkg1 = result3.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident4.to_string());

    ops.set_query("core/re".to_string());
    ops.set_start(0);
    ops.set_stop(20);
    ops.set_distinct(true);
    let result4 = ds.search_origin_package_for_origin(&ops).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result4.get_idents().len(), 2);
    assert_eq!(result4.get_start(), 0);
    assert_eq!(result4.get_stop(), 1);
    assert_eq!(result4.get_count(), 2);
    let pkg1 = result4.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), "core/red");
    let pkg2 = result4.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), "core/red_dog");

    ops.set_query("red".to_string());
    ops.set_start(0);
    ops.set_stop(20);
    ops.set_distinct(true);
    let result5 = ds.search_origin_package_for_origin(&ops).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result5.get_idents().len(), 5);
    assert_eq!(result5.get_start(), 0);
    assert_eq!(result5.get_stop(), 4);
    assert_eq!(result5.get_count(), 1);
    let pkg1 = result5.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), "core/red");
    let pkg2 = result5.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), "core/red_dog");
    let pkg3 = result5.get_idents().iter().nth(2).unwrap();
    assert_eq!(pkg3.to_string(), "ace/red_dog");
    let pkg4 = result5.get_idents().iter().nth(3).unwrap();
    assert_eq!(pkg4.to_string(), "core2/red");
    let pkg5 = result5.get_idents().iter().nth(4).unwrap();
    assert_eq!(pkg5.to_string(), "josh/red_dog");

    ops.set_origin("".to_string());
    ops.set_query("red".to_string());
    ops.set_start(0);
    ops.set_stop(20);
    ops.set_distinct(false);
    let result6 = ds.search_origin_package_for_origin(&ops).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result6.get_idents().len(), 6);
    assert_eq!(result6.get_start(), 0);
    assert_eq!(result6.get_stop(), 5);
    assert_eq!(result6.get_count(), 1);
    let pkg1 = result6.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), "core/red/2017.01.17/20170209064044");
    let pkg2 = result6.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), "core/red/2017.01.18/20170209064044");
    let pkg3 = result6.get_idents().iter().nth(2).unwrap();
    assert_eq!(pkg3.to_string(), "core/red_dog/2017.01.19/20170209064045");
    let pkg4 = result6.get_idents().iter().nth(3).unwrap();
    assert_eq!(pkg4.to_string(), "ace/red_dog/2017.01.19/20170209064045");
    let pkg5 = result6.get_idents().iter().nth(4).unwrap();
    assert_eq!(pkg5.to_string(), "core2/red/2017.01.18/20170209064045");
    let pkg6 = result6.get_idents().iter().nth(5).unwrap();
    assert_eq!(pkg6.to_string(), "josh/red_dog/2017.01.19/20170209064045");
}

#[test]
fn create_origin_channel() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    ds.create_origin_channel(&oscc).expect(
        "Failed to create origin public key",
    );

    // Create new database connection
    let conn = ds.pool.get(&oscc).expect("Cannot get connection from pool");

    let rows = conn.query("SELECT COUNT(*) FROM origin_channels", &[])
        .expect("Failed to query database for number of channels");
    let count: i64 = rows.iter().nth(0).unwrap().get(0);
    assert_eq!(count, 3); // note: count of 3 includes the default 'unstable' and 'stable' channels
}

#[test]
fn create_origin_channel_handles_unique_constraint_violations_correctly() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    ds.create_origin_channel(&oscc).expect(
        "Failed to create origin public key",
    );

    // Create a duplicate origin channel which should fail
    let mut occ = originsrv::OriginChannelCreate::new();
    occ.set_origin_id(neurosis.get_id());
    occ.set_origin_name(neurosis.get_name().to_string());
    occ.set_name(String::from("eve"));
    let resp = ds.create_origin_channel(&oscc);

    assert!(resp.is_err(), "Insertion should've triggered an error");
}

#[test]
fn list_origin_channel() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );
    let mut oscc2 = originsrv::OriginChannelCreate::new();
    oscc2.set_origin_id(neurosis.get_id());
    oscc2.set_origin_name(neurosis.get_name().to_string());
    oscc2.set_name(String::from("online"));
    oscc2.set_owner_id(1);
    ds.create_origin_channel(&oscc2).expect(
        "Failed to create origin channel",
    );

    let mut occl = originsrv::OriginChannelListRequest::new();
    occl.set_origin_id(neurosis.get_id());
    let channels = ds.list_origin_channels(&occl).expect(
        "Could not get the channels from the database",
    );
    let channel_1 = channels.get_channels().iter().nth(0).unwrap();
    assert_eq!(channel_1.get_name(), "eve");
    let channel_2 = channels.get_channels().iter().nth(1).unwrap();
    assert_eq!(channel_2.get_name(), "online");
}

#[test]
fn list_origin_package_channels_for_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("core")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut og = originsrv::OriginGet::new();
    og.set_name(String::from("core"));
    let og_result = ds.get_origin(&og).expect("Could not get origin").unwrap();

    // Create a new origin channel
    let mut occ = originsrv::OriginChannelCreate::new();
    occ.set_origin_id(og_result.get_id());
    occ.set_origin_name(neurosis.get_name().to_string());
    occ.set_name(String::from("foo"));
    occ.set_owner_id(1);
    let channel = ds.create_origin_channel(&occ).expect(
        "Could not create channel",
    );

    let mut ident = originsrv::OriginPackageIdent::new();
    ident.set_origin("core".to_string());
    ident.set_name("red".to_string());
    ident.set_version("2017.01.17".to_string());
    ident.set_release("20170209064044".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(og_result.get_id());
    package.set_ident(ident.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut package_get = originsrv::OriginPackageGet::new();
    package_get.set_ident(ident.clone());
    let result = ds.get_origin_package(&package_get)
        .expect("Failed to get origin package")
        .unwrap();

    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel.get_id());
    opp.set_package_id(result.get_id());
    opp.set_ident(ident.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    let mut opclr = originsrv::OriginPackageChannelListRequest::new();
    opclr.set_ident(ident);
    let resp = ds.list_origin_package_channels_for_package(&opclr).expect(
        "Could not list channels for package",
    );

    assert_eq!(resp.get_channels().len(), 2); // 2 because "unstable" is implicitly created
    assert_eq!(resp.get_channels().iter().nth(0).unwrap().get_name(), "foo");
}

#[test]
fn get_origin_channel() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );
    let mut oscc2 = originsrv::OriginChannelCreate::new();
    oscc2.set_origin_id(neurosis.get_id());
    oscc2.set_origin_name(neurosis.get_name().to_string());
    oscc2.set_name(String::from("online"));
    oscc2.set_owner_id(1);
    let channel2 = ds.create_origin_channel(&oscc2).expect(
        "Failed to create origin channel",
    );

    let mut ocg = originsrv::OriginChannelGet::new();
    ocg.set_origin_name(neurosis.get_name().to_string());
    ocg.set_name(channel2.get_name().to_string());
    let channel = ds.get_origin_channel(&ocg)
        .expect("Could not get the channels from the database")
        .unwrap();

    assert_eq!(channel.get_id(), channel2.get_id());
}

#[test]
fn promote_origin_package_group() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("core")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(neurosis.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);
    let package1 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident2.clone());
    package.set_target("x86_64-linux".to_string());
    let package2 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    package.set_ident(ident3.clone());
    package.set_target("x86_64-linux".to_string());
    let package3 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut package_ids: Vec<u64> = Vec::new();
    package_ids.push(package1.get_id());
    package_ids.push(package2.get_id());
    package_ids.push(package3.get_id());

    let mut opp = originsrv::OriginPackageGroupPromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_ids(package_ids);
    opp.set_origin(origin.get_name().to_string());
    ds.promote_origin_package_group(&opp).expect(
        "Could not promote package",
    );

    let mut opl = originsrv::OriginChannelPackageListRequest::new();
    opl.set_name(channel1.get_name().to_string());
    opl.set_ident(originsrv::OriginPackageIdent::from_str("core").unwrap());
    opl.set_start(0);
    opl.set_stop(20);
    let result = ds.list_origin_channel_package_for_channel(&opl.clone())
        .expect("Could not get the packages from the database");
    assert_eq!(result.get_idents().len(), 3);
    assert_eq!(result.get_start(), 0);
    assert_eq!(result.get_stop(), 2);
    assert_eq!(result.get_count(), 3);
    let pkg1 = result.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident1.to_string());
    let pkg2 = result.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), ident2.to_string());
    let pkg3 = result.get_idents().iter().nth(2).unwrap();
    assert_eq!(pkg3.to_string(), ident3.to_string());
}

#[test]
fn get_demote_channel_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("neurosis".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(neurosis.get_id());
    package.set_ident(ident1.clone());
    let package = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_id(package.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    let mut opd = originsrv::OriginPackageDemote::new();
    opd.set_channel_id(channel1.get_id());
    opd.set_package_id(package.get_id());
    opd.set_ident(ident1);
    ds.demote_origin_package(&opd).expect(
        "Could not promote package",
    );
}

#[test]
fn get_promote_channel_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("neurosis".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(neurosis.get_id());
    package.set_ident(ident1.clone());
    let package = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_id(package.get_id());
    opp.set_ident(ident1);
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );
}

#[test]
fn get_channel_package() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("neurosis".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(neurosis.get_id());
    package.set_ident(ident1.clone());
    let package = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_id(package.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    let mut package_get = originsrv::OriginChannelPackageGet::new();
    package_get.set_ident(ident1.clone());
    package_get.set_name(channel1.get_name().to_string());
    let result = ds.get_origin_channel_package(&package_get)
        .expect("Failed to get origin channel package")
        .unwrap();

    assert_eq!(result.get_owner_id(), 1);
    assert_eq!(result.get_origin_id(), neurosis.get_id());
    assert_eq!(result.get_ident().to_string(), ident1.to_string());
}

#[test]
fn get_latest_channel_package() {
    let ds = datastore_test!(DataStore);

    //create origin
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(origin.get_id());
    oscc.set_origin_name(origin.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    // Create another origin channel
    let mut oscc2 = originsrv::OriginChannelCreate::new();
    oscc2.set_origin_id(origin.get_id());
    oscc2.set_origin_name(origin.get_name().to_string());
    oscc2.set_name(String::from("online"));
    oscc2.set_owner_id(1);
    let channel2 = ds.create_origin_channel(&oscc2).expect(
        "Failed to create origin channel",
    );

    //setup idents
    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    //setup package
    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);

    // package1 core/cacerts/2017.01.17/20170209064044 windows
    let package1 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // package2 core/cacerts/2017.01.18/20170209064044 linux
    package.set_ident(ident2.clone());
    package.set_target("x86_64-linux".to_string());
    let package2 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // package3 core/cacerts/2017.01.18/20170209064045 linux
    package.set_ident(ident3.clone());
    package.set_target("x86_64-linux".to_string());
    let package3 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // channel1/package1
    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_id(package1.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel1/package2
    opp.set_package_id(package2.get_id());
    opp.set_ident(ident2.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel1/package3
    opp.set_package_id(package3.get_id());
    opp.set_ident(ident3.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel2/package1
    opp.set_channel_id(channel2.get_id());
    opp.set_package_id(package1.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel2/package2
    opp.set_package_id(package2.get_id());
    opp.set_ident(ident2.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    let mut package_get = originsrv::OriginChannelPackageLatestGet::new();
    package_get.set_name(channel1.get_name().to_string());
    let mut search_ident = originsrv::OriginPackageIdent::new();
    search_ident.set_origin("core".to_string());
    search_ident.set_name("cacerts".to_string());
    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-windows".to_string());

    // latest core/cacerts in channel1 for windows
    let result1 = ds.get_origin_channel_package_latest(&package_get.clone())
        .unwrap();

    search_ident.set_version("2017.01.18".to_string());
    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-linux".to_string());

    // latest core/cacerts/2017.01.18 in channel1 for linux
    let result2 = ds.get_origin_channel_package_latest(&package_get.clone())
        .unwrap();

    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-windows".to_string());

    // latest core/cacerts/2017.01.18 in channel1 for windows
    let result3 = ds.get_origin_channel_package_latest(&package_get.clone())
        .unwrap();

    package_get.set_name(channel2.get_name().to_string());
    search_ident.set_version("".to_string());
    package_get.set_ident(search_ident.clone());
    package_get.set_target("x86_64-linux".to_string());

    // latest core/cacerts in channel2 for linux
    let result4 = ds.get_origin_channel_package_latest(&package_get.clone())
        .unwrap();

    assert_eq!(result1.unwrap().to_string(), ident1.to_string());
    assert_eq!(result2.unwrap().to_string(), ident3.to_string());
    assert!(result3.is_none());
    assert_eq!(result4.unwrap().to_string(), ident2.to_string());
}

#[test]
fn list_origin_channel_package_for_channel() {
    let ds = datastore_test!(DataStore);

    //create origin
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("core"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    let origin = ds.create_origin(&origin)
        .expect("Should create origin")
        .unwrap();

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(origin.get_id());
    oscc.set_origin_name(origin.get_name().to_string());
    oscc.set_name(String::from("eve"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    // Create another origin channel
    let mut oscc2 = originsrv::OriginChannelCreate::new();
    oscc2.set_origin_id(origin.get_id());
    oscc2.set_origin_name(origin.get_name().to_string());
    oscc2.set_name(String::from("online"));
    oscc2.set_owner_id(1);
    let channel2 = ds.create_origin_channel(&oscc2).expect(
        "Failed to create origin channel",
    );

    //setup idents
    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("core".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    let mut ident2 = originsrv::OriginPackageIdent::new();
    ident2.set_origin("core".to_string());
    ident2.set_name("cacerts".to_string());
    ident2.set_version("2017.01.18".to_string());
    ident2.set_release("20170209064044".to_string());

    let mut ident3 = originsrv::OriginPackageIdent::new();
    ident3.set_origin("core".to_string());
    ident3.set_name("cacerts".to_string());
    ident3.set_version("2017.01.18".to_string());
    ident3.set_release("20170209064045".to_string());

    let mut ident4 = originsrv::OriginPackageIdent::new();
    ident4.set_origin("core".to_string());
    ident4.set_name("cacerts2".to_string());
    ident4.set_version("2017.01.19".to_string());
    ident4.set_release("20170209064045".to_string());

    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(origin.get_id());
    package.set_ident(ident1.clone());
    package.set_checksum("checksum".to_string());
    package.set_manifest("manifest".to_string());
    package.set_config("config".to_string());
    package.set_target("x86_64-windows".to_string());
    package.set_exposes(vec![1, 2]);

    // package1 core/cacerts/2017.01.17/20170209064044
    let package1 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // package2 core/cacerts/2017.01.18/20170209064044
    package.set_ident(ident2.clone());
    let package2 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // package3 core/cacerts/2017.01.18/20170209064045
    package.set_ident(ident3.clone());
    let package3 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // package1 core/cacerts2/2017.01.19/20170209064045
    package.set_ident(ident4.clone());
    let package4 = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // channel1/package1
    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel1.get_id());
    opp.set_package_id(package1.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel1/package2
    opp.set_package_id(package2.get_id());
    opp.set_ident(ident2.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel2/package3
    opp.set_channel_id(channel2.get_id());
    opp.set_package_id(package3.get_id());
    opp.set_ident(ident3.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // channel2/package4
    opp.set_package_id(package4.get_id());
    opp.set_ident(ident4.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    let mut opl = originsrv::OriginChannelPackageListRequest::new();
    opl.set_name(channel1.get_name().to_string());
    opl.set_ident(
        originsrv::OriginPackageIdent::from_str("core/cacerts").unwrap(),
    );
    opl.set_start(0);
    opl.set_stop(2);
    let result = ds.list_origin_channel_package_for_channel(&opl.clone())
        .expect("Could not get the packages from the database");
    assert_eq!(result.get_idents().len(), 2);
    assert_eq!(result.get_start(), 0);
    assert_eq!(result.get_stop(), 1);
    assert_eq!(result.get_count(), 2);
    let pkg1 = result.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident1.to_string());
    let pkg2 = result.get_idents().iter().nth(1).unwrap();
    assert_eq!(pkg2.to_string(), ident2.to_string());

    opl.set_name(channel2.get_name().to_string());
    opl.set_ident(originsrv::OriginPackageIdent::from_str("core").unwrap());
    opl.set_start(1);
    opl.set_stop(20);
    let result2 = ds.list_origin_channel_package_for_channel(&opl).expect(
        "Could not get the packages from the database",
    );
    assert_eq!(result2.get_idents().len(), 1);
    assert_eq!(result2.get_start(), 1);
    assert_eq!(result2.get_stop(), 1);
    assert_eq!(result2.get_count(), 2);
    let pkg1 = result2.get_idents().iter().nth(0).unwrap();
    assert_eq!(pkg1.to_string(), ident4.to_string());
}

#[test]
fn delete_origin_channel_by_name() {
    let ds = datastore_test!(DataStore);
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin channel
    let mut oscc = originsrv::OriginChannelCreate::new();
    oscc.set_origin_id(neurosis.get_id());
    oscc.set_origin_name(neurosis.get_name().to_string());
    oscc.set_name(String::from("arthas"));
    oscc.set_owner_id(1);
    let channel1 = ds.create_origin_channel(&oscc).expect(
        "Failed to create origin channel",
    );

    // Check that channel exists
    let mut ocg = originsrv::OriginChannelGet::new();
    ocg.set_origin_name(neurosis.get_name().to_string());
    ocg.set_name(channel1.get_name().to_string());
    let _ = ds.get_origin_channel(&ocg)
        .expect("Could not get the channels from the database")
        .unwrap();

    // Delete channel
    let mut ocd = originsrv::OriginChannelDelete::new();
    ocd.set_id(channel1.get_id());
    ocd.set_origin_id(neurosis.get_id());
    assert!(
        ds.delete_origin_channel_by_id(&ocd).is_ok(),
        "Should delete existing channel"
    );

    // Check that channel no longer exists
    let mut ocg2 = originsrv::OriginChannelGet::new();
    ocg2.set_origin_name(neurosis.get_name().to_string());
    ocg2.set_name(channel1.get_name().to_string());

    assert!(
        ds.get_origin_channel(&ocg)
            .expect("Error getting channel from database")
            .is_none(),
        "Channel should not exist"
    );

    // Test for condition where a channel has a promoted package

    // Create a new origin channel
    let mut oscc2 = originsrv::OriginChannelCreate::new();
    oscc2.set_origin_id(neurosis.get_id());
    oscc2.set_origin_name(neurosis.get_name().to_string());
    oscc2.set_name(String::from("sylvanas"));
    oscc2.set_owner_id(1);
    let channel2 = ds.create_origin_channel(&oscc2).expect(
        "Failed to create origin channel",
    );

    // Check that channel exists
    let mut ocg2 = originsrv::OriginChannelGet::new();
    ocg2.set_origin_name(neurosis.get_name().to_string());
    ocg2.set_name(channel2.get_name().to_string());
    let _ = ds.get_origin_channel(&ocg2)
        .expect("Could not get the channels from the database")
        .unwrap();

    // Create a new package ident
    let mut ident1 = originsrv::OriginPackageIdent::new();
    ident1.set_origin("neurosis".to_string());
    ident1.set_name("cacerts".to_string());
    ident1.set_version("2017.01.17".to_string());
    ident1.set_release("20170209064044".to_string());

    //Create a new package
    let mut package = originsrv::OriginPackageCreate::new();
    package.set_owner_id(1);
    package.set_origin_id(neurosis.get_id());
    package.set_ident(ident1.clone());
    let package = ds.create_origin_package(&package.clone()).expect(
        "Failed to create origin package",
    );

    // Promote that package to the channel
    let mut opp = originsrv::OriginPackagePromote::new();
    opp.set_channel_id(channel2.get_id());
    opp.set_package_id(package.get_id());
    opp.set_ident(ident1.clone());
    ds.promote_origin_package(&opp).expect(
        "Could not promote package",
    );

    // Check that there is a relationship between the package and the channel
    let mut package_get = originsrv::OriginChannelPackageGet::new();
    package_get.set_ident(ident1.clone());
    package_get.set_name(channel2.get_name().to_string());
    let _ = ds.get_origin_channel_package(&package_get)
        .expect("Failed to get origin channel package")
        .unwrap();

    // Delete channel
    let mut ocd2 = originsrv::OriginChannelDelete::new();
    ocd2.set_origin_id(neurosis.get_id());
    ocd2.set_id(channel2.get_id());
    assert!(
        ds.delete_origin_channel_by_id(&ocd2).is_ok(),
        "Should delete existing channel"
    );

    // Check that the package still exists
    let mut package_get = originsrv::OriginPackageGet::new();
    package_get.set_ident(ident1.clone());
    let _ = ds.get_origin_package(&package_get)
        .expect("Failed to get origin package")
        .unwrap();


    // Check that there is no longer a relationship between the deleted channel and the package
    let mut package_get = originsrv::OriginChannelPackageGet::new();
    package_get.set_ident(ident1.clone());
    package_get.set_name(channel2.get_name().to_string());
    assert!(
        ds.get_origin_channel_package(&package_get)
            .expect("Error getting package from database")
            .is_none(),
        "Channel package should not exist"
    );
}
