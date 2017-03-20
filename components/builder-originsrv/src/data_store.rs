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

//! The PostgreSQL backend for the Vault.

use db::pool::Pool;
use db::migration::Migrator;
use protocol::originsrv;
use postgres;
use protobuf;

use config::Config;
use error::{Result, Error};
use migrations;

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
}

impl DataStore {
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore_connection_url,
                             config.pool_size,
                             config.datastore_connection_retry_ms,
                             config.datastore_connection_timeout,
                             config.datastore_connection_test)?;
        Ok(DataStore { pool: pool })
    }

    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        Ok(DataStore { pool: pool })
    }

    pub fn setup(&self) -> Result<()> {
        let mut migrator = Migrator::new(&self.pool);
        migrator.setup()?;

        // The order here matters. Once you have deployed the software, you can never change it.
        migrations::next_id::migrate(&mut migrator)?;
        migrations::origins::migrate(&mut migrator)?;
        migrations::origin_public_keys::migrate(&mut migrator)?;
        migrations::origin_secret_keys::migrate(&mut migrator)?;
        migrations::origin_invitations::migrate(&mut migrator)?;
        migrations::origin_projects::migrate(&mut migrator)?;

        Ok(())
    }

    pub fn update_origin_project(&self, opc: &originsrv::OriginProjectUpdate) -> Result<()> {
        let conn = self.pool.get()?;
        let project = opc.get_project();
        conn.execute("SELECT update_origin_project_v1($1, $2, $3, $4, $5, $6, $7)",
                     &[&(project.get_id() as i64),
                       &(project.get_origin_id() as i64),
                       &project.get_package_name(),
                       &project.get_plan_path(),
                       &project.get_vcs_type(),
                       &project.get_vcs_data(),
                       &(project.get_owner_id() as i64)])
            .map_err(Error::OriginProjectUpdate)?;
        Ok(())
    }

    pub fn delete_origin_project_by_name(&self, name: &str) -> Result<()> {
        let conn = self.pool.get()?;
        conn.execute("SELECT delete_origin_project_v1($1)", &[&name])
            .map_err(Error::OriginProjectDelete)?;
        Ok(())
    }

    pub fn get_origin_project_by_name(&self,
                                      name: &str)
                                      -> Result<Option<originsrv::OriginProject>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_project_v1($1)", &[&name])
                        .map_err(Error::OriginProjectGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_project(&row)))
        } else {
            Ok(None)
        }
    }

    pub fn row_to_origin_project(&self, row: &postgres::rows::Row) -> originsrv::OriginProject {
        let mut project = originsrv::OriginProject::new();
        let id: i64 = row.get("id");
        project.set_id(id as u64);
        let origin_id: i64 = row.get("origin_id");
        project.set_origin_id(origin_id as u64);
        let owner_id: i64 = row.get("owner_id");
        project.set_owner_id(owner_id as u64);
        project.set_origin_name(row.get("origin_name"));
        project.set_package_name(row.get("package_name"));
        project.set_name(row.get("name"));
        project.set_plan_path(row.get("plan_path"));
        project.set_vcs_type(row.get("vcs_type"));
        project.set_vcs_data(row.get("vcs_data"));
        project
    }

    pub fn create_origin_project(&self,
                                 opc: &originsrv::OriginProjectCreate)
                                 -> Result<originsrv::OriginProject> {
        let conn = self.pool.get()?;
        let project = opc.get_project();
        let rows = conn.query("SELECT * FROM insert_origin_project_v1($1, $2, $3, $4, $5, $6)",
                              &[&project.get_origin_name(),
                                &project.get_package_name(),
                                &project.get_plan_path(),
                                &project.get_vcs_type(),
                                &project.get_vcs_data(),
                                &(project.get_owner_id() as i64)])
            .map_err(Error::OriginProjectCreate)?;
        let row = rows.get(0);
        Ok(self.row_to_origin_project(&row))
    }

    // Take that, long line zealots - I'll out function name length you yet, I swear to all that is
    // holy.
    pub fn check_account_in_origin_by_origin_and_account_id(&self,
                                                            origin_name: &str,
                                                            account_id: i64)
                                                            -> Result<bool> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM check_account_in_origin_members_v1($1, $2)",
                               &[&origin_name, &account_id])
                        .map_err(Error::OriginAccountInOrigin)?;
        if rows.len() != 0 { Ok(true) } else { Ok(false) }
    }

    pub fn check_account_in_origin(&self,
                                   coar: &originsrv::CheckOriginAccessRequest)
                                   -> Result<bool> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM check_account_in_origin_members_v1($1, $2)",
                               &[&coar.get_origin_name(), &(coar.get_account_id() as i64)])
                        .map_err(Error::OriginAccountInOrigin)?;
        if rows.len() != 0 { Ok(true) } else { Ok(false) }
    }

    pub fn list_origins_by_account(&self,
                                   aolr: &originsrv::AccountOriginListRequest)
                                   -> Result<originsrv::AccountOriginListResponse> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM list_origin_by_account_id($1)",
                               &[&(aolr.get_account_id() as i64)])
                        .map_err(Error::OriginAccountList)?;
        let mut response = originsrv::AccountOriginListResponse::new();
        response.set_account_id(aolr.get_account_id().clone());
        let mut origins = protobuf::RepeatedField::new();
        for row in rows {
            origins.push(row.get("origin_name"));
        }
        response.set_origins(origins);
        Ok(response)
    }

    pub fn list_origin_members(&self,
                               omlr: &originsrv::OriginMemberListRequest)
                               -> Result<originsrv::OriginMemberListResponse> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM list_origin_members_v1($1)",
                               &[&(omlr.get_origin_id() as i64)])
                        .map_err(Error::OriginMemberList)?;

        let mut response = originsrv::OriginMemberListResponse::new();
        response.set_origin_id(omlr.get_origin_id());

        let mut members = protobuf::RepeatedField::new();
        for row in rows {
            members.push(row.get("account_name"));
        }

        response.set_members(members);
        Ok(response)
    }

    pub fn validate_origin_invitation(&self,
                                      oiar: &originsrv::OriginInvitationValidateRequest)
                                      -> Result<originsrv::OriginInvitationValidateResponse> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM validate_origin_invitation_v1($1, $2)",
                               &[&(oiar.get_invite_id() as i64),
                                 &(oiar.get_account_accepting_request() as i64)])
                        .map_err(Error::OriginInvitationValidate)?;
        let mut response = originsrv::OriginInvitationValidateResponse::new();
        if rows.len() > 0 {
            response.set_is_valid(true);
        } else {
            response.set_is_valid(false);
        }
        Ok(response)
    }

    pub fn accept_origin_invitation(&self,
                                    oiar: &originsrv::OriginInvitationAcceptRequest)
                                    -> Result<()> {
        let conn = self.pool.get()?;
        let tr = conn.transaction().map_err(Error::DbTransactionStart)?;
        tr.execute("SELECT * FROM accept_origin_invitation_v1($1, $2)",
                     &[&(oiar.get_invite_id() as i64), &oiar.get_ignore()])
            .map_err(Error::OriginInvitationAccept)?;
        tr.commit().map_err(Error::DbTransactionCommit)?;
        Ok(())
    }

    pub fn list_origin_invitations_for_account
        (&self,
         oilr: &originsrv::AccountInvitationListRequest)
         -> Result<Option<Vec<originsrv::OriginInvitation>>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_invitations_for_account_v1($1)",
                               &[&(oilr.get_account_id() as i64)])
                        .map_err(Error::OriginInvitationListForAccount)?;
        if rows.len() != 0 {
            let mut list_of_oi: Vec<originsrv::OriginInvitation> = Vec::new();
            for row in rows {
                list_of_oi.push(self.row_to_origin_invitation(&row));
            }
            Ok(Some(list_of_oi))
        } else {
            Ok(None)
        }
    }

    pub fn list_origin_invitations_for_origin
        (&self,
         oilr: &originsrv::OriginInvitationListRequest)
         -> Result<originsrv::OriginInvitationListResponse> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_invitations_for_origin_v1($1)",
                               &[&(oilr.get_origin_id() as i64)])
                        .map_err(Error::OriginInvitationListForOrigin)?;

        let mut response = originsrv::OriginInvitationListResponse::new();
        response.set_origin_id(oilr.get_origin_id());
        let mut invitations = protobuf::RepeatedField::new();
        for row in rows {
            invitations.push(self.row_to_origin_invitation(&row));
        }
        response.set_invitations(invitations);
        Ok(response)
    }

    fn row_to_origin_invitation(&self, row: &postgres::rows::Row) -> originsrv::OriginInvitation {
        let mut oi = originsrv::OriginInvitation::new();
        let oi_id: i64 = row.get("id");
        oi.set_id(oi_id as u64);
        let oi_account_id: i64 = row.get("account_id");
        oi.set_account_id(oi_account_id as u64);
        oi.set_account_name(row.get("account_name"));
        let oi_origin_id: i64 = row.get("origin_id");
        oi.set_origin_id(oi_origin_id as u64);
        oi.set_origin_name(row.get("origin_name"));
        let oi_owner_id: i64 = row.get("owner_id");
        oi.set_owner_id(oi_owner_id as u64);
        oi
    }

    pub fn create_origin_invitation(&self,
                                    oic: &originsrv::OriginInvitationCreate)
                                    -> Result<Option<originsrv::OriginInvitation>> {
        let conn = self.pool.get()?;
        let rows = conn.query("SELECT * FROM insert_origin_invitation_v1($1, $2, $3, $4, $5)",
                              &[&(oic.get_origin_id() as i64),
                                &oic.get_origin_name(),
                                &(oic.get_account_id() as i64),
                                &oic.get_account_name(),
                                &(oic.get_owner_id() as i64)])
            .map_err(Error::OriginInvitationCreate)?;
        if rows.len() == 1 {
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_invitation(&row)))
        } else {
            Ok(None)
        }
    }

    pub fn create_origin_secret_key(&self,
                                    osk: &originsrv::OriginSecretKeyCreate)
                                    -> Result<originsrv::OriginSecretKey> {
        let conn = self.pool.get()?;
        let rows = conn.query("SELECT * FROM insert_origin_secret_key_v1($1, $2, $3, $4, $5, $6)",
                              &[&(osk.get_origin_id() as i64),
                                &(osk.get_owner_id() as i64),
                                &osk.get_name(),
                                &osk.get_revision(),
                                &format!("{}-{}", osk.get_name(), osk.get_revision()),
                                &osk.get_body()])
            .map_err(Error::OriginSecretKeyCreate)?;
        let row = rows.iter().nth(0).expect("Insert returns row, but no row present");
        Ok(self.row_to_origin_secret_key(row))
    }

    fn row_to_origin_secret_key(&self, row: postgres::rows::Row) -> originsrv::OriginSecretKey {
        let mut osk = originsrv::OriginSecretKey::new();
        let osk_id: i64 = row.get("id");
        osk.set_id(osk_id as u64);
        let osk_origin_id: i64 = row.get("origin_id");
        osk.set_origin_id(osk_origin_id as u64);
        osk.set_name(row.get("name"));
        osk.set_revision(row.get("revision"));
        osk.set_body(row.get("body"));
        let osk_owner_id: i64 = row.get("owner_id");
        osk.set_owner_id(osk_owner_id as u64);
        osk
    }

    pub fn get_origin_secret_key(&self,
                                 osk_get: &originsrv::OriginSecretKeyGet)
                                 -> Result<Option<originsrv::OriginSecretKey>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_secret_key_v1($1)",
                               &[&osk_get.get_origin()])
                        .map_err(Error::OriginSecretKeyGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_secret_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn create_origin_public_key(&self,
                                    opk: &originsrv::OriginPublicKeyCreate)
                                    -> Result<originsrv::OriginPublicKey> {
        let conn = self.pool.get()?;
        let rows = conn.query("SELECT * FROM insert_origin_public_key_v1($1, $2, $3, $4, $5, $6)",
                              &[&(opk.get_origin_id() as i64),
                                &(opk.get_owner_id() as i64),
                                &opk.get_name(),
                                &opk.get_revision(),
                                &format!("{}-{}", opk.get_name(), opk.get_revision()),
                                &opk.get_body()])
            .map_err(Error::OriginPublicKeyCreate)?;
        let row = rows.iter().nth(0).expect("Insert returns row, but no row present");
        Ok(self.row_to_origin_public_key(row))
    }

    fn row_to_origin_public_key(&self, row: postgres::rows::Row) -> originsrv::OriginPublicKey {
        let mut opk = originsrv::OriginPublicKey::new();
        let opk_id: i64 = row.get("id");
        opk.set_id(opk_id as u64);
        let opk_origin_id: i64 = row.get("origin_id");
        opk.set_origin_id(opk_origin_id as u64);
        opk.set_name(row.get("name"));
        opk.set_revision(row.get("revision"));
        opk.set_body(row.get("body"));
        let opk_owner_id: i64 = row.get("owner_id");
        opk.set_owner_id(opk_owner_id as u64);
        opk
    }

    pub fn get_origin_public_key(&self,
                                 opk_get: &originsrv::OriginPublicKeyGet)
                                 -> Result<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_public_key_v1($1, $2)",
                               &[&opk_get.get_origin(), &opk_get.get_revision()])
                        .map_err(Error::OriginPublicKeyGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_public_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin_public_key_latest(&self,
                                        opk_get: &originsrv::OriginPublicKeyLatestGet)
                                        -> Result<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_public_key_latest_v1($1)",
                               &[&opk_get.get_origin()])
                        .map_err(Error::OriginPublicKeyLatestGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_public_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn list_origin_public_keys_for_origin(&self,
                                              opklr: &originsrv::OriginPublicKeyListRequest)
                                              -> Result<originsrv::OriginPublicKeyListResponse> {
        let conn = self.pool.get()?;
        let rows = &conn.query("SELECT * FROM get_origin_public_keys_for_origin_v1($1)",
                               &[&(opklr.get_origin_id() as i64)])
                        .map_err(Error::OriginPublicKeyListForOrigin)?;

        let mut response = originsrv::OriginPublicKeyListResponse::new();
        response.set_origin_id(opklr.get_origin_id());
        let mut keys = protobuf::RepeatedField::new();
        for row in rows {
            keys.push(self.row_to_origin_public_key(row));
        }
        response.set_keys(keys);
        Ok(response)
    }

    fn row_to_origin(&self, row: postgres::rows::Row) -> originsrv::Origin {
        let mut origin = originsrv::Origin::new();
        let oid: i64 = row.get("id");
        origin.set_id(oid as u64);
        origin.set_name(row.get("name"));
        let ooid: i64 = row.get("owner_id");
        origin.set_owner_id(ooid as u64);
        let private_key_name = row.get_opt("private_key_name");
        if let Some(Ok(pk)) = private_key_name {
            origin.set_private_key_name(pk);
        }
        origin
    }

    pub fn create_origin(&self,
                         origin: &originsrv::OriginCreate)
                         -> Result<Option<originsrv::Origin>> {
        let conn = self.pool.get()?;
        let rows = conn.query("SELECT * FROM insert_origin_v1($1, $2, $3)",
                              &[&origin.get_name(),
                                &(origin.get_owner_id() as i64),
                                &origin.get_owner_name()])
            .map_err(Error::OriginCreate)?;
        if rows.len() == 1 {
            let row = rows.iter().nth(0).expect("Insert returns row, but no row present");
            Ok(Some(self.row_to_origin(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin(&self,
                      origin_get: &originsrv::OriginGet)
                      -> Result<Option<originsrv::Origin>> {
        self.get_origin_by_name(origin_get.get_name())
    }

    pub fn get_origin_by_name(&self, origin_name: &str) -> Result<Option<originsrv::Origin>> {
        let conn = self.pool.get()?;
        let rows =
            &conn.query("SELECT * FROM origins_with_secret_key_full_name_v1 WHERE name = $1 LIMIT \
                        1",
                        &[&origin_name])
                 .map_err(Error::OriginGet)?;
        if rows.len() != 0 {
            let row = rows.iter().nth(0).unwrap();
            let mut origin = originsrv::Origin::new();
            let oid: i64 = row.get("id");
            origin.set_id(oid as u64);
            origin.set_name(row.get("name"));
            let ooid: i64 = row.get("owner_id");
            origin.set_owner_id(ooid as u64);
            let private_key_name: Option<String> = row.get("private_key_name");
            if let Some(pk) = private_key_name {
                origin.set_private_key_name(pk);
            }
            Ok(Some(origin))
        } else {
            Ok(None)
        }
    }
}
