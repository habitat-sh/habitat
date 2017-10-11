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

//! The PostgreSQL backend for the Account Server.

use std::sync::Arc;

use db::config::{DataStoreCfg, ShardId};
use db::pool::Pool;
use db::migration::Migrator;
use protocol::sessionsrv;
use postgres;
use protobuf;

use error::{SrvError, SrvResult};
use migrations;

#[derive(Debug, Clone)]
pub struct DataStore {
    pub pool: Pool,
}

impl DataStore {
    pub fn new(cfg: &DataStoreCfg, shards: Vec<ShardId>) -> SrvResult<DataStore> {
        let pool = Pool::new(&cfg, shards)?;
        Ok(DataStore { pool: pool })
    }

    pub fn from_pool(pool: Pool, _: Arc<String>) -> SrvResult<DataStore> {
        Ok(DataStore { pool: pool })
    }

    pub fn setup(&self) -> SrvResult<()> {
        let conn = self.pool.get_raw()?;
        let xact = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        let mut migrator = Migrator::new(xact, self.pool.shards.clone());

        migrator.setup()?;

        migrations::accounts::migrate(&mut migrator)?;
        migrations::sessions::migrate(&mut migrator)?;
        migrations::invitations::migrate(&mut migrator)?;

        migrator.finish()?;

        Ok(())
    }

    pub fn account_find_or_create(
        &self,
        msg: &sessionsrv::AccountFindOrCreate,
    ) -> SrvResult<sessionsrv::Account> {
        let conn = self.pool.get(msg)?;
        let rows = conn.query(
            "SELECT * FROM select_or_insert_account_v1($1, $2)",
            &[&msg.get_name(), &msg.get_email()],
        )?;
        let row = rows.get(0);
        Ok(self.row_to_account(row))
    }

    pub fn create_account(
        &self,
        account_create: &sessionsrv::AccountCreate,
    ) -> SrvResult<sessionsrv::Account> {
        let conn = self.pool.get(account_create)?;
        let rows = conn.query(
            "SELECT * FROM select_or_insert_account_v1($1, $2)",
            &[&account_create.get_name(), &account_create.get_email()],
        ).map_err(SrvError::AccountCreate)?;
        let row = rows.get(0);
        let account = self.row_to_account(row);
        Ok(account)
    }

    pub fn get_account(
        &self,
        account_get: &sessionsrv::AccountGet,
    ) -> SrvResult<Option<sessionsrv::Account>> {
        let conn = self.pool.get(account_get)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_name_v1($1)",
            &[&account_get.get_name()],
        ).map_err(SrvError::AccountGet)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_account_by_id(
        &self,
        account_get_id: &sessionsrv::AccountGetId,
    ) -> SrvResult<Option<sessionsrv::Account>> {
        let conn = self.pool.get(account_get_id)?;
        let rows = conn.query(
            "SELECT * FROM get_account_by_id_v1($1)",
            &[&(account_get_id.get_id() as i64)],
        ).map_err(SrvError::AccountGetById)?;
        if rows.len() != 0 {
            let row = rows.get(0);
            Ok(Some(self.row_to_account(row)))
        } else {
            Ok(None)
        }
    }

    pub fn get_origins_by_account(
        &self,
        request: &sessionsrv::AccountOriginListRequest,
    ) -> SrvResult<sessionsrv::AccountOriginListResponse> {
        let conn = self.pool.get(request)?;
        let rows = conn.query(
            "SELECT * FROM get_account_origins_v1($1)",
            &[&(request.get_account_id() as i64)],
        ).map_err(SrvError::OriginAccountList)?;
        let mut response = sessionsrv::AccountOriginListResponse::new();
        response.set_account_id(request.get_account_id());
        let mut origins = protobuf::RepeatedField::new();

        if rows.len() > 0 {
            for row in rows.iter() {
                origins.push(row.get("origin_name"));
            }
        }
        response.set_origins(origins);
        Ok(response)
    }

