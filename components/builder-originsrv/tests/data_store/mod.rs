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

use protocol::originsrv;
use originsrv::data_store::DataStore;

#[test]
fn migration() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
    });
}

#[test]
fn create_origin() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        let mut origin = originsrv::OriginCreate::new();
        origin.set_name(String::from("neurosis"));
        origin.set_owner_id(1);
        origin.set_owner_name(String::from("scottkelly"));
        ds.create_origin(&origin).expect("Should create origin");
    });
}

#[test]
fn get_origin_by_name() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        let mut origin = originsrv::OriginCreate::new();
        origin.set_name(String::from("neurosis"));
        origin.set_owner_id(1);
        origin.set_owner_name(String::from("scottkelly"));
        ds.create_origin(&origin).expect("Should create origin");

        let new_origin = ds.get_origin_by_name("neurosis").expect("Could not get the origin");
        assert!(new_origin.is_some(), "Origin did not exist");
        let fg = new_origin.unwrap();
        assert_eq!(fg.get_name(), "neurosis");
        assert_eq!(fg.get_owner_id(), 1);
        assert_eq!(fg.get_private_key_name(), "");
    });
}

#[test]
fn create_origin_secret_key() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");

    // Origin secret keys get returned with the origin
    let first = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(first.get_private_key_name(), "neurosis-20160612031944");

    // They are also sorted based on the latest key if there is more than one
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");
    let second = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(second.get_private_key_name(), "neurosis-20160612031945");
}

#[test]
fn get_origin_secret_key() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");

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
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_public_key(&oskc).expect("Failed to create origin public key");
    let mut oskc2 = originsrv::OriginPublicKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_revision(String::from("20160612031945"));
    oskc2.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc2).expect("Failed to create origin public key");

    let mut oskg = originsrv::OriginPublicKeyGet::new();
    oskg.set_owner_id(1);
    oskg.set_origin(oskc.get_name().to_string());
    oskg.set_revision(String::from("20160612031944"));
    let key =
        ds.get_origin_public_key(&oskg).expect("Could not get the key").expect("key did not exist");
    assert_eq!(key.get_body(), oskc.get_body());
}

#[test]
fn get_origin_public_key_latest() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_public_key(&oskc).expect("Failed to create origin public key");
    oskc.set_revision(String::from("20160612031945"));
    oskc.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc).expect("Failed to create origin public key");

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
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_public_key(&oskc).expect("Failed to create origin public key");
    let mut oskc2 = originsrv::OriginPublicKeyCreate::new();
    oskc2.set_name(String::from("neurosis"));
    oskc2.set_origin_id(neurosis.get_id());
    oskc2.set_owner_id(1);
    oskc2.set_revision(String::from("20160612031945"));
    oskc2.set_body(String::from("very_very_public").into_bytes());
    ds.create_origin_public_key(&oskc2).expect("Failed to create origin public key");

    let mut oskl = originsrv::OriginPublicKeyListRequest::new();
    oskl.set_origin_id(neurosis.get_id());
    let keys = ds.list_origin_public_keys_for_origin(&oskl)
        .expect("Could not get the keys from the database");
    assert_eq!(keys.get_keys().len(), 2);
    let key1 = keys.get_keys()
        .iter()
        .nth(0)
        .unwrap();
    assert_eq!(key1.get_revision(), "20160612031945");
    let key2 = keys.get_keys()
        .iter()
        .nth(1)
        .unwrap();
    assert_eq!(key2.get_revision(), "20160612031944");
}

#[test]
fn create_origin_invitation() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation again, which should be a no-op");

    oic.set_owner_id(5);
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation again, which should be a no-op");

    // We should never create an invitation for the same person and org
    let conn = ds.pool.get().expect("Cannot get connection from pool");
    let rows = conn.query("SELECT COUNT(*) FROM origin_invitations", &[])
        .expect("Failed to query database for number of invitations");
    let count: i64 = rows.iter()
        .nth(0)
        .unwrap()
        .get(0);
    assert_eq!(count, 1);
}

