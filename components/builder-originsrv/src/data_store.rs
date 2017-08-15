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
use db::async::{AsyncServer, EventOutcome};
use db::migration::Migrator;
use db::error::{Error as DbError, Result as DbResult};
use hab_net::routing::Broker;
use hab_core::package::PackageIdent;
use postgres::rows::Rows;
use protocol::{originsrv, sessionsrv, scheduler};
use protocol::net::NetOk;
use protocol::originsrv::Pageable;
use postgres;
use protobuf;

use config::Config;
use error::{Result, Error};
use migrations;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;


#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
    pub async: AsyncServer,
}

impl Drop for DataStore {
    fn drop(&mut self) {
        self.async.stop();
    }
}

impl DataStore {
    pub fn new(config: &Config) -> Result<DataStore> {
        let pool = Pool::new(&config.datastore, config.shards.clone())?;
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    pub fn from_pool(pool: Pool) -> Result<DataStore> {
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap),
        })
    }

    pub fn setup(&self) -> Result<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(Error::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        migrations::origins::migrate(&mut migrator)?;
        migrations::origin_public_keys::migrate(&mut migrator)?;
        migrations::origin_secret_keys::migrate(&mut migrator)?;
        migrations::origin_invitations::migrate(&mut migrator)?;
        migrations::origin_projects::migrate(&mut migrator)?;
        migrations::origin_packages::migrate(&mut migrator)?;
        migrations::origin_channels::migrate(&mut migrator)?;

        migrator.finish()?;

        self.async.register(
            "sync_invitations".to_string(),
            sync_invitations,
        );
        self.async.register(
            "sync_origins".to_string(),
            sync_origins,
        );
        self.async.register(
            "sync_packages".to_string(),
            sync_packages,
        );

        Ok(())
    }

    pub fn start_async(&self) {
        // This is an arc under the hood
        let async_thread = self.async.clone();
        async_thread.start(4);
    }

    pub fn update_origin_project(&self, opc: &originsrv::OriginProjectUpdate) -> Result<()> {
        let conn = self.pool.get(opc)?;
        let project = opc.get_project();
        conn.execute(
            "SELECT update_origin_project_v1($1, $2, $3, $4, $5, $6, $7)",
            &[
                &(project.get_id() as i64),
                &(project.get_origin_id() as i64),
                &project.get_package_name(),
                &project.get_plan_path(),
                &project.get_vcs_type(),
                &project.get_vcs_data(),
                &(project.get_owner_id() as i64),
            ],
        ).map_err(Error::OriginProjectUpdate)?;
        Ok(())
    }

    pub fn delete_origin_project_by_name(&self, name: &str) -> Result<()> {
        let mut opd = originsrv::OriginProjectDelete::new();
        opd.set_name(name.to_string());
        let conn = self.pool.get(&opd)?;
        conn.execute("SELECT delete_origin_project_v1($1)", &[&name])
            .map_err(Error::OriginProjectDelete)?;
        Ok(())
    }

    pub fn get_origin_project_by_name(
        &self,
        name: &str,
    ) -> Result<Option<originsrv::OriginProject>> {
        let mut opg = originsrv::OriginProjectGet::new();
        opg.set_name(name.to_string());
        let conn = self.pool.get(&opg)?;
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

    pub fn create_origin_project(
        &self,
        opc: &originsrv::OriginProjectCreate,
    ) -> Result<originsrv::OriginProject> {
        let conn = self.pool.get(opc)?;
        let project = opc.get_project();
        let rows = conn.query(
            "SELECT * FROM insert_origin_project_v1($1, $2, $3, $4, $5, $6)",
            &[
                &project.get_origin_name(),
                &project.get_package_name(),
                &project.get_plan_path(),
                &project.get_vcs_type(),
                &project.get_vcs_data(),
                &(project.get_owner_id() as i64),
            ],
        ).map_err(Error::OriginProjectCreate)?;
        let row = rows.get(0);
        Ok(self.row_to_origin_project(&row))
    }

    pub fn check_account_in_origin(
        &self,
        coar: &originsrv::CheckOriginAccessRequest,
    ) -> Result<bool> {
        let conn = self.pool.get(coar)?;
        let rows = &conn.query(
            "SELECT * FROM check_account_in_origin_members_v1($1, $2)",
            &[&coar.get_origin_name(), &(coar.get_account_id() as i64)],
        ).map_err(Error::OriginAccountInOrigin)?;
        if rows.len() != 0 { Ok(true) } else { Ok(false) }
    }

    pub fn list_origin_members(
        &self,
        omlr: &originsrv::OriginMemberListRequest,
    ) -> Result<originsrv::OriginMemberListResponse> {
        let conn = self.pool.get(omlr)?;
        let rows = &conn.query(
            "SELECT * FROM list_origin_members_v1($1)",
            &[&(omlr.get_origin_id() as i64)],
        ).map_err(Error::OriginMemberList)?;

        let mut response = originsrv::OriginMemberListResponse::new();
        response.set_origin_id(omlr.get_origin_id());

        let mut members = protobuf::RepeatedField::new();
        for row in rows {
            members.push(row.get("account_name"));
        }

        response.set_members(members);
        Ok(response)
    }

    // This function can fail if the corresponding sessionsrv shard is down - this is so that the
    // user won't experience delay on seeing the invitation be accepted.
    pub fn accept_origin_invitation(
        &self,
        oiar: &originsrv::OriginInvitationAcceptRequest,
    ) -> Result<()> {
        let mut bconn = Broker::connect()?;
        let mut aoia = sessionsrv::AccountOriginInvitationAcceptRequest::new();
        aoia.set_account_id(oiar.get_account_id());
        aoia.set_invite_id(oiar.get_invite_id());
        aoia.set_origin_name(oiar.get_origin_name().to_string());
        aoia.set_ignore(oiar.get_ignore());
        match bconn.route::<sessionsrv::AccountOriginInvitationAcceptRequest, NetOk>(&aoia) {
            Ok(_) => {
                debug!(
                    "Updated session service; accepted/ignored invitation, {:?}",
                    aoia
                );
            }
            Err(e) => {
                error!(
                    "Failed to update session service on invitation acceptance/ignore, {:?}, {:?}",
                    aoia,
                    e
                );
                return Err(Error::from(e));
            }
        }

        let conn = self.pool.get(oiar)?;
        let tr = conn.transaction().map_err(Error::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM accept_origin_invitation_v1($1, $2)",
            &[&(oiar.get_invite_id() as i64), &oiar.get_ignore()],
        ).map_err(Error::OriginInvitationAccept)?;
        tr.commit().map_err(Error::DbTransactionCommit)?;
        Ok(())
    }

    pub fn list_origin_invitations_for_origin(
        &self,
        oilr: &originsrv::OriginInvitationListRequest,
    ) -> Result<originsrv::OriginInvitationListResponse> {
        let conn = self.pool.get(oilr)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_invitations_for_origin_v1($1)",
            &[&(oilr.get_origin_id() as i64)],
        ).map_err(Error::OriginInvitationListForOrigin)?;

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

    pub fn create_origin_invitation(
        &self,
        oic: &originsrv::OriginInvitationCreate,
    ) -> Result<Option<originsrv::OriginInvitation>> {
        let conn = self.pool.get(oic)?;
        let rows = conn.query(
            "SELECT * FROM insert_origin_invitation_v1($1, $2, $3, $4, $5)",
            &[
                &(oic.get_origin_id() as i64),
                &oic.get_origin_name(),
                &(oic.get_account_id() as i64),
                &oic.get_account_name(),
                &(oic.get_owner_id() as i64),
            ],
        ).map_err(Error::OriginInvitationCreate)?;
        if rows.len() == 1 {
            self.async.schedule("sync_invitations")?;
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_invitation(&row)))
        } else {
            Ok(None)
        }
    }

    pub fn create_origin_secret_key(
        &self,
        osk: &originsrv::OriginSecretKeyCreate,
    ) -> Result<originsrv::OriginSecretKey> {
        let conn = self.pool.get(osk)?;
        let rows = conn.query(
            "SELECT * FROM insert_origin_secret_key_v1($1, $2, $3, $4, $5, $6)",
            &[
                &(osk.get_origin_id() as i64),
                &(osk.get_owner_id() as i64),
                &osk.get_name(),
                &osk.get_revision(),
                &format!("{}-{}", osk.get_name(), osk.get_revision()),
                &osk.get_body(),
            ],
        ).map_err(Error::OriginSecretKeyCreate)?;
        let row = rows.iter().nth(0).expect(
            "Insert returns row, but no row present",
        );
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

    pub fn get_origin_secret_key(
        &self,
        osk_get: &originsrv::OriginSecretKeyGet,
    ) -> Result<Option<originsrv::OriginSecretKey>> {
        let conn = self.pool.get(osk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_secret_key_v1($1)",
            &[&osk_get.get_origin()],
        ).map_err(Error::OriginSecretKeyGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_secret_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn create_origin_public_key(
        &self,
        opk: &originsrv::OriginPublicKeyCreate,
    ) -> Result<originsrv::OriginPublicKey> {
        let conn = self.pool.get(opk)?;
        let rows = conn.query(
            "SELECT * FROM insert_origin_public_key_v1($1, $2, $3, $4, $5, $6)",
            &[
                &(opk.get_origin_id() as i64),
                &(opk.get_owner_id() as i64),
                &opk.get_name(),
                &opk.get_revision(),
                &format!("{}-{}", opk.get_name(), opk.get_revision()),
                &opk.get_body(),
            ],
        ).map_err(Error::OriginPublicKeyCreate)?;
        let row = rows.iter().nth(0).expect(
            "Insert returns row, but no row present",
        );
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

    pub fn get_origin_public_key(
        &self,
        opk_get: &originsrv::OriginPublicKeyGet,
    ) -> Result<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get(opk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_key_v1($1, $2)",
            &[&opk_get.get_origin(), &opk_get.get_revision()],
        ).map_err(Error::OriginPublicKeyGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_public_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin_public_key_latest(
        &self,
        opk_get: &originsrv::OriginPublicKeyLatestGet,
    ) -> Result<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get(opk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_key_latest_v1($1)",
            &[&opk_get.get_origin()],
        ).map_err(Error::OriginPublicKeyLatestGet)?;
        if rows.len() != 0 {
            // We just checked - we know there is a value here
            let row = rows.iter().nth(0).unwrap();
            Ok(Some(self.row_to_origin_public_key(row)))
        } else {
            Ok(None)
        }
    }

    pub fn list_origin_public_keys_for_origin(
        &self,
        opklr: &originsrv::OriginPublicKeyListRequest,
    ) -> Result<originsrv::OriginPublicKeyListResponse> {
        let conn = self.pool.get(opklr)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_keys_for_origin_v1($1)",
            &[&(opklr.get_origin_id() as i64)],
        ).map_err(Error::OriginPublicKeyListForOrigin)?;

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

    pub fn create_origin(
        &self,
        origin: &originsrv::OriginCreate,
    ) -> Result<Option<originsrv::Origin>> {
        let conn = self.pool.get(origin)?;
        let rows = conn.query(
            "SELECT * FROM insert_origin_v1($1, $2, $3)",
            &[
                &origin.get_name(),
                &(origin.get_owner_id() as i64),
                &origin.get_owner_name(),
            ],
        ).map_err(Error::OriginCreate)?;
        if rows.len() == 1 {
            self.async.schedule("sync_origins")?;
            let row = rows.iter().nth(0).expect(
                "Insert returns row, but no row present",
            );
            Ok(Some(self.row_to_origin(row)))
        } else {
            // I don't think this will ever happen because a unique constraint violation (or any
            // other error) will trigger an error on the query and return from this function
            // before this if statement ever executes.
            Ok(None)
        }
    }

    pub fn get_origin(
        &self,
        origin_get: &originsrv::OriginGet,
    ) -> Result<Option<originsrv::Origin>> {
        self.get_origin_by_name(origin_get.get_name())
    }

    pub fn get_origin_by_name(&self, origin_name: &str) -> Result<Option<originsrv::Origin>> {
        let mut origin_get = originsrv::OriginGet::new();
        origin_get.set_name(origin_name.to_string());
        let conn = self.pool.get(&origin_get)?;
        let rows = &conn.query(
            "SELECT * FROM origins_with_secret_key_full_name_v1 WHERE name = $1 LIMIT \
                        1",
            &[&origin_name],
        ).map_err(Error::OriginGet)?;
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

    pub fn create_origin_package(
        &self,
        opc: &originsrv::OriginPackageCreate,
    ) -> Result<originsrv::OriginPackage> {
        let conn = self.pool.get(opc)?;
        let ident = opc.get_ident();
        let rows = conn.query(
            "SELECT * FROM insert_origin_package_v1($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            &[
                &(opc.get_origin_id() as i64),
                &(opc.get_owner_id() as i64),
                &ident.get_name(),
                &ident.to_string(),
                &opc.get_checksum(),
                &opc.get_manifest(),
                &opc.get_config(),
                &opc.get_target(),
                &self.into_delimited(opc.get_deps().to_vec()),
                &self.into_delimited(opc.get_tdeps().to_vec()),
                &self.into_delimited(opc.get_exposes().to_vec()),
            ],
        ).map_err(Error::OriginPackageCreate)?;

        self.async.schedule("sync_packages")?;

        let row = rows.get(0);
        Ok(self.row_to_origin_package(&row))
    }

    pub fn get_origin_package(
        &self,
        opg: &originsrv::OriginPackageGet,
    ) -> Result<Option<originsrv::OriginPackage>> {
        let conn = self.pool.get(opg)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_package_v1($1)",
            &[&opg.get_ident().to_string()],
        ).map_err(Error::OriginPackageGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_package(&row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin_channel_package(
        &self,
        ocpg: &originsrv::OriginChannelPackageGet,
    ) -> Result<Option<originsrv::OriginPackage>> {
        let conn = self.pool.get(ocpg)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_channel_package_v1($1, $2, $3)",
            &[
                &ocpg.get_ident().get_origin(),
                &ocpg.get_name(),
                &ocpg.get_ident().to_string(),
            ],
        ).map_err(Error::OriginChannelPackageGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_package(&row)))
        } else {
            Ok(None)
        }
    }

    fn rows_to_latest_ident(
        &self,
        rows: &postgres::rows::Rows,
    ) -> Result<originsrv::OriginPackageIdent> {

        let mut pkgs: Vec<PackageIdent> = Vec::new();

        for row in rows.iter() {
            let ident: String = row.get("ident");
            let pkg_ident = PackageIdent::from_str(ident.as_str()).unwrap();
            pkgs.push(pkg_ident);
        }

        // TODO: The PackageIdent compare is extremely slow, causing even small lists
        // to take significant time to sort. Look at speeding this up if it becomes a
        // bottleneck.
        pkgs.sort();
        let latest_ident = pkgs.pop().unwrap();
        let ident_str = format!("{}", latest_ident);
        Ok(
            originsrv::OriginPackageIdent::from_str(ident_str.as_str()).unwrap(),
        )
    }

    pub fn get_origin_package_latest(
        &self,
        opc: &originsrv::OriginPackageLatestGet,
    ) -> Result<Option<originsrv::OriginPackageIdent>> {
        let conn = self.pool.get(opc)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_package_latest_v2($1, $2)",
            &[&self.searchable_ident(opc.get_ident()), &opc.get_target()],
        ).map_err(Error::OriginPackageLatestGet)?;
        if rows.len() != 0 {
            let latest = self.rows_to_latest_ident(&rows).unwrap();
            Ok(Some(latest))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin_channel_package_latest(
        &self,
        ocpg: &originsrv::OriginChannelPackageLatestGet,
    ) -> Result<Option<originsrv::OriginPackageIdent>> {
        let conn = self.pool.get(ocpg)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_channel_package_latest_v2($1, $2, $3, $4)",
            &[
                &ocpg.get_ident().get_origin(),
                &ocpg.get_name(),
                &self.searchable_ident(ocpg.get_ident()),
                &ocpg.get_target(),
            ],
        ).map_err(Error::OriginChannelPackageLatestGet)?;

        if rows.len() != 0 {
            let latest = self.rows_to_latest_ident(&rows).unwrap();
            Ok(Some(latest))
        } else {
            Ok(None)
        }
    }

    pub fn list_origin_package_versions_for_origin(
        &self,
        opvl: &originsrv::OriginPackageVersionListRequest,
    ) -> Result<originsrv::OriginPackageVersionListResponse> {
        let conn = self.pool.get(opvl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_versions_for_origin_v3($1, $2)",
            &[&opvl.get_origin(), &opvl.get_name()],
        ).map_err(Error::OriginPackageVersionList)?;

        let mut response = originsrv::OriginPackageVersionListResponse::new();
        let mut idents = Vec::new();
        let mut version_map = HashMap::new();
        for row in rows.iter() {
            let ver: String = row.get("version");
            let ident =
                PackageIdent::new(opvl.get_origin(), opvl.get_name(), Some(ver.as_str()), None);

            let release_count: i64 = row.get("release_count");
            let latest: String = row.get("latest");
            let mut version = originsrv::OriginPackageVersion::new();
            version.set_origin(opvl.get_origin().to_string());
            version.set_name(opvl.get_name().to_string());
            version.set_version(ver);
            version.set_release_count(release_count as u64);
            version.set_latest(latest);

            version_map.insert(ident.clone(), version);
            idents.push(ident);
        }

        let mut versions = protobuf::RepeatedField::new();
        for ident in idents {
            versions.push(version_map.remove(&ident).unwrap());
        }
        response.set_versions(versions);
        Ok(response)
    }

    pub fn list_origin_package_channels_for_package(
        &self,
        opcl: &originsrv::OriginPackageChannelListRequest,
    ) -> Result<originsrv::OriginPackageChannelListResponse> {
        let conn = self.pool.get(opcl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_channels_for_package_v1($1)",
            &[&self.searchable_ident(opcl.get_ident())],
        ).map_err(Error::OriginPackageChannelList)?;

        let mut response = originsrv::OriginPackageChannelListResponse::new();
        let mut channels = protobuf::RepeatedField::new();
        for row in rows.iter() {
            channels.push(self.row_to_origin_channel(&row));
        }
        response.set_channels(channels);
        Ok(response)
    }

    pub fn list_origin_package_for_origin(
        &self,
        opl: &originsrv::OriginPackageListRequest,
    ) -> Result<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(opl)?;

        let query = if *&opl.get_distinct() {
            "SELECT * FROM get_origin_packages_for_origin_distinct_v1($1, $2, $3)"
        } else {
            "SELECT * FROM get_origin_packages_for_origin_v2($1, $2, $3)"
        };

        let rows = conn.query(
            query,
            &[
                &self.searchable_ident(opl.get_ident()),
                &opl.limit(),
                &(opl.get_start() as i64),
            ],
        ).map_err(Error::OriginPackageList)?;

        let mut response = originsrv::OriginPackageListResponse::new();
        response.set_start(opl.get_start());
        response.set_stop(self.last_index(opl, &rows));
        let mut idents = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let count: i64 = row.get("total_count");
            response.set_count(count as u64);
            idents.push(self.row_to_origin_package_ident(&row));
        }
        response.set_idents(idents);
        Ok(response)
    }

    fn last_index<P: Pageable>(&self, list_request: &P, rows: &Rows) -> u64 {
        if rows.len() == 0 {
            list_request.get_range()[1]
        } else {
            list_request.get_range()[0] + (rows.len() as u64) - 1
        }
    }

    fn searchable_ident(&self, ident: &originsrv::OriginPackageIdent) -> String {
        let mut search_ident = ident.to_string();

        if search_ident.split("/").count() < 4 {
            if !search_ident.ends_with("/") {
                search_ident.push('/');
            }
        }

        search_ident
    }

    pub fn list_origin_channel_package_for_channel(
        &self,
        opl: &originsrv::OriginChannelPackageListRequest,
    ) -> Result<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(opl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_channel_packages_for_channel_v1($1, $2, $3, $4, $5)",
            &[
                &opl.get_ident().get_origin(),
                &opl.get_name(),
                &self.searchable_ident(opl.get_ident()),
                &opl.limit(),
                &(opl.get_start() as i64),
            ],
        ).map_err(Error::OriginChannelPackageList)?;

        let mut response = originsrv::OriginPackageListResponse::new();
        response.set_start(opl.get_start());
        response.set_stop(self.last_index(opl, &rows));
        let mut idents = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let count: i64 = row.get("total_count");
            response.set_count(count as u64);
            idents.push(self.row_to_origin_package_ident(&row));
        }
        response.set_idents(idents);
        Ok(response)
    }

    pub fn list_origin_package_unique_for_origin(
        &self,
        opl: &originsrv::OriginPackageUniqueListRequest,
    ) -> Result<originsrv::OriginPackageUniqueListResponse> {
        let conn = self.pool.get(opl)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_packages_unique_for_origin_v1($1, $2, $3)",
            &[&opl.get_origin(), &opl.limit(), &(opl.get_start() as i64)],
        ).map_err(Error::OriginPackageUniqueList)?;

        let mut response = originsrv::OriginPackageUniqueListResponse::new();
        response.set_start(opl.get_start());
        response.set_stop(self.last_index(opl, &rows));
        let mut idents = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let count: i64 = row.get("total_count");
            response.set_count(count as u64);
            let mut ident = originsrv::OriginPackageIdent::new();
            ident.set_origin(opl.get_origin().to_string());
            ident.set_name(row.get("name"));
            idents.push(ident);
        }
        response.set_idents(idents);
        Ok(response)
    }

    pub fn search_origin_package_for_origin(
        &self,
        ops: &originsrv::OriginPackageSearchRequest,
    ) -> Result<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(ops)?;

        let rows = if *&ops.get_distinct() {
            conn.query(
                "SELECT * FROM search_all_origin_packages_dynamic_v2($1, $2, $3)",
                &[&ops.get_query(), &ops.limit(), &(ops.get_start() as i64)],
            ).map_err(Error::OriginPackageSearch)?
        } else {
            if ops.get_origin().is_empty() {
                conn.query(
                    "SELECT * FROM search_all_origin_packages_v1($1, $2, $3)",
                    &[&ops.get_query(), &ops.limit(), &(ops.get_start() as i64)],
                ).map_err(Error::OriginPackageSearch)?
            } else {
                conn.query(
                    "SELECT * FROM search_origin_packages_for_origin_v1($1, $2, $3, $4)",
                    &[
                        &ops.get_origin(),
                        &ops.get_query(),
                        &ops.limit(),
                        &(ops.get_start() as i64),
                    ],
                ).map_err(Error::OriginPackageSearch)?
            }
        };

        let mut response = originsrv::OriginPackageListResponse::new();
        response.set_start(ops.get_start());
        response.set_stop(self.last_index(ops, &rows));
        let mut idents = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let count: i64 = row.get("total_count");
            response.set_count(count as u64);
            idents.push(self.row_to_origin_package_ident(&row));
        }

        idents.sort_by(|a, b| a.cmp(b));
        response.set_idents(idents);
        Ok(response)
    }

    fn into_delimited<T: Display>(&self, parts: Vec<T>) -> String {
        let mut buffer = String::new();
        for part in parts.iter() {
            buffer.push_str(&format!("{}:", part));
        }
        buffer
    }

    fn into_idents(
        &self,
        column: String,
    ) -> protobuf::RepeatedField<originsrv::OriginPackageIdent> {
        let mut idents = protobuf::RepeatedField::new();
        for ident in column.split(":") {
            if !ident.is_empty() {
                idents.push(originsrv::OriginPackageIdent::from_str(ident).unwrap());
            }
        }
        idents
    }

    fn row_to_origin_package(&self, row: &postgres::rows::Row) -> originsrv::OriginPackage {
        let mut package = originsrv::OriginPackage::new();
        let id: i64 = row.get("id");
        package.set_id(id as u64);
        let origin_id: i64 = row.get("origin_id");
        package.set_origin_id(origin_id as u64);
        let owner_id: i64 = row.get("owner_id");
        package.set_owner_id(owner_id as u64);
        let ident: String = row.get("ident");
        package.set_ident(
            originsrv::OriginPackageIdent::from_str(ident.as_str()).unwrap(),
        );
        package.set_checksum(row.get("checksum"));
        package.set_manifest(row.get("manifest"));
        package.set_config(row.get("config"));
        package.set_target(row.get("target"));
        let expose: String = row.get("exposes");
        let mut exposes: Vec<u32> = Vec::new();
        for ex in expose.split(":") {
            match ex.parse::<u32>() {
                Ok(e) => exposes.push(e),
                Err(_) => {}
            }
        }
        package.set_exposes(exposes);
        package.set_deps(self.into_idents(row.get("deps")));
        package.set_tdeps(self.into_idents(row.get("tdeps")));
        package
    }

    fn row_to_origin_package_ident(
        &self,
        row: &postgres::rows::Row,
    ) -> originsrv::OriginPackageIdent {
        let ident: String = row.get("ident");
        originsrv::OriginPackageIdent::from_str(ident.as_str()).unwrap()
    }

    pub fn create_origin_channel(
        &self,
        occ: &originsrv::OriginChannelCreate,
    ) -> Result<originsrv::OriginChannel> {
        let conn = self.pool.get(occ)?;

        let rows = conn.query(
            "SELECT * FROM insert_origin_channel_v1($1, $2, $3)",
            &[
                &(occ.get_origin_id() as i64),
                &(occ.get_owner_id() as i64),
                &occ.get_name(),
            ],
        ).map_err(Error::OriginChannelCreate)?;
        let row = rows.iter().nth(0).expect(
            "Insert returns row, but no row present",
        );
        Ok(self.row_to_origin_channel(&row))
    }

    fn row_to_origin_channel(&self, row: &postgres::rows::Row) -> originsrv::OriginChannel {
        let mut occ = originsrv::OriginChannel::new();
        let occ_id: i64 = row.get("id");
        occ.set_id(occ_id as u64);
        let occ_origin_id: i64 = row.get("origin_id");
        occ.set_origin_id(occ_origin_id as u64);
        occ.set_name(row.get("name"));
        let occ_owner_id: i64 = row.get("owner_id");
        occ.set_owner_id(occ_owner_id as u64);
        occ
    }

    pub fn list_origin_channels(
        &self,
        oclr: &originsrv::OriginChannelListRequest,
    ) -> Result<originsrv::OriginChannelListResponse> {
        let conn = self.pool.get(oclr)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_channels_for_origin_v1($1)",
            &[&(oclr.get_origin_id() as i64)],
        ).map_err(Error::OriginChannelList)?;

        let mut response = originsrv::OriginChannelListResponse::new();
        response.set_origin_id(oclr.get_origin_id());

        let mut channels = protobuf::RepeatedField::new();
        for row in rows {
            channels.push(self.row_to_origin_channel(&row))
        }

        response.set_channels(channels);
        Ok(response)
    }

    pub fn get_origin_channel(
        &self,
        ocg: &originsrv::OriginChannelGet,
    ) -> Result<Option<originsrv::OriginChannel>> {
        let conn = self.pool.get(ocg)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_channel_v1($1, $2)",
            &[&ocg.get_origin_name(), &ocg.get_name()],
        ).map_err(Error::OriginChannelGet)?;

        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_origin_channel(&row)))
        } else {
            Ok(None)
        }
    }

    pub fn promote_origin_package_group(
        &self,
        opp: &originsrv::OriginPackageGroupPromote,
    ) -> Result<()> {
        let conn = self.pool.get(opp)?;
        let pkg_ids: Vec<i64> = opp.get_package_ids()
            .to_vec()
            .iter()
            .map(|&x| x as i64)
            .collect();

        &conn.query(
            "SELECT * FROM promote_origin_package_group_v1($1, $2)",
            &[&(opp.get_channel_id() as i64), &(pkg_ids)],
        ).map_err(Error::OriginPackageGroupPromote)?;

        Ok(())
    }

    pub fn promote_origin_package(&self, opp: &originsrv::OriginPackagePromote) -> Result<()> {
        let conn = self.pool.get(opp)?;
        &conn.query(
            "SELECT * FROM promote_origin_package_v1($1, $2)",
            &[
                &(opp.get_channel_id() as i64),
                &(opp.get_package_id() as i64),
            ],
        ).map_err(Error::OriginPackagePromote)?;

        Ok(())
    }

    pub fn demote_origin_package(&self, opp: &originsrv::OriginPackageDemote) -> Result<()> {
        let conn = self.pool.get(opp)?;
        &conn.query(
            "SELECT * FROM demote_origin_package_v1($1, $2)",
            &[
                &(opp.get_channel_id() as i64),
                &(opp.get_package_id() as i64),
            ],
        ).map_err(Error::OriginPackageDemote)?;

        Ok(())
    }

    pub fn delete_origin_channel_by_id(&self, ocd: &originsrv::OriginChannelDelete) -> Result<()> {
        let conn = self.pool.get(ocd)?;
        conn.execute(
            "SELECT delete_origin_channel_v1($1)",
            &[&(ocd.get_id() as i64)],
        ).map_err(Error::OriginChannelDelete)?;
        Ok(())
    }
}

