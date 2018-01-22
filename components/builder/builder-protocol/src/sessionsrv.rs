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

use std::result;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use message::{Persistable, Routable};

use sharding::InstaId;
pub use message::sessionsrv::*;

impl Routable for SessionCreate {
    type H = u32;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_extern_id())
    }
}

impl Routable for SessionGet {
    type H = u32;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_token().get_extern_id())
    }
}

impl Persistable for Account {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Into<Session> for Account {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("account", 3)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("email", self.get_email())?;
        strukt.end()
    }
}

impl Routable for AccountCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl Routable for AccountFindOrCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl Routable for AccountGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl Routable for AccountGetId {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_id()))
    }
}

impl Routable for AccountUpdate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_id()))
    }
}


impl Routable for AccountOriginInvitationCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountInvitationListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountOriginInvitationIgnoreRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountOriginInvitationRescindRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountOriginListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountOriginCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Routable for AccountOriginRemove {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_name().to_string())
    }
}

impl Routable for AccountOriginInvitationAcceptRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_account_id()))
    }
}

impl Serialize for AccountInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "account_invitation_list_response",
            2,
        )?;
        strukt.serialize_field(
            "account_id",
            &self.get_account_id().to_string(),
        )?;
        strukt.serialize_field(
            "invitations",
            self.get_invitations(),
        )?;
        strukt.end()
    }
}

impl Serialize for AccountOriginInvitation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("account_origin_invitation", 6)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field(
            "origin_invitation_id",
            &self.get_origin_invitation_id().to_string(),
        )?;
        strukt.serialize_field(
            "account_id",
            &self.get_account_id().to_string(),
        )?;
        strukt.serialize_field(
            "account_name",
            self.get_account_name(),
        )?;
        strukt.serialize_field(
            "origin_id",
            &self.get_origin_id().to_string(),
        )?;
        strukt.serialize_field(
            "origin_name",
            self.get_origin_name(),
        )?;
        strukt.serialize_field(
            "owner_id",
            &self.get_owner_id().to_string(),
        )?;
        strukt.end()
    }
}

impl Serialize for AccountOriginListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct(
            "account_origin_list_response",
            2,
        )?;
        strukt.serialize_field(
            "account_id",
            &self.get_account_id().to_string(),
        )?;
        strukt.serialize_field("origins", self.get_origins())?;
        strukt.end()
    }
}

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("session", 6)?;
        strukt.serialize_field("id", &self.get_id().to_string())?;
        strukt.serialize_field("name", self.get_name())?;
        strukt.serialize_field("email", self.get_email())?;
        strukt.serialize_field("token", self.get_token())?;
        strukt.serialize_field("flags", &self.get_flags())?;
        strukt.serialize_field(
            "oauth_token",
            self.get_oauth_token(),
        )?;
        strukt.end()
    }
}