#[test]
fn list_origin_invitations_for_origin() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    let no_invites = ds.list_origin_invitations_for_origin(&oilr)
        .expect("Failed to get origin list from database");
    assert_eq!(no_invites.get_invitations().len(),
               0,
               "We have invitations when we should have none");

    let mut oic = originsrv::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(2);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    oic.set_account_id(3);
    oic.set_account_name(String::from("maynard_james_keenan"));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    oic.set_account_id(4);
    oic.set_account_name(String::from("danny_cary"));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    // List comes back in alphabetical order by origin
    let oi_list = ds.list_origin_invitations_for_origin(&oilr)
        .expect("Could not get origin invitation list from database");
    assert_eq!(oi_list.get_invitations().len(), 3);
    let danny = oi_list.get_invitations()
        .iter()
        .nth(0)
        .unwrap();
    assert_eq!(danny.get_account_id(), 4);
    let maynard = oi_list.get_invitations()
        .iter()
        .nth(1)
        .unwrap();
    assert_eq!(maynard.get_account_id(), 3);
    let noel = oi_list.get_invitations()
        .iter()
        .nth(2)
        .unwrap();
    assert_eq!(noel.get_account_id(), 2);
}

#[test]
fn list_origin_invitations_for_account() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut torigin = originsrv::OriginCreate::new();
    torigin.set_name(String::from("tool"));
    torigin.set_owner_id(2);
    torigin.set_owner_name(String::from("maynard"));
    ds.create_origin(&torigin).expect("Should create origin");

    let tool = ds.get_origin_by_name("tool")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut ailr = originsrv::AccountInvitationListRequest::new();
    ailr.set_account_id(3);
    let no_invites = ds.list_origin_invitations_for_account(&ailr)
        .expect("Failed to get origin list from database");
    assert!(no_invites.is_none(),
            "We have invitations when we should have none");

    let mut oic = originsrv::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(3);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    oic.set_origin_id(tool.get_id());
    oic.set_origin_name(String::from(tool.get_name()));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    // We shouldn't see mr pants in our result set
    oic.set_account_id(4);
    oic.set_account_name(String::from("poopy_pants"));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    // List comes back in alphabetical order by origin
    let oi_list = ds.list_origin_invitations_for_account(&ailr)
        .expect("Could not get origin invitation list from database")
        .expect("No origin invites for origin that should have 3");
    assert_eq!(oi_list.len(), 2);

    let neurosis = oi_list.iter().nth(0).unwrap();
    assert_eq!(neurosis.get_origin_name(), "neurosis");
    let tool = oi_list.iter().nth(1).unwrap();
    assert_eq!(tool.get_origin_name(), "tool");
}

#[test]
fn accept_origin_invitation() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
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
    oic.set_account_id(3);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    let mut ailr = originsrv::AccountInvitationListRequest::new();
    ailr.set_account_id(3);
    let invite = ds.list_origin_invitations_for_account(&ailr)
        .expect("failed to get invitations from database")
        .expect("there should be invitatations")
        .into_iter()
        .nth(0)
        .expect("there should be an invitation");

    let mut oiar = originsrv::OriginInvitationAcceptRequest::new();
    oiar.set_account_accepting_request(3);
    oiar.set_invite_id(invite.get_id() as u64);
    oiar.set_ignore(false);
    ds.accept_origin_invitation(&oiar).expect("Failed to accept origin invitation");

    // Accepting an invitation means deleting the invite from the table
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Invitations were not deleted on acceptance");

    // Create the invitation again - it should not take, because the member exists
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Invitation was created even if the member exists");

    oic.set_account_id(4);
    oic.set_account_name(String::from("steve_perry"));
    ds.create_origin_invitation(&oic).expect("Failed to create an origin invitation");
    ailr.set_account_id(4);
    let steves_invite = ds.list_origin_invitations_for_account(&ailr)
        .expect("failed to get invitations from database")
        .expect("there should be invitatations")
        .into_iter()
        .nth(0)
        .expect("there should be an invitation");
    oiar.set_account_accepting_request(4);
    oiar.set_invite_id(steves_invite.get_id() as u64);
    oiar.set_ignore(true);
    ds.accept_origin_invitation(&oiar).expect("Failed to accept origin invitation");

    // Accepting with an ignore means you will never see this request
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Ignored Invitations were not removed from the list on acceptance");

    let mut omlr = originsrv::OriginMemberListRequest::new();
    omlr.set_origin_id(neurosis.get_id());
    let members =
        ds.list_origin_members(&omlr).expect("Failed to get origin members from the database");
    assert!(members.get_members().contains(&String::from("scottkelly")),
            "scotkelly should be a member");
    assert!(members.get_members().contains(&String::from("noel_gallagher")),
            "noel_gallagher should be a member");
    assert!(!members.get_members().contains(&String::from("steve_perry")),
            "steve_perry is a member, but he ignored his invite");
}

