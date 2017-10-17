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

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;

use bldr_core::helpers::transition_visibility;
use db::async::{AsyncServer, EventOutcome};
use db::config::{DataStoreCfg, ShardId};
use db::error::{Error as DbError, Result as DbResult};
use db::migration::Migrator;
use db::pool::Pool;
use hab_net::conn::{RouteClient, RouteConn};
use hab_net::{ErrCode, NetError};
use hab_core::package::PackageIdent;
use postgres::rows::Rows;
use protocol::{originsrv, sessionsrv, jobsrv};
use protocol::net::NetOk;
use protocol::originsrv::Pageable;
use postgres;
use protobuf;

use error::{SrvError, SrvResult};
use migrations;

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
    pub fn new(
        cfg: &DataStoreCfg,
        shards: Vec<ShardId>,
        router_pipe: Arc<String>,
    ) -> SrvResult<DataStore> {
        let pool = Pool::new(&cfg, shards)?;
        let ap = pool.clone();
        Ok(DataStore {
            pool: pool,
            async: AsyncServer::new(ap, router_pipe),
        })
    }

    pub fn from_pool(pool: Pool, router_pipe: Arc<String>) -> SrvResult<DataStore> {
        Ok(DataStore {
            async: AsyncServer::new(pool.clone(), router_pipe),
            pool: pool,
        })
    }

    pub fn setup(&self) -> SrvResult<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        migrations::origins::migrate(&mut migrator)?;
        migrations::origin_public_keys::migrate(&mut migrator)?;
        migrations::origin_secret_keys::migrate(&mut migrator)?;
        migrations::origin_invitations::migrate(&mut migrator)?;
        migrations::origin_projects::migrate(&mut migrator)?;
        migrations::origin_packages::migrate(&mut migrator)?;
        migrations::origin_channels::migrate(&mut migrator)?;
        migrations::origin_integrations::migrate(&mut migrator)?;

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
        let async_thread = self.async.clone();
        async_thread.start(4);
    }

    pub fn update_origin_package(&self, opu: &originsrv::OriginPackageUpdate) -> SrvResult<()> {
        let conn = self.pool.get(opu)?;
        let pkg = opu.get_pkg();
        let ident = pkg.get_ident();

        conn.execute(
            "SELECT update_origin_package_v1($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[
                &(pkg.get_id() as i64),
                &(pkg.get_owner_id() as i64),
                &ident.get_name(),
                &ident.to_string(),
                &pkg.get_checksum(),
                &pkg.get_manifest(),
                &pkg.get_config(),
                &pkg.get_target(),
                &self.into_delimited(pkg.get_deps().to_vec()),
                &self.into_delimited(pkg.get_tdeps().to_vec()),
                &self.into_delimited(pkg.get_exposes().to_vec()),
                &pkg.get_visibility().to_string(),
            ],
        ).map_err(SrvError::OriginPackageUpdate)?;
        self.async.schedule("sync_packages")?;
        Ok(())
    }

    pub fn update_origin_project(&self, opc: &originsrv::OriginProjectUpdate) -> SrvResult<()> {
        let conn = self.pool.get(opc)?;
        let project = opc.get_project();

        conn.execute(
            "SELECT update_origin_project_v3($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[
                &(project.get_id() as i64),
                &(project.get_origin_id() as i64),
                &project.get_package_name(),
                &project.get_plan_path(),
                &project.get_vcs_type(),
                &project.get_vcs_data(),
                &(project.get_owner_id() as i64),
                &(project.get_vcs_installation_id() as i64),
                &project.get_visibility().to_string(),
            ],
        ).map_err(SrvError::OriginProjectUpdate)?;

        // Get all the packages tied to this project
        let rows = conn.query(
            "SELECT * FROM get_all_origin_packages_for_ident_v1($1)",
            &[&project.get_name()],
        ).map_err(SrvError::VisibilityCascade)?;

        let mut map = HashMap::new();

        // For each row, store its id in our map, keyed on visibility
        for row in rows.iter() {
            let id: i64 = row.get("id");
            let pv: String = row.get("visibility");
            let vis: originsrv::OriginPackageVisibility =
                pv.parse().map_err(SrvError::UnknownOriginPackageVisibility)?;
            let new_vis = transition_visibility(project.get_visibility(), vis);
            map.entry(new_vis).or_insert(Vec::new()).push(id);
        }

        // Now do a bulk update for each different visibility
        for (vis, id_vector) in map.iter() {
            let vis_str = vis.to_string();
            conn.execute(
                "SELECT update_package_visibility_in_bulk_v1($1, $2)",
                &[&vis_str, id_vector],
            ).map_err(SrvError::VisibilityCascade)?;
        }
        Ok(())
    }

    pub fn delete_origin_project_by_name(&self, name: &str) -> SrvResult<()> {
        let mut opd = originsrv::OriginProjectDelete::new();
        opd.set_name(name.to_string());
        let conn = self.pool.get(&opd)?;
        conn.execute("SELECT delete_origin_project_v1($1)", &[&name])
            .map_err(SrvError::OriginProjectDelete)?;
        Ok(())
    }

    pub fn get_origin_project_by_name(
        &self,
        name: &str,
    ) -> SrvResult<Option<originsrv::OriginProject>> {
        let mut opg = originsrv::OriginProjectGet::new();
        opg.set_name(name.to_string());
        let conn = self.pool.get(&opg)?;
        let rows = &conn.query("SELECT * FROM get_origin_project_v1($1)", &[&name])
            .map_err(SrvError::OriginProjectGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let project = self.row_to_origin_project(&row)?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub fn row_to_origin_project(
        &self,
        row: &postgres::rows::Row,
    ) -> SrvResult<originsrv::OriginProject> {
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

        if let Some(Ok(install_id)) = row.get_opt::<&str, i64>("vcs_installation_id") {
            project.set_vcs_installation_id(install_id as u32);
        }

        let pv: String = row.get("visibility");
        let pv2: originsrv::OriginPackageVisibility =
            pv.parse().map_err(SrvError::UnknownOriginPackageVisibility)?;
        project.set_visibility(pv2);

        Ok(project)
    }

    pub fn create_origin_project(
        &self,
        opc: &originsrv::OriginProjectCreate,
    ) -> SrvResult<originsrv::OriginProject> {
        let conn = self.pool.get(opc)?;
        let project = opc.get_project();
        let install_id: Option<i64> = {
            if project.has_vcs_installation_id() {
                Some(project.get_vcs_installation_id() as i64)
            } else {
                None
            }
        };
        let rows = conn.query(
            "SELECT * FROM insert_origin_project_v4($1, $2, $3, $4, $5, $6, $7, $8)",
            &[
                &project.get_origin_name(),
                &project.get_package_name(),
                &project.get_plan_path(),
                &project.get_vcs_type(),
                &project.get_vcs_data(),
                &(project.get_owner_id() as i64),
                &install_id,
                &project.get_visibility().to_string(),
            ],
        ).map_err(SrvError::OriginProjectCreate)?;
        let row = rows.get(0);
        let project = self.row_to_origin_project(&row)?;
        Ok(project)
    }

    pub fn get_origin_project_list(
        &self,
        opl: &originsrv::OriginProjectListGet,
    ) -> SrvResult<originsrv::OriginProjectList> {
        let conn = self.pool.get(opl)?;
        let origin = opl.get_origin();

        let rows = conn.query("SELECT * FROM get_origin_project_list_v1($1)", &[&origin])
            .map_err(SrvError::OriginProjectListGet)?;

        let mut response = originsrv::OriginProjectList::new();
        let mut projects = protobuf::RepeatedField::new();
        for row in rows.iter() {
            projects.push(row.get("package_name"));
        }

        response.set_names(projects);
        Ok(response)
    }

    pub fn create_project_integration(
        &self,
        opic: &originsrv::OriginProjectIntegrationCreate,
    ) -> SrvResult<()> {
        let conn = self.pool.get(opic)?;

        let rows = conn.query(
            "SELECT * FROM upsert_origin_project_integration_v1($1, $2, $3, $4, $5)",
            &[
                &opic.get_integration().get_origin(),
                &opic.get_integration().get_name(),
                &opic.get_integration().get_integration(),
                &opic.get_integration().get_integration_name(),
                &opic.get_integration().get_body(),
            ],
        ).map_err(SrvError::OriginProjectIntegrationCreate)?;
        rows.iter().nth(0).expect(
            "Insert returns row, but no row present",
        );
        Ok(())
    }

    pub fn get_project_integration(
        &self,
        opig: &originsrv::OriginProjectIntegrationGet,
    ) -> SrvResult<Option<originsrv::OriginProjectIntegration>> {
        let conn = self.pool.get(opig)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_project_integrations_v1($1, $2, $3, $4)",
            &[
                &opig.get_integration().get_origin(),
                &opig.get_integration().get_name(),
                &opig.get_integration().get_integration(),
                &opig.get_integration().get_integration_name(),
            ],
        ).map_err(SrvError::OriginProjectIntegrationGet)?;

        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_project_integration(&row)))
        } else {
            Ok(None)
        }
    }

    fn row_to_project_integration(
        &self,
        row: &postgres::rows::Row,
    ) -> originsrv::OriginProjectIntegration {
        let mut opi = originsrv::OriginProjectIntegration::new();
        opi.set_origin(row.get("origin"));
        opi.set_name(row.get("name"));
        opi.set_integration(row.get("integration"));
        opi.set_integration_name(row.get("integration_name"));
        opi.set_body(row.get("body"));
        opi
    }

    pub fn origin_project_integration_request(
        &self,
        opir: &originsrv::OriginProjectIntegrationRequest,
    ) -> SrvResult<originsrv::OriginProjectIntegrationResponse> {
        let conn = self.pool.get(opir)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_project_integrations_for_project_v1($1, $2)",
            &[&opir.get_origin(), &opir.get_name()],
        ).map_err(SrvError::OriginProjectIntegrationRequest)?;

        let mut response = originsrv::OriginProjectIntegrationResponse::new();
        let mut integrations = protobuf::RepeatedField::new();

        for row in rows {
            integrations.push(self.row_to_project_integration(&row));
        }

        response.set_integrations(integrations);
        Ok(response)
    }

    pub fn check_account_in_origin(
        &self,
        coar: &originsrv::CheckOriginAccessRequest,
    ) -> SrvResult<bool> {
        let conn = self.pool.get(coar)?;
        let rows = &conn.query(
            "SELECT * FROM check_account_in_origin_members_v1($1, $2)",
            &[&coar.get_origin_name(), &(coar.get_account_id() as i64)],
        ).map_err(SrvError::OriginAccountInOrigin)?;
        if rows.len() != 0 { Ok(true) } else { Ok(false) }
    }

    pub fn my_origins(
        &self,
        mor: &originsrv::MyOriginsRequest,
    ) -> SrvResult<originsrv::MyOriginsResponse> {
        let conn = self.pool.get(mor)?;
        let rows = &conn.query(
            "SELECT * FROM my_origins_v1($1) ORDER BY name",
            &[&(mor.get_account_id() as i64)],
        ).map_err(SrvError::MyOrigins)?;

        let mut response = originsrv::MyOriginsResponse::new();

        let mut origins = protobuf::RepeatedField::new();
        for row in rows {
            let o = self.row_to_origin(row)?;
            origins.push(o);
        }

        response.set_origins(origins);
        Ok(response)
    }

    pub fn list_origin_members(
        &self,
        omlr: &originsrv::OriginMemberListRequest,
    ) -> SrvResult<originsrv::OriginMemberListResponse> {
        let conn = self.pool.get(omlr)?;
        let rows = &conn.query(
            "SELECT * FROM list_origin_members_v1($1)",
            &[&(omlr.get_origin_id() as i64)],
        ).map_err(SrvError::OriginMemberList)?;

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
        conn: &mut RouteConn,
        oiar: &originsrv::OriginInvitationAcceptRequest,
    ) -> SrvResult<()> {
        let mut aoia = sessionsrv::AccountOriginInvitationAcceptRequest::new();
        aoia.set_account_id(oiar.get_account_id());
        aoia.set_invite_id(oiar.get_invite_id());
        aoia.set_origin_name(oiar.get_origin_name().to_string());
        aoia.set_ignore(oiar.get_ignore());
        match conn.route::<sessionsrv::AccountOriginInvitationAcceptRequest, NetOk>(&aoia) {
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
                return Err(SrvError::from(e));
            }
        }

        let conn = self.pool.get(oiar)?;
        let tr = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM accept_origin_invitation_v1($1, $2)",
            &[&(oiar.get_invite_id() as i64), &oiar.get_ignore()],
        ).map_err(SrvError::OriginInvitationAccept)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn ignore_origin_invitation(
        &self,
        conn: &mut RouteConn,
        oiir: &originsrv::OriginInvitationIgnoreRequest,
    ) -> SrvResult<()> {
        let mut aoiir = sessionsrv::AccountOriginInvitationIgnoreRequest::new();
        aoiir.set_account_id(oiir.get_account_id());
        aoiir.set_invitation_id(oiir.get_invitation_id());

        match conn.route::<sessionsrv::AccountOriginInvitationIgnoreRequest, NetOk>(&aoiir) {
            Ok(_) => {
                debug!("Updated session service; ignored invitation, {:?}", aoiir);
            }
            Err(e) => {
                error!(
                    "Failed to update session service on invitation ignore, {:?}, {:?}",
                    aoiir,
                    e
                );
                return Err(SrvError::from(e));
            }
        }

        let conn = self.pool.get(oiir)?;
        let tr = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM ignore_origin_invitation_v1($1, $2)",
            &[
                &(oiir.get_invitation_id() as i64),
                &(oiir.get_account_id() as i64),
            ],
        ).map_err(SrvError::OriginInvitationIgnore)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn rescind_origin_invitation(
        &self,
        conn: &mut RouteConn,
        oirr: &originsrv::OriginInvitationRescindRequest,
    ) -> SrvResult<()> {
        let pconn = self.pool.get(oirr)?;
        let mut aoirr = sessionsrv::AccountOriginInvitationRescindRequest::new();

        let rows = &pconn
            .query(
                "SELECT * FROM get_origin_invitation_v1($1)",
                &[&(oirr.get_invitation_id() as i64)],
            )
            .map_err(SrvError::OriginInvitationGet)?;

        if rows.len() == 1 {
            let row = rows.get(0);
            let account_id: i64 = row.get("account_id");
            aoirr.set_account_id(account_id as u64);
        } else {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "od:rescind-origin-invitation:0");
            return Err(SrvError::NetError(err));
        }

        aoirr.set_invitation_id(oirr.get_invitation_id());

        match conn.route::<sessionsrv::AccountOriginInvitationRescindRequest, NetOk>(&aoirr) {
            Ok(_) => {
                debug!("Updated session service; rescinded invitation, {:?}", aoirr);
            }
            Err(e) => {
                error!(
                    "Failed to update session service on invitation rescind, {:?}, {:?}",
                    aoirr,
                    e
                );
                return Err(SrvError::from(e));
            }
        }

        let tr = pconn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM rescind_origin_invitation_v1($1, $2)",
            &[
                &(oirr.get_invitation_id() as i64),
                &(oirr.get_owner_id() as i64),
            ],
        ).map_err(SrvError::OriginInvitationRescind)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn list_origin_invitations_for_origin(
        &self,
        oilr: &originsrv::OriginInvitationListRequest,
    ) -> SrvResult<originsrv::OriginInvitationListResponse> {
        let conn = self.pool.get(oilr)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_invitations_for_origin_v1($1)",
            &[&(oilr.get_origin_id() as i64)],
        ).map_err(SrvError::OriginInvitationListForOrigin)?;

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
    ) -> SrvResult<Option<originsrv::OriginInvitation>> {
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
        ).map_err(SrvError::OriginInvitationCreate)?;
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
    ) -> SrvResult<originsrv::OriginSecretKey> {
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
        ).map_err(SrvError::OriginSecretKeyCreate)?;
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
    ) -> SrvResult<Option<originsrv::OriginSecretKey>> {
        let conn = self.pool.get(osk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_secret_key_v1($1)",
            &[&osk_get.get_origin()],
        ).map_err(SrvError::OriginSecretKeyGet)?;
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
    ) -> SrvResult<originsrv::OriginPublicKey> {
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
        ).map_err(SrvError::OriginPublicKeyCreate)?;
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
    ) -> SrvResult<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get(opk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_key_v1($1, $2)",
            &[&opk_get.get_origin(), &opk_get.get_revision()],
        ).map_err(SrvError::OriginPublicKeyGet)?;
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
    ) -> SrvResult<Option<originsrv::OriginPublicKey>> {
        let conn = self.pool.get(opk_get)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_key_latest_v1($1)",
            &[&opk_get.get_origin()],
        ).map_err(SrvError::OriginPublicKeyLatestGet)?;
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
    ) -> SrvResult<originsrv::OriginPublicKeyListResponse> {
        let conn = self.pool.get(opklr)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_public_keys_for_origin_v1($1)",
            &[&(opklr.get_origin_id() as i64)],
        ).map_err(SrvError::OriginPublicKeyListForOrigin)?;

        let mut response = originsrv::OriginPublicKeyListResponse::new();
        response.set_origin_id(opklr.get_origin_id());
        let mut keys = protobuf::RepeatedField::new();
        for row in rows {
            keys.push(self.row_to_origin_public_key(row));
        }
        response.set_keys(keys);
        Ok(response)
    }

    fn row_to_origin(&self, row: postgres::rows::Row) -> SrvResult<originsrv::Origin> {
        let mut origin = originsrv::Origin::new();
        let oid: i64 = row.get("id");
        origin.set_id(oid as u64);
        origin.set_name(row.get("name"));

        let dpv: String = row.get("default_package_visibility");
        let new_dpv: originsrv::OriginPackageVisibility = dpv.parse().map_err(
            SrvError::UnknownOriginPackageVisibility,
        )?;
        origin.set_default_package_visibility(new_dpv);
        let ooid: i64 = row.get("owner_id");
        origin.set_owner_id(ooid as u64);
        let private_key_name = row.get_opt("private_key_name");
        if let Some(Ok(pk)) = private_key_name {
            origin.set_private_key_name(pk);
        }
        Ok(origin)
    }

    pub fn create_origin(
        &self,
        origin: &originsrv::OriginCreate,
    ) -> SrvResult<Option<originsrv::Origin>> {
        let conn = self.pool.get(origin)?;
        let mut dpv = origin.get_default_package_visibility().to_string();

        if dpv.is_empty() {
            dpv = originsrv::OriginPackageVisibility::default().to_string();
        }

        let rows = conn.query(
            "SELECT * FROM insert_origin_v2($1, $2, $3, $4)",
            &[
                &origin.get_name(),
                &(origin.get_owner_id() as i64),
                &origin.get_owner_name(),
                &dpv,
            ],
        ).map_err(SrvError::OriginCreate)?;
        if rows.len() == 1 {
            self.async.schedule("sync_origins")?;
            let row = rows.iter().nth(0).expect(
                "Insert returns row, but no row present",
            );
            let o = self.row_to_origin(row)?;
            Ok(Some(o))
        } else {
            // I don't think this will ever happen because a unique constraint violation (or any
            // other error) will trigger an error on the query and return from this function
            // before this if statement ever executes.
            Ok(None)
        }
    }

    pub fn update_origin(&self, ou: &originsrv::OriginUpdate) -> SrvResult<()> {
        let conn = self.pool.get(ou)?;
        let dpv = ou.get_default_package_visibility().to_string();

        conn.execute(
            "SELECT update_origin_v1($1, $2)",
            &[&(ou.get_id() as i64), &dpv],
        ).map_err(SrvError::OriginUpdate)?;
        Ok(())
    }

    pub fn get_origin(
        &self,
        origin_get: &originsrv::OriginGet,
    ) -> SrvResult<Option<originsrv::Origin>> {
        self.get_origin_by_name(origin_get.get_name())
    }

    pub fn get_origin_by_name(&self, origin_name: &str) -> SrvResult<Option<originsrv::Origin>> {
        let mut origin_get = originsrv::OriginGet::new();
        origin_get.set_name(origin_name.to_string());
        let conn = self.pool.get(&origin_get)?;
        let rows = &conn.query(
            "SELECT * FROM origins_with_secret_key_full_name_v2 WHERE name = $1 LIMIT \
                        1",
            &[&origin_name],
        ).map_err(SrvError::OriginGet)?;
        if rows.len() != 0 {
            let row = rows.iter().nth(0).unwrap();
            let mut origin = originsrv::Origin::new();
            let oid: i64 = row.get("id");
            origin.set_id(oid as u64);
            origin.set_name(row.get("name"));
            let ooid: i64 = row.get("owner_id");
            let dpv: String = row.get("default_package_visibility");
            let new_dpv: originsrv::OriginPackageVisibility = dpv.parse().map_err(
                SrvError::UnknownOriginPackageVisibility,
            )?;
            origin.set_default_package_visibility(new_dpv);
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
    ) -> SrvResult<originsrv::OriginPackage> {
        let conn = self.pool.get(opc)?;
        let ident = opc.get_ident();

        let rows = conn.query(
            "SELECT * FROM insert_origin_package_v3($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
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
                &opc.get_visibility().to_string()
            ],
        ).map_err(SrvError::OriginPackageCreate)?;

        self.async.schedule("sync_packages")?;

        let row = rows.get(0);
        self.row_to_origin_package(&row)
    }

    pub fn get_origin_package(
        &self,
        opg: &originsrv::OriginPackageGet,
    ) -> SrvResult<Option<originsrv::OriginPackage>> {
        let conn = self.pool.get(opg)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_v4($1, $2)",
            &[
                &opg.get_ident().to_string(),
                &self.vec_to_delimited_string(opg.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageGet)?;

        if rows.len() != 0 {
            let row = rows.get(0);
            let pkg = self.row_to_origin_package(&row)?;
            Ok(Some(pkg))
        } else {
            Ok(None)
        }
    }

    pub fn get_origin_channel_package(
        &self,
        ocpg: &originsrv::OriginChannelPackageGet,
    ) -> SrvResult<Option<originsrv::OriginPackage>> {
        let conn = self.pool.get(ocpg)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_channel_package_v4($1, $2, $3, $4)",
            &[
                &ocpg.get_ident().get_origin(),
                &ocpg.get_name(),
                &ocpg.get_ident().to_string(),
                &self.vec_to_delimited_string(ocpg.get_visibilities()),
            ],
        ).map_err(SrvError::OriginChannelPackageGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            let pkg = self.row_to_origin_package(&row)?;
            Ok(Some(pkg))
        } else {
            Ok(None)
        }
    }

    fn rows_to_latest_ident(
        &self,
        rows: &postgres::rows::Rows,
    ) -> SrvResult<originsrv::OriginPackageIdent> {

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
    ) -> SrvResult<Option<originsrv::OriginPackageIdent>> {
        let conn = self.pool.get(opc)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_package_latest_v5($1, $2, $3)",
            &[
                &self.searchable_ident(opc.get_ident()),
                &opc.get_target(),
                &self.vec_to_delimited_string(opc.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageLatestGet)?;
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
    ) -> SrvResult<Option<originsrv::OriginPackageIdent>> {
        let conn = self.pool.get(ocpg)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_channel_package_latest_v5($1, $2, $3, $4, $5)",
            &[
                &ocpg.get_ident().get_origin(),
                &ocpg.get_name(),
                &self.searchable_ident(ocpg.get_ident()),
                &ocpg.get_target(),
                &self.vec_to_delimited_string(ocpg.get_visibilities()),
            ],
        ).map_err(SrvError::OriginChannelPackageLatestGet)?;

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
    ) -> SrvResult<originsrv::OriginPackageVersionListResponse> {
        let conn = self.pool.get(opvl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_versions_for_origin_v7($1, $2, $3)",
            &[
                &opvl.get_origin(),
                &opvl.get_name(),
                &self.vec_to_delimited_string(opvl.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageVersionList)?;

        let mut response = originsrv::OriginPackageVersionListResponse::new();
        let mut idents = Vec::new();
        let mut version_map = HashMap::new();
        for row in rows.iter() {
            let ver: String = row.get("version");
            let ident =
                PackageIdent::new(opvl.get_origin(), opvl.get_name(), Some(ver.as_str()), None);

            let release_count: i64 = row.get("release_count");
            let latest: String = row.get("latest");
            let platforms_str: String = row.get("platforms");
            let platforms_vec = platforms_str.split(',').map(|x| x.to_string()).collect();
            let platforms = protobuf::RepeatedField::from_vec(platforms_vec);

            let mut version = originsrv::OriginPackageVersion::new();
            version.set_origin(opvl.get_origin().to_string());
            version.set_name(opvl.get_name().to_string());
            version.set_version(ver);
            version.set_release_count(release_count as u64);
            version.set_latest(latest);
            version.set_platforms(platforms);

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

    pub fn list_origin_package_platforms_for_package(
        &self,
        oppl: &originsrv::OriginPackagePlatformListRequest,
    ) -> SrvResult<originsrv::OriginPackagePlatformListResponse> {
        let conn = self.pool.get(oppl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_platforms_for_package_v4($1, $2)",
            &[
                &self.searchable_ident(oppl.get_ident()),
                &self.vec_to_delimited_string(oppl.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackagePlatformList)?;

        let mut response = originsrv::OriginPackagePlatformListResponse::new();
        let mut platforms = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let platform = row.get("target");
            platforms.push(platform);
        }
        response.set_platforms(platforms);
        Ok(response)
    }

    pub fn list_origin_package_channels_for_package(
        &self,
        opcl: &originsrv::OriginPackageChannelListRequest,
    ) -> SrvResult<Option<originsrv::OriginPackageChannelListResponse>> {
        let conn = self.pool.get(opcl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_package_channels_for_package_v4($1, $2)",
            &[
                &self.searchable_ident(opcl.get_ident()),
                &self.vec_to_delimited_string(opcl.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageChannelList)?;

        // If there are no rows returned, we know either the ident they passed doesn't exist or it
        // does exist but it's a private package and they don't have access to see it (since every
        // package should be in at least "unstable")
        if rows.len() == 0 {
            return Ok(None);
        }

        let mut response = originsrv::OriginPackageChannelListResponse::new();
        let mut channels = protobuf::RepeatedField::new();
        for row in rows.iter() {
            channels.push(self.row_to_origin_channel(&row));
        }
        response.set_channels(channels);
        Ok(Some(response))
    }

    pub fn list_origin_package_for_origin(
        &self,
        opl: &originsrv::OriginPackageListRequest,
    ) -> SrvResult<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(opl)?;

        let query = if *&opl.get_distinct() {
            "SELECT * FROM get_origin_packages_for_origin_distinct_v4($1, $2, $3, $4)"
        } else {
            "SELECT * FROM get_origin_packages_for_origin_v5($1, $2, $3, $4)"
        };

        let rows = conn.query(
            query,
            &[
                &self.searchable_ident(opl.get_ident()),
                &opl.limit(),
                &(opl.get_start() as i64),
                &self.vec_to_delimited_string(opl.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageList)?;

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
    ) -> SrvResult<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(opl)?;

        let rows = conn.query(
            "SELECT * FROM get_origin_channel_packages_for_channel_v3($1, $2, $3, $4, $5, $6)",
            &[
                &opl.get_ident().get_origin(),
                &opl.get_name(),
                &self.searchable_ident(opl.get_ident()),
                &self.vec_to_delimited_string(opl.get_visibilities()),
                &opl.limit(),
                &(opl.get_start() as i64),
            ],
        ).map_err(SrvError::OriginChannelPackageList)?;

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
    ) -> SrvResult<originsrv::OriginPackageUniqueListResponse> {
        let conn = self.pool.get(opl)?;
        let rows = conn.query(
            "SELECT * FROM get_origin_packages_unique_for_origin_v4($1, $2, $3, $4)",
            &[
                &opl.get_origin(),
                &opl.limit(),
                &(opl.get_start() as i64),
                &self.vec_to_delimited_string(opl.get_visibilities()),
            ],
        ).map_err(SrvError::OriginPackageUniqueList)?;

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
    ) -> SrvResult<originsrv::OriginPackageListResponse> {
        let conn = self.pool.get(ops)?;

        let rows = if *&ops.get_distinct() {
            conn.query(
                "SELECT COUNT(*) OVER () AS the_real_total, * FROM search_all_origin_packages_dynamic_v6($1, $2) ORDER BY ident LIMIT $3 OFFSET $4",
                &[
                    &ops.get_query(),
                    &self.vec_to_delimited_string(ops.get_my_origins()),
                    &ops.limit(),
                    &(ops.get_start() as i64),
                ],
            ).map_err(SrvError::OriginPackageSearch)?
        } else {
            if ops.get_origin().is_empty() {
                conn.query(
                    "SELECT COUNT(*) OVER () AS the_real_total, * FROM search_all_origin_packages_v5($1, $2) ORDER BY ident LIMIT $3 OFFSET $4",
                    &[
                        &ops.get_query(),
                        &self.vec_to_delimited_string(ops.get_my_origins()),
                        &ops.limit(),
                        &(ops.get_start() as i64),
                    ],
                ).map_err(SrvError::OriginPackageSearch)?
            } else {
                conn.query(
                    "SELECT COUNT(*) OVER () AS the_real_total, * FROM search_origin_packages_for_origin_v4($1, $2, $3, $4, $5)",
                    &[
                        &ops.get_origin(),
                        &ops.get_query(),
                        &ops.limit(),
                        &(ops.get_start() as i64),
                        &self.vec_to_delimited_string(ops.get_my_origins())
                    ],
                ).map_err(SrvError::OriginPackageSearch)?
            }
        };

        let mut response = originsrv::OriginPackageListResponse::new();
        response.set_start(ops.get_start());
        response.set_stop(self.last_index(ops, &rows));
        let mut idents = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let count: i64 = row.get("the_real_total");
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

    fn vec_to_delimited_string<T: Display>(&self, parts: &[T]) -> String {
        parts
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(",")
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

    fn row_to_origin_package(
        &self,
        row: &postgres::rows::Row,
    ) -> SrvResult<originsrv::OriginPackage> {
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

        let pv: String = row.get("visibility");
        let pv2: originsrv::OriginPackageVisibility =
            pv.parse().map_err(SrvError::UnknownOriginPackageVisibility)?;
        package.set_visibility(pv2);

        Ok(package)
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
    ) -> SrvResult<originsrv::OriginChannel> {
        let conn = self.pool.get(occ)?;

        let rows = conn.query(
            "SELECT * FROM insert_origin_channel_v1($1, $2, $3)",
            &[
                &(occ.get_origin_id() as i64),
                &(occ.get_owner_id() as i64),
                &occ.get_name(),
            ],
        ).map_err(SrvError::OriginChannelCreate)?;
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
    ) -> SrvResult<originsrv::OriginChannelListResponse> {
        let conn = self.pool.get(oclr)?;

        let rows = &conn.query(
            "SELECT * FROM get_origin_channels_for_origin_v2($1, $2)",
            &[
                &(oclr.get_origin_id() as i64),
                &oclr.get_include_sandbox_channels(),
            ],
        ).map_err(SrvError::OriginChannelList)?;

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
    ) -> SrvResult<Option<originsrv::OriginChannel>> {
        let conn = self.pool.get(ocg)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_channel_v1($1, $2)",
            &[&ocg.get_origin_name(), &ocg.get_name()],
        ).map_err(SrvError::OriginChannelGet)?;

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
    ) -> SrvResult<()> {
        let conn = self.pool.get(opp)?;
        let pkg_ids: Vec<i64> = opp.get_package_ids()
            .to_vec()
            .iter()
            .map(|&x| x as i64)
            .collect();

        &conn.query(
            "SELECT * FROM promote_origin_package_group_v1($1, $2)",
            &[&(opp.get_channel_id() as i64), &(pkg_ids)],
        ).map_err(SrvError::OriginPackageGroupPromote)?;

        Ok(())
    }

    pub fn promote_origin_package(&self, opp: &originsrv::OriginPackagePromote) -> SrvResult<()> {
        let conn = self.pool.get(opp)?;
        &conn.query(
            "SELECT * FROM promote_origin_package_v1($1, $2)",
            &[
                &(opp.get_channel_id() as i64),
                &(opp.get_package_id() as i64),
            ],
        ).map_err(SrvError::OriginPackagePromote)?;

        Ok(())
    }

    pub fn demote_origin_package(&self, opp: &originsrv::OriginPackageDemote) -> SrvResult<()> {
        let conn = self.pool.get(opp)?;
        &conn.query(
            "SELECT * FROM demote_origin_package_v1($1, $2)",
            &[
                &(opp.get_channel_id() as i64),
                &(opp.get_package_id() as i64),
            ],
        ).map_err(SrvError::OriginPackageDemote)?;

        Ok(())
    }

    pub fn delete_origin_channel_by_id(
        &self,
        ocd: &originsrv::OriginChannelDelete,
    ) -> SrvResult<()> {
        let conn = self.pool.get(ocd)?;
        conn.execute(
            "SELECT delete_origin_channel_v1($1)",
            &[&(ocd.get_id() as i64)],
        ).map_err(SrvError::OriginChannelDelete)?;
        Ok(())
    }

    pub fn create_origin_integration(
        &self,
        oic: &originsrv::OriginIntegrationCreate,
    ) -> SrvResult<()> {
        let conn = self.pool.get(oic)?;

        let rows = conn.query(
            "SELECT * FROM insert_origin_integration_v1($1, $2, $3, $4)",
            &[
                &oic.get_integration().get_origin(),
                &oic.get_integration().get_integration(),
                &oic.get_integration().get_name(),
                &oic.get_integration().get_body(),
            ],
        ).map_err(SrvError::OriginIntegrationCreate)?;
        rows.iter().nth(0).expect(
            "Insert returns row, but no row present",
        );
        Ok(())
    }

    pub fn get_origin_integration_names(
        &self,
        oig: &originsrv::OriginIntegrationGetNames,
    ) -> SrvResult<Option<originsrv::OriginIntegrationNames>> {
        let conn = self.pool.get(oig)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_integrations_v1($1, $2)",
            &[&oig.get_origin(), &oig.get_integration()],
        ).map_err(SrvError::OriginIntegrationGetNames)?;

        if rows.len() != 0 {
            Ok(Some(self.rows_to_origin_integration_names(&rows)))
        } else {
            Ok(None)
        }
    }

    pub fn origin_integration_request(
        &self,
        oir: &originsrv::OriginIntegrationRequest,
    ) -> SrvResult<originsrv::OriginIntegrationResponse> {
        let conn = self.pool.get(oir)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_integrations_for_origin_v1($1)",
            &[&oir.get_origin()],
        ).map_err(SrvError::OriginIntegrationRequest)?;

        let mut response = originsrv::OriginIntegrationResponse::new();
        let mut integrations = protobuf::RepeatedField::new();

        for row in rows {
            integrations.push(self.row_to_origin_integration(&row));
        }

        response.set_integrations(integrations);
        Ok(response)
    }

    fn row_to_origin_integration(&self, row: &postgres::rows::Row) -> originsrv::OriginIntegration {
        let mut oi = originsrv::OriginIntegration::new();
        oi.set_origin(row.get("origin"));
        oi.set_integration(row.get("integration"));
        oi.set_name(row.get("name"));
        oi.set_body(row.get("body"));
        oi
    }

    pub fn delete_origin_integration(
        &self,
        oid: &originsrv::OriginIntegrationDelete,
    ) -> SrvResult<()> {
        let conn = self.pool.get(oid)?;

        conn.execute(
            "SELECT delete_origin_integration_v1($1, $2, $3)",
            &[
                &oid.get_integration().get_origin(),
                &oid.get_integration().get_integration(),
                &oid.get_integration().get_name(),
            ],
        ).map_err(SrvError::OriginIntegrationDelete)?;
        Ok(())
    }

    pub fn delete_origin_member(&self, omr: &originsrv::OriginMemberRemove) -> SrvResult<()> {
        let conn = self.pool.get(omr)?;

        conn.execute(
            "SELECT delete_origin_member_v1($1, $2)",
            &[&(omr.get_origin_id() as i64), &omr.get_account_name()],
        ).map_err(SrvError::OriginMemberDelete)?;
        Ok(())
    }

    fn rows_to_origin_integration_names(
        &self,
        rows: &postgres::rows::Rows,
    ) -> originsrv::OriginIntegrationNames {
        let mut oin = originsrv::OriginIntegrationNames::new();
        let mut names = protobuf::RepeatedField::new();
        for row in rows.iter() {
            let name: String = row.get("name");
            names.push(name);
        }

        oin.set_names(names);
        oin
    }
}

fn sync_origins(pool: Pool, mut route_conn: RouteClient) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_origins_v1()", &[]).map_err(
            DbError::AsyncFunctionCheck,
        )?;
        if rows.len() > 0 {
            let mut request = sessionsrv::AccountOriginCreate::new();
            for row in rows.iter() {
                let aid: i64 = row.get("account_id");
                let oid: i64 = row.get("origin_id");
                request.set_account_id(aid as u64);
                request.set_account_name(row.get("account_name"));
                request.set_origin_id(oid as u64);
                request.set_origin_name(row.get("origin_name"));
                match route_conn.route::<sessionsrv::AccountOriginCreate, NetOk>(&request) {
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
                            "Failed to sync origin creation with the session service, {:?}, {}",
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

fn sync_packages(pool: Pool, mut route_conn: RouteClient) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query("SELECT * FROM sync_packages_v2()", &[])
            .map_err(DbError::AsyncFunctionCheck)?;
        if rows.len() > 0 {
            let mut request = jobsrv::JobGraphPackageCreate::new();
            for row in rows.iter() {
                let pid: i64 = row.get("package_id");
                let ident: String = row.get("package_ident");
                let deps_column: String = row.get("package_deps");
                let target: String = row.get("package_target");

                let mut deps = protobuf::RepeatedField::new();
                for ident in deps_column.split(":") {
                    if !ident.is_empty() {
                        let opi = originsrv::OriginPackageIdent::from_str(ident).unwrap();
                        let dep_str = format!("{}", opi);
                        deps.push(dep_str);
                    }
                }
                request.set_ident(ident);
                request.set_target(target);
                request.set_deps(deps);

                match route_conn.route::<jobsrv::JobGraphPackageCreate, NetOk>(&request) {
                    Ok(_) => {
                        conn.query("SELECT * FROM set_packages_sync_v1($1)", &[&pid])
                            .map_err(DbError::AsyncFunctionUpdate)?;
                        debug!(
                            "Updated jobsrv service with package creation, {:?}",
                            request
                        );
                    }
                    Err(e) => {
                        warn!(
                            "Failed to sync package creation with the jobsrv service, {:?}: {}",
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

fn sync_invitations(pool: Pool, mut route_conn: RouteClient) -> DbResult<EventOutcome> {
    let mut result = EventOutcome::Finished;
    for shard in pool.shards.iter() {
        let conn = pool.get_shard(*shard)?;
        let rows = &conn.query(
            "SELECT * FROM get_origin_invitations_not_synced_with_account_v1()",
            &[],
        ).map_err(DbError::AsyncFunctionCheck)?;
        if rows.len() > 0 {
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
                match route_conn.route::<sessionsrv::AccountOriginInvitationCreate, NetOk>(&aoic) {
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
