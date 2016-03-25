// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The http sidecar for bldr services. Provides an interface to verifying and validating
//! promises.
//!
//! Supports:
//!
//! * /config: Returns the current configuration of the service
//! * /health: Returns the current health of the service
//! * /status: Returns the current status of the service, from the supervisors point of view

use rustc_serialize::json;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, StopReason, ActorSender};

use error::{BldrError, ErrorKind};
use health_check;
use package::Package;
use service_config::ServiceConfig;
use gossip::member::{MemberList, MemberId};
use gossip::rumor::RumorList;
use gossip::detector::Detector;
use census::{CensusList, CensusEntry, CensusEntryId, Census};
use election::{Election, ElectionList};

static LOGKEY: &'static str = "SC";
const GET_HEALTH: &'static str = "/health";
const GET_CONFIG: &'static str = "/config";
const GET_STATUS: &'static str = "/status";
const GET_GOSSIP: &'static str = "/gossip";
const GET_CENSUS: &'static str = "/census";
const GET_ELECTION: &'static str = "/election";
const LISTEN_ADDR: &'static str = "0.0.0.0:9631";

pub type SidecarActor = wonder::actor::Actor<SidecarMessage>;

pub struct Sidecar;

pub struct SidecarState {
    /// The package this sidecar is helping out
    pub package: Arc<RwLock<Package>>,
    /// The configuration of the supervised service
    pub config: Arc<RwLock<ServiceConfig>>,
    pub member_list: Arc<RwLock<MemberList>>,
    pub rumor_list: Arc<RwLock<RumorList>>,
    pub census_list: Arc<RwLock<CensusList>>,
    pub detector: Arc<RwLock<Detector>>,
    pub election_list: Arc<RwLock<ElectionList>>,
}

#[derive(Debug)]
pub enum SidecarMessage {
    Ok,
    Stop,
}

impl SidecarState {
    pub fn new(package: Arc<RwLock<Package>>,
               config: Arc<RwLock<ServiceConfig>>,
               member_list: Arc<RwLock<MemberList>>,
               rumor_list: Arc<RwLock<RumorList>>,
               census_list: Arc<RwLock<CensusList>>,
               detector: Arc<RwLock<Detector>>,
               election_list: Arc<RwLock<ElectionList>>)
               -> Self {
        SidecarState {
            package: package,
            config: config,
            member_list: member_list,
            rumor_list: rumor_list,
            census_list: census_list,
            detector: detector,
            election_list: election_list,
        }
    }
}

impl Sidecar {
    /// Start the sidecar.
    pub fn start(package: Arc<RwLock<Package>>,
                 config: Arc<RwLock<ServiceConfig>>,
                 member_list: Arc<RwLock<MemberList>>,
                 rumor_list: Arc<RwLock<RumorList>>,
                 census_list: Arc<RwLock<CensusList>>,
                 detector: Arc<RwLock<Detector>>,
                 election_list: Arc<RwLock<ElectionList>>)
                 -> SidecarActor {
        let state = SidecarState::new(package,
                                      config,
                                      member_list,
                                      rumor_list,
                                      census_list,
                                      detector,
                                      election_list);
        wonder::actor::Builder::new(Sidecar).name("sidecar".to_string()).start(state).unwrap()
    }
}

impl GenServer for Sidecar {
    type T = SidecarMessage;
    type S = SidecarState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, _state: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(0))
    }

    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        let mut router = Router::new();
        let package_1 = state.package.clone();
        let package_2 = state.package.clone();
        let package_3 = state.package.clone();
        let config_1 = state.config.clone();

        router.get(GET_CONFIG, move |r: &mut Request| config(&package_1, r));
        router.get(GET_STATUS, move |r: &mut Request| status(&package_2, r));
        router.get(GET_HEALTH,
                   move |r: &mut Request| health(&package_3, &config_1, r));

        let ml = state.member_list.clone();
        let rl = state.rumor_list.clone();
        let detector = state.detector.clone();
        let id = {
            Arc::new(ml.read().unwrap().my_id.clone())
        };

        router.get(GET_GOSSIP,
                   move |r: &mut Request| gossip(&ml, &rl, &detector, &id, r));

        let cl1 = state.census_list.clone();
        router.get(GET_CENSUS, move |r: &mut Request| census(&cl1, r));

        let el = state.election_list.clone();
        router.get(GET_ELECTION, move |r: &mut Request| election(&el, r));

        match Iron::new(router).http(LISTEN_ADDR) {
            Ok(_) => HandleResult::NoReply(None),
            Err(_) => {
                HandleResult::Stop(StopReason::Fatal("couldn't start router".to_string()), None)
            }
        }
    }
}

#[derive(Debug, RustcEncodable)]
struct ElectionResponse<'a> {
    elections: &'a HashMap<String, Election>,
    mine: Option<&'a Election>,
}

