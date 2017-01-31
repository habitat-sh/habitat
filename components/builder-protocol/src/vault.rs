// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::result;

use serde::{Serialize, Serializer};

pub use message::vault::*;
use message::{Persistable, Routable};
use sharding::InstaId;

impl Persistable for Origin {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Routable for OriginGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: This won't accurately find the origin without it. We can switch to using the ID
        // of the origin or perform a reverse lookup by storing the name->ID map on a particular
        // vault server.
        Some(self.get_name().to_string())
    }
}

impl Routable for OriginCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Serialize for Origin {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("origin", 4));
        try!(serializer.serialize_struct_elt(&mut state, "id", self.get_id()));
        try!(serializer.serialize_struct_elt(&mut state, "name", self.get_name()));
        try!(serializer.serialize_struct_elt(&mut state, "owner_id", self.get_owner_id()));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "private_key_name",
                                             self.get_private_key_name()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for OriginMemberRemove {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginMemberListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Persistable for OriginSecretKey {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginSecretKey {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("origin_secret_key", 6));
        try!(serializer.serialize_struct_elt(&mut state, "id", self.get_id()));
        try!(serializer.serialize_struct_elt(&mut state, "origin_id", self.get_origin_id()));
        try!(serializer.serialize_struct_elt(&mut state, "name", self.get_name()));
        try!(serializer.serialize_struct_elt(&mut state, "revision", self.get_revision()));
        try!(serializer.serialize_struct_elt(&mut state, "body", self.get_body()));
        try!(serializer.serialize_struct_elt(&mut state, "owner_id", self.get_owner_id()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for OriginSecretKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginSecretKeyGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginInvitationCreate {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_owner_id())
    }
}

impl Persistable for OriginInvitation {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginInvitation {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("origin_invitation", 6));
        try!(serializer.serialize_struct_elt(&mut state, "id", self.get_id()));
        try!(serializer.serialize_struct_elt(&mut state, "account_id", self.get_account_id()));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "accaccount_name",
                                             self.get_account_name()));
        try!(serializer.serialize_struct_elt(&mut state, "origin_id", self.get_origin_id()));
        try!(serializer.serialize_struct_elt(&mut state, "origin_name", self.get_origin_name()));
        try!(serializer.serialize_struct_elt(&mut state, "owner_id", self.get_owner_id()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for AccountInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_account_id())
    }
}

impl Routable for AccountInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_account_id())
    }
}

impl Serialize for AccountInvitationListResponse {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("account_invitation_list_response", 2));
        try!(serializer.serialize_struct_elt(&mut state, "account_id", self.get_account_id()));
        try!(serializer.serialize_struct_elt(&mut state, "invitations", self.get_invitations()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for OriginInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_origin_id())
    }
}

impl Routable for OriginInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_origin_id())
    }
}

impl Serialize for OriginInvitationListResponse {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("origin_invitation_list_response", 2));
        try!(serializer.serialize_struct_elt(&mut state, "origin_id", self.get_origin_id()));
        try!(serializer.serialize_struct_elt(&mut state, "invitations", self.get_invitations()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for OriginInvitationAcceptRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO: we don't have an origin id here...
        Some(self.get_invite_id())
    }
}

impl Serialize for OriginMemberListResponse {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("origin_member_list_response", 2));
        try!(serializer.serialize_struct_elt(&mut state, "origin_id", self.get_origin_id()));
        try!(serializer.serialize_struct_elt(&mut state, "members", self.get_members()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for AccountOriginListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_account_id())
    }
}

impl Serialize for AccountOriginListResponse {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("account_origin_list_response", 2));
        try!(serializer.serialize_struct_elt(&mut state, "account_id", self.get_account_id()));
        try!(serializer.serialize_struct_elt(&mut state, "origins", self.get_origins()));
        serializer.serialize_struct_end(state)
    }
}

impl Routable for CheckOriginAccessRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO:
        Some(self.get_account_id())
    }
}

impl Routable for ProjectGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_id().to_string())
    }
}

impl Routable for ProjectCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_project().get_id().to_string())
    }
}

impl Routable for ProjectDelete {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_id().to_string())
    }
}

impl Routable for ProjectUpdate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_project().get_id().to_string())
    }
}

impl Persistable for Project {
    type Key = String;

    fn primary_key(&self) -> Self::Key {
        self.get_id().to_string()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for Project {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("project", 2));
        try!(serializer.serialize_struct_elt(&mut state, "id", self.get_id()));
        try!(serializer.serialize_struct_elt(&mut state, "plan_path", self.get_plan_path()));
        try!(serializer.serialize_struct_elt(&mut state, "vcs", self.get_git()));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for VCSGit {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("vcs", 2));
        try!(serializer.serialize_struct_elt(&mut state, "type", "git".to_string()));
        try!(serializer.serialize_struct_elt(&mut state, "url", self.get_url()));
        serializer.serialize_struct_end(state)
    }
}