    pub fn accept_origin_invitation(
        &self,
        request: &sessionsrv::AccountOriginInvitationAcceptRequest,
    ) -> SrvResult<()> {
        let conn = self.pool.get(request)?;
        let tr = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM accept_account_invitation_v1($1, $2)",
            &[&(request.get_invite_id() as i64), &request.get_ignore()],
        ).map_err(SrvError::AccountOriginInvitationAccept)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn ignore_origin_invitation(
        &self,
        request: &sessionsrv::AccountOriginInvitationIgnoreRequest,
    ) -> SrvResult<()> {
        let conn = self.pool.get(request)?;
        let tr = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM ignore_account_invitation_v1($1, $2)",
            &[
                &(request.get_invitation_id() as i64),
                &(request.get_account_id() as i64),
            ],
        ).map_err(SrvError::AccountOriginInvitationIgnore)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn rescind_origin_invitation(
        &self,
        request: &sessionsrv::AccountOriginInvitationRescindRequest,
    ) -> SrvResult<()> {
        let conn = self.pool.get(request)?;
        let tr = conn.transaction().map_err(SrvError::DbTransactionStart)?;
        tr.execute(
            "SELECT * FROM rescind_account_invitation_v1($1, $2)",
            &[
                &(request.get_invitation_id() as i64),
                &(request.get_account_id() as i64),
            ],
        ).map_err(SrvError::AccountOriginInvitationRescind)?;
        tr.commit().map_err(SrvError::DbTransactionCommit)?;
        Ok(())
    }

    pub fn create_origin(&self, request: &sessionsrv::AccountOriginCreate) -> SrvResult<()> {
        let conn = self.pool.get(request)?;
        conn.execute(
            "SELECT * FROM insert_account_origin_v1($1, $2, $3, $4)",
            &[
                &(request.get_account_id() as i64),
                &request.get_account_name(),
                &(request.get_origin_id() as i64),
                &request.get_origin_name(),
            ],
        ).map_err(SrvError::OriginCreate)?;
        Ok(())
    }

    pub fn delete_origin(&self, request: &sessionsrv::AccountOriginRemove) -> SrvResult<()> {
        let conn = self.pool.get(request)?;
        conn.execute(
            "SELECT delete_account_origin_v1($1, $2)",
            &[
                &request.get_account_name(),
                &(request.get_origin_id() as i64),
            ],
        ).map_err(SrvError::OriginCreate)?;
        Ok(())
    }

    pub fn create_account_origin_invitation(
        &self,
        invitation_create: &sessionsrv::AccountOriginInvitationCreate,
    ) -> SrvResult<()> {
        let conn = self.pool.get(invitation_create)?;
        let _rows = conn.query(
            "SELECT * FROM insert_account_invitation_v1($1, $2, $3, $4, $5, $6)",
            &[
                &(invitation_create.get_origin_id() as i64),
                &invitation_create.get_origin_name(),
                &(invitation_create.get_origin_invitation_id() as i64),
                &(invitation_create.get_account_id() as i64),
                &invitation_create.get_account_name(),
                &(invitation_create.get_owner_id() as i64),
            ],
        ).map_err(SrvError::AccountOriginInvitationCreate)?;
        Ok(())
    }

    pub fn list_invitations(
        &self,
        ailr: &sessionsrv::AccountInvitationListRequest,
    ) -> SrvResult<sessionsrv::AccountInvitationListResponse> {
        let conn = self.pool.get(ailr)?;
        let rows = &conn.query(
            "SELECT * FROM get_invitations_for_account_v1($1)",
            &[&(ailr.get_account_id() as i64)],
        ).map_err(SrvError::AccountOriginInvitationList)?;

        let mut response = sessionsrv::AccountInvitationListResponse::new();
        response.set_account_id(ailr.get_account_id());
        let mut invitations = protobuf::RepeatedField::new();
        for row in rows {
            let mut oi = sessionsrv::AccountOriginInvitation::new();
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
            let oi_origin_invitation_id: i64 = row.get("origin_invitation_id");
            oi.set_origin_invitation_id(oi_origin_invitation_id as u64);
            invitations.push(oi);
        }
        response.set_invitations(invitations);
        Ok(response)
    }

    fn row_to_account(&self, row: postgres::rows::Row) -> sessionsrv::Account {
        let mut account = sessionsrv::Account::new();
        let id: i64 = row.get("id");
        account.set_id(id as u64);
        account.set_email(row.get("email"));
        account.set_name(row.get("name"));
        account
    }
}