fn election(election_list: &Arc<RwLock<ElectionList>>, _req: &mut Request) -> IronResult<Response> {
    let el = election_list.read().unwrap();
    let er = ElectionResponse {
        elections: &el.elections,
        mine: el.election(),
    };

    let json_response = match json::encode(&er) {
        Ok(json_response) => json_response,
        Err(e) => return Err(IronError::from(bldr_error!(ErrorKind::JsonEncode(e)))),
    };

    Ok(Response::with((status::Ok, json_response)))
}

#[derive(Debug, RustcEncodable)]
struct GossipResponse<'a> {
    id: &'a MemberId,
    member_list: &'a MemberList,
    rumor_list: &'a RumorList,
    detector: &'a Detector,
}

/// The /gossip callback.
///
/// Returns information about the gossip ring.
fn gossip(member_list: &Arc<RwLock<MemberList>>,
          rumor_list: &Arc<RwLock<RumorList>>,
          detector: &Arc<RwLock<Detector>>,
          id: &Arc<MemberId>,
          _req: &mut Request)
          -> IronResult<Response> {
    let ml = member_list.read().unwrap();
    let rl = rumor_list.read().unwrap();
    let detector = detector.read().unwrap();

    let gossip_response = GossipResponse {
        id: id,
        member_list: &*ml,
        rumor_list: &*rl,
        detector: &*detector,
    };

    let json_response = match json::encode(&gossip_response) {
        Ok(json_response) => json_response,
        Err(e) => return Err(IronError::from(bldr_error!(ErrorKind::JsonEncode(e)))),
    };

    Ok(Response::with((status::Ok, json_response)))
}

#[derive(Debug, RustcEncodable)]
struct CensusResponse<'a> {
    id: &'a CensusEntryId,
    census_list: &'a CensusList,
    me: &'a CensusEntry,
    local_census: &'a Census,
    minimum_quorum: bool,
    quorum: bool,
    leader: Option<&'a CensusEntry>,
}

/// The /census callback.
///
/// Returns information about the census.
fn census(census_list: &Arc<RwLock<CensusList>>, _req: &mut Request) -> IronResult<Response> {
    let cl = census_list.read().unwrap();
    let response = CensusResponse {
        id: &cl.me().id.clone(),
        census_list: &cl,
        me: cl.me(),
        local_census: cl.local_census(),
        minimum_quorum: cl.local_census().minimum_quorum(),
        quorum: cl.local_census().has_quorum(),
        leader: cl.local_census().get_leader(),
    };

    let json_response = match json::encode(&response) {
        Ok(json_response) => json_response,
        Err(e) => return Err(IronError::from(bldr_error!(ErrorKind::JsonEncode(e)))),
    };

    Ok(Response::with((status::Ok, json_response)))
}

/// The /config callback.
///
/// Returns the current running configuration.
///
/// # Failures
///
/// * Fails if the configuration cannot be found.
fn config(lock: &Arc<RwLock<Package>>, _req: &mut Request) -> IronResult<Response> {
    let package = lock.read().unwrap();
    let last_config = try!(package.last_config());
    Ok(Response::with((status::Ok, last_config)))
}

/// The /status callback.
///
/// Returns the current status from the supervisors perspective.
///
/// # Failures
///
/// * Fails if the supervisor cannot return the status.
fn status(lock: &Arc<RwLock<Package>>, _req: &mut Request) -> IronResult<Response> {
    let package = lock.read().unwrap();
    let output = try!(package.status_via_pidfile());
    Ok(Response::with((status::Ok, output)))
}

/// The /health callback.
///
/// Runs the packages health_check, if one is defined. Returns the status, and outputs both the
/// status and config.
///
/// # Failures
///
/// * If the health_check cannot be run.
fn health(package_lock: &Arc<RwLock<Package>>,
          config_lock: &Arc<RwLock<ServiceConfig>>,
          _req: &mut Request)
          -> IronResult<Response> {
    let result = {
        let package = package_lock.read().unwrap();
        let config = config_lock.read().unwrap();
        try!(package.health_check(&config))
    };

    match result.status {
        health_check::Status::Ok | health_check::Status::Warning => {
            Ok(Response::with((status::Ok, format!("{}", result))))
        }
        health_check::Status::Critical => {
            Ok(Response::with((status::ServiceUnavailable, format!("{}", result))))
        }
        health_check::Status::Unknown => {
            Ok(Response::with((status::InternalServerError, format!("{}", result))))
        }
    }
}

/// Translates BldrErrors into IronErrors
impl From<BldrError> for IronError {
    fn from(err: BldrError) -> IronError {
        IronError {
            error: Box::new(err),
            response: Response::with((status::InternalServerError, "Internal bldr error")),
        }
    }
}
