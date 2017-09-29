// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use hcore::service::ServiceGroup;

use census::{CensusGroup, CensusMember, CensusRing, ElectionStatus};
use manager::Sys;
use manager::service::{Cfg, Pkg, ServiceBind};

#[derive(Clone, Debug, Serialize)]
pub struct Binds<'a>(HashMap<String, BindGroup<'a>>);

impl<'a> Binds<'a> {
    fn new<T>(bindings: T, census: &'a CensusRing) -> Self
    where
        T: Iterator<Item = &'a ServiceBind>,
    {
        let mut map = HashMap::default();
        for bind in bindings {
            if let Some(group) = census.census_group_for(&bind.service_group) {
                map.insert(bind.name.to_string(), BindGroup::new(group));
            }
        }
        Binds(map)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct BindGroup<'a> {
    pub first: Option<SvcMember<'a>>,
    pub members: Vec<SvcMember<'a>>,
}

impl<'a> BindGroup<'a> {
    fn new(group: &'a CensusGroup) -> Self {
        BindGroup {
            first: select_first(group),
            members: group.members().iter().map(|m| SvcMember(m)).collect(),
        }
    }
}

/// The context of a render call.
///
/// It stores information on a Service and its configuration.
#[derive(Clone, Debug, Serialize)]
pub struct RenderContext<'a> {
    pub sys: &'a Sys,
    pub pkg: &'a Pkg,
    pub cfg: &'a Cfg,
    pub svc: Svc<'a>,
    pub bind: Binds<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn new<T>(
        service_group: &ServiceGroup,
        sys: &'a Sys,
        pkg: &'a Pkg,
        cfg: &'a Cfg,
        census: &'a CensusRing,
        bindings: T,
    ) -> RenderContext<'a>
    where
        T: Iterator<Item = &'a ServiceBind>,
    {
        let census_group = census.census_group_for(&service_group).expect(
            "Census Group missing from list!",
        );
        RenderContext {
            sys: sys,
            pkg: pkg,
            cfg: cfg,
            svc: Svc::new(census_group),
            bind: Binds::new(bindings, census),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Svc<'a> {
    pub service: &'a str,
    pub group: &'a str,
    pub org: Option<&'a str>,
    pub election_is_running: bool,
    pub election_is_no_quorum: bool,
    pub election_is_finished: bool,
    pub update_election_is_running: bool,
    pub update_election_is_no_quorum: bool,
    pub update_election_is_finished: bool,
    pub me: SvcMember<'a>,
    pub first: SvcMember<'a>,
    pub members: Vec<SvcMember<'a>>,
    pub leader: Option<SvcMember<'a>>,
    pub update_leader: Option<SvcMember<'a>>,
}

impl<'a> Svc<'a> {
    fn new(census_group: &'a CensusGroup) -> Self {
        Svc {
            service: census_group.service_group.service(),
            group: census_group.service_group.group(),
            org: census_group.service_group.org(),
            election_is_running: census_group.election_status == ElectionStatus::ElectionInProgress,
            election_is_no_quorum: census_group.election_status == ElectionStatus::ElectionNoQuorum,
            election_is_finished: census_group.election_status == ElectionStatus::ElectionFinished,
            update_election_is_running: census_group.election_status ==
                ElectionStatus::ElectionInProgress,
            update_election_is_no_quorum: census_group.election_status ==
                ElectionStatus::ElectionNoQuorum,
            update_election_is_finished: census_group.election_status ==
                ElectionStatus::ElectionFinished,
            me: SvcMember(census_group.me().expect("Missing 'me'")),
            members: census_group
                .members()
                .iter()
                .map(|m| SvcMember(m))
                .collect(),
            leader: census_group.leader().map(|m| SvcMember(m)),
            first: select_first(census_group).expect("First should always be present on svc"),
            update_leader: census_group.update_leader().map(|m| SvcMember(m)),
        }
    }
}

/// A friendly representation of a `CensusMember` to the templating system.
#[derive(Clone, Debug, Serialize)]
pub struct SvcMember<'a>(&'a CensusMember);

/// Helper for pulling the leader or first member from a census group. This is used to populate the
/// `.first` field in `bind` and `svc`.
fn select_first(census_group: &CensusGroup) -> Option<SvcMember> {
    match census_group.leader() {
        Some(member) => Some(SvcMember(member)),
        None => {
            census_group.members().first().and_then(
                |m| Some(SvcMember(m)),
            )
        }
    }
}