#[test]
fn check_account_in_origin() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut coar = originsrv::CheckOriginAccessRequest::new();
    coar.set_origin_name(String::from("neurosis"));
    coar.set_account_id(1);

    assert!(ds.check_account_in_origin(&coar).expect("failed to check membership in the database"),
            "Member should have been in the origin");
}

#[test]
fn create_origin_project() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    ds.create_origin_project(&opc).expect("Failed to create origin project");
}

#[test]
fn get_origin_project_by_name() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    ds.create_origin_project(&opc);

    let project = ds.get_origin_project_by_name("neurosis/zeal")
        .expect("Error getting project form database")
        .expect("Project does not exist");
    assert!(project.get_id() != 0, "Should have a real ID");
    assert_eq!(project.get_origin_id(),
               neurosis.get_id(),
               "Should have the right origin id");
    assert_eq!(project.get_origin_name(),
               "neurosis",
               "Should have the right origin name");
    assert_eq!(project.get_package_name(),
               "zeal",
               "Should have zeal as the package name");
    assert_eq!(project.get_name(),
               "neurosis/zeal",
               "Should have neurosis/zeal as the project name");
    assert_eq!(project.get_plan_path(),
               "foo",
               "Should have foo as the plan path");
    assert_eq!(project.get_owner_id(), 1, "Should have the right owner id");
    assert_eq!(project.get_vcs_type(),
               "git",
               "Should have the right vcs type");
    assert_eq!(project.get_vcs_data(),
               "git://github.com/habitat-sh/core-plans",
               "Should have the right vcs data");
}

#[test]
fn delete_origin_project_by_name() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op);

    ds.create_origin_project(&opc).expect("Failed to create project in database");
    assert!(ds.delete_origin_project_by_name("neurosis/zeal").is_ok(),
            "Should delete existing project");

    assert!(ds.get_origin_project_by_name("neurosis/zeal")
                .expect("Error getting project form database")
                .is_none(),
            "Project should not exist");
}

#[test]
fn update_origin_project() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = originsrv::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut op = originsrv::OriginProject::new();
    op.set_origin_name(String::from("neurosis"));
    op.set_package_name(String::from("zeal"));
    op.set_plan_path(String::from("foo"));
    op.set_vcs_type(String::from("git"));
    op.set_vcs_data(String::from("git://github.com/habitat-sh/core-plans"));
    op.set_owner_id(1);

    let mut opc = originsrv::OriginProjectCreate::new();
    opc.set_project(op.clone());

    ds.create_origin_project(&opc).expect("Failed to create project in database");

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

    ds.update_origin_project(&opu).expect("Failed to update project in database");

    let sepultura = ds.get_origin_project_by_name("neurosis/sepultura")
        .expect("Error getting project form database")
        .expect("Project does not exist");
    assert_eq!(updated_project.get_id(),
               sepultura.get_id(),
               "Should have a the same ID");
    assert_eq!(updated_project.get_origin_id(),
               sepultura.get_origin_id(),
               "Should have the same origin id");
    assert_eq!(updated_project.get_origin_name(),
               sepultura.get_origin_name(),
               "Should have the same origin name");
    assert_eq!(sepultura.get_package_name(),
               "sepultura",
               "Should have the correct package name");
    assert_eq!(sepultura.get_name(),
               "neurosis/sepultura",
               "Should have neurosis/sepultura as the project name");
    assert_eq!(updated_project.get_plan_path(),
               sepultura.get_plan_path(),
               "Should have the same plan path");
    assert_eq!(updated_project.get_owner_id(),
               sepultura.get_owner_id(),
               "Should have the same owner id");
    assert_eq!(updated_project.get_vcs_type(),
               sepultura.get_vcs_type(),
               "Should have the updated vcs type");
    assert_eq!(updated_project.get_vcs_data(),
               sepultura.get_vcs_data(),
               "Should have the same vcs data");
}
