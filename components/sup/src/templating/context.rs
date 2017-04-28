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
    fn new(bindings: &[ServiceBind], census: &'a CensusRing) -> Self {
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
    members: Vec<SvcMember<'a>>,
}

impl<'a> BindGroup<'a> {
    fn new(group: &'a CensusGroup) -> Self {
        BindGroup { members: group.members().iter().map(|m| SvcMember(m)).collect() }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RenderContext<'a> {
    pub sys: &'a Sys,
    pub pkg: &'a Pkg,
    pub cfg: &'a Cfg,
    pub svc: Svc<'a>,
    pub bind: Binds<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn new(service_group: &ServiceGroup,
               sys: &'a Sys,
               pkg: &'a Pkg,
               cfg: &'a Cfg,
               census: &'a CensusRing,
               bindings: &[ServiceBind])
               -> RenderContext<'a> {
        let census_group = census
            .census_group_for(&service_group)
            .expect("Census Group missing from list!");
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
    pub group: &'a ServiceGroup,
    pub election_running: bool,
    pub election_no_quorum: bool,
    pub election_finished: bool,
    pub update_election_running: bool,
    pub update_election_no_quorum: bool,
    pub update_election_finished: bool,
    pub me: SvcMember<'a>,
    pub members: Vec<SvcMember<'a>>,
}

impl<'a> Svc<'a> {
    fn new(census_group: &'a CensusGroup) -> Self {
        Svc {
            group: &census_group.service_group,
            election_running: census_group.election_status == ElectionStatus::ElectionInProgress,
            election_no_quorum: census_group.election_status == ElectionStatus::ElectionNoQuorum,
            election_finished: census_group.election_status == ElectionStatus::ElectionFinished,
            update_election_running: census_group.election_status ==
                                     ElectionStatus::ElectionInProgress,
            update_election_no_quorum: census_group.election_status ==
                                       ElectionStatus::ElectionNoQuorum,
            update_election_finished: census_group.election_status ==
                                      ElectionStatus::ElectionFinished,
            me: SvcMember(census_group.me().expect("Missing 'me'")),
            members: census_group
                .members()
                .iter()
                .map(|m| SvcMember(m))
                .collect(),
        }
    }
}

/// A friendly representation of a `CensusMember` to the templating system.
#[derive(Clone, Debug, Serialize)]
pub struct SvcMember<'a>(&'a CensusMember);