fn sync_origins(pool: Pool) -> DbResult<EventOutcome> {
    error!("I like my butt");
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_origins_v1()", &[]).map_err(
            DbError::AsyncFunctionCheck,
        )?;
        if rows.len() > 0 {
            let mut bconn = Broker::connect()?;
            let mut request = sessionsrv::AccountOriginCreate::new();
            for row in rows.iter() {
                let aid: i64 = row.get("account_id");
                let oid: i64 = row.get("origin_id");
                request.set_account_id(aid as u64);
                request.set_account_name(row.get("account_name"));
                request.set_origin_id(oid as u64);
                request.set_origin_name(row.get("origin_name"));
                match bconn.route::<sessionsrv::AccountOriginCreate, NetOk>(&request) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_session_sync_v1($1)", &[&oid])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!(
                            "Updated session service with origin creation, {:?}",
                            request
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync origin creation with the session service, {:?}: {}",
                            request,
                            e
                        );
                        result = EventOutcome::Retry;
                    }
                }
            }
        }
    }
    Ok(result)
}

fn sync_packages(pool: Pool) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_packages_v1()", &[])
            .map_err(DbError::AsyncFunctionCheck)?;
        if rows.len() > 0 {
            let mut bconn = Broker::connect()?;
            let mut request = scheduler::PackageCreate::new();
            for row in rows.iter() {
                let pid: i64 = row.get("package_id");
                let ident: String = row.get("package_ident");
                let deps_column: String = row.get("package_deps");

                let mut deps = protobuf::RepeatedField::new();
                for ident in deps_column.split(":") {
                    if !ident.is_empty() {
                        let opi = originsrv::OriginPackageIdent::from_str(ident).unwrap();
                        let dep_str = format!("{}", opi);
                        deps.push(dep_str);
                    }
                }
                request.set_ident(ident);
                request.set_deps(deps);

                match bconn.route::<scheduler::PackageCreate, NetOk>(&request) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_packages_sync_v1($1)", &[&pid])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!(
                            "Updated scheduler service with package creation, {:?}",
                            request
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync package creation with the scheduler service, {:?}: {}",
                            request,
                            e
                        );
                        result = EventOutcome::Retry;
                    }
                }
            }
        }
    }
    Ok(result)
}

fn sync_invitations(pool: Pool) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_invitations_not_synced_with_account_v1()",
            &[],
        ).map_err(DbError::AsyncFunctionCheck)?;
        if rows.len() > 0 {
            let mut bconn = Broker::connect()?;
            for row in rows.iter() {
                let mut aoic = sessionsrv::AccountOriginInvitationCreate::new();
                let aid: i64 = row.get("account_id");
                aoic.set_account_id(aid as u64);
                let oid: i64 = row.get("origin_id");
                aoic.set_origin_id(oid as u64);
                let oiid: i64 = row.get("id");
                aoic.set_origin_invitation_id(oiid as u64);
                let owner_id: i64 = row.get("owner_id");
                aoic.set_owner_id(owner_id as u64);
                aoic.set_account_name(row.get("account_name"));
                aoic.set_origin_name(row.get("origin_name"));
                match bconn.route::<sessionsrv::AccountOriginInvitationCreate, NetOk>(&aoic) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_account_sync_v1($1)", &[&oiid])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!("Updated session service with origin invitation, {:?}", aoic);
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync invitation with the session service, {:?}: {}",
                            aoic,
                            e
                        );
                        result = EventOutcome::Retry;
                    }
                }
            }
        }
    }
    Ok(result)
}
