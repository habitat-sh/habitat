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

import { groupBy } from "lodash";
import * as depotApi from "../depotApi";
import * as fakeApi from "../fakeApi";
import { Package } from "../records/Package";

export const CLEAR_PACKAGES = "CLEAR_PACKAGES";
export const CLEAR_LATEST_IN_CHANNEL = "CLEAR_LATEST_IN_CHANNEL";
export const CLEAR_LATEST_PACKAGE = "CLEAR_LATEST_PACKAGE";
export const POPULATE_DASHBOARD_RECENT = "POPULATE_DASHBOARD_RECENT";
export const CLEAR_PACKAGE_VERSIONS = "CLEAR_PACKAGE_VERSIONS";
export const POPULATE_EXPLORE = "POPULATE_EXPLORE";
export const POPULATE_EXPLORE_STATS = "POPULATE_EXPLORE_STATS";
export const SET_CURRENT_PACKAGE = "SET_CURRENT_PACKAGE";
export const SET_LATEST_IN_CHANNEL = "SET_LATEST_IN_CHANNEL";
export const SET_LATEST_PACKAGE = "SET_LATEST_PACKAGE";
export const SET_CURRENT_PACKAGE_CHANNELS = "SET_CURRENT_PACKAGE_CHANNELS";
export const SET_CURRENT_PACKAGE_VERSIONS = "SET_CURRENT_PACKAGE_VERSIONS";
export const SET_PACKAGES_NEXT_RANGE = "SET_PACKAGES_NEXT_RANGE";
export const SET_PACKAGES_SEARCH_QUERY = "SET_PACKAGES_SEARCH_QUERY";
export const SET_PACKAGES_TOTAL_COUNT = "SET_PACKAGES_TOTAL_COUNT";
export const SET_VISIBLE_PACKAGES = "SET_VISIBLE_PACKAGES";
export const SET_VISIBLE_PACKAGE_CHANNELS = "SET_VISIBLE_PACKAGE_CHANNELS";

function clearPackages() {
    return {
        type: CLEAR_PACKAGES,
    };
}

function clearLatestInChannel(channel: string) {
    return {
        type: CLEAR_LATEST_IN_CHANNEL,
        payload: { channel }
    };
}

function clearLatestPackage() {
    return {
        type: CLEAR_LATEST_PACKAGE
    };
}

export function fetchDashboardRecent(origin: string) {
    return dispatch => {
        return depotApi.get({ origin: origin })
            .then(data => dispatch(populateDashboardRecent(data)))
            .catch(error => console.error(error));
    };
}

function clearPackageVersions() {
    return {
        type: CLEAR_PACKAGE_VERSIONS
    };
}

// Fetch the explore endpoint
export function fetchExplore() {
    return dispatch => {
        Promise.all([
            fakeApi.get("explore.json")
                .then(response => dispatch(populateExplore(response)))
                .catch(error => console.error(error)),
            depotApi.getStats("core")
                .then(data => dispatch(populateExploreStats(data)))
                .catch(error => console.error(error))
        ]);
    };
}

export function fetchPackage(pkg) {
    return dispatch => {
        dispatch(clearPackages());
        depotApi.get(pkg.ident).then(response => {
            dispatch(setCurrentPackage(response["results"]));
        }).catch(error => {
            dispatch(setCurrentPackage(undefined, error));
        });
    };
}

export function fetchLatestPackage(origin: string, name: string) {
    return dispatch => {
        dispatch(clearLatestPackage());

        depotApi.getLatest(origin, name).then(response => {
            dispatch(setLatestPackage(response));
        }).catch(error => {
            dispatch(setLatestPackage(undefined, error));
        });
    };
}

export function fetchLatestInChannel(origin: string, name: string, channel: string) {
    return dispatch => {
        dispatch(clearLatestInChannel(channel));

        depotApi.getLatestInChannel(origin, name, channel)
            .then(response => {
                dispatch(setLatestInChannel(channel, response));
            })
            .catch(error => {
                dispatch(setLatestInChannel(channel, undefined, error));
            });
    };
}

export function fetchPackageVersions(origin: string, name: string) {
    return dispatch => {
        dispatch(clearPackages());
        dispatch(clearPackageVersions());
        depotApi.getPackageVersions(origin, name).then(response => {
            dispatch(setCurrentPackageVersions(response));
        }).catch(error => {
            dispatch(setCurrentPackageVersions(undefined, error));
        });
    };
}

export function getUniquePackages(
    origin: string,
    nextRange: number = 0,
    token: string = ""
) {
    return dispatch => {
        if (nextRange === 0) {
            dispatch(clearPackages());
        }

        depotApi.getUnique(origin, nextRange, token).then(response => {
            dispatch(setVisiblePackages(response["results"]));
            dispatch(setPackagesTotalCount(response["totalCount"]));
            dispatch(setPackagesNextRange(response["nextRange"]));
        }).catch(error => {
            dispatch(setVisiblePackages(undefined, error));
        });
    };
}

export function filterPackagesBy(
    params,
    query: string,
    distinct: boolean,
    nextRange: number = 0
) {
    return dispatch => {
        if (nextRange === 0) {
            dispatch(clearPackages());
        }

        if (query) {
            params.query = encodeURIComponent(query);
        }

        if (distinct) {
            params.distinct = true;
        }

        depotApi.get(params, nextRange).then(response => {
            dispatch(setVisiblePackages(response["results"]));
            dispatch(setPackagesTotalCount(response["totalCount"]));
            dispatch(setPackagesNextRange(response["nextRange"]));
        }).catch(error => {
            dispatch(setVisiblePackages(undefined, error));
        });
    };
}

export function populateDashboardRecent(data) {
    let grouped = groupBy(data.results.package_list.reverse(), "name");
    let mapped = [];

    for (let k in grouped) {
        mapped.push(grouped[k][0]);
    }

    return {
        type: POPULATE_DASHBOARD_RECENT,
        payload: mapped
    };
}

export function populateExplore(data) {
    return {
        type: POPULATE_EXPLORE,
        payload: data,
    };
}

export function populateExploreStats(data) {
    return {
        type: POPULATE_EXPLORE_STATS,
        payload: data,
    };
}

export function setCurrentPackage(pkg, error = undefined) {
    return {
        type: SET_CURRENT_PACKAGE,
        payload: pkg,
        error: error,
    };
}

export function setLatestPackage(pkg, error = undefined) {
    return {
        type: SET_LATEST_PACKAGE,
        payload: pkg,
        error: error,
    };
}

export function setLatestInChannel(channel, pkg, error = undefined) {
    return {
        type: SET_LATEST_IN_CHANNEL,
        payload: { channel, pkg },
        error: error,
    };
}

export function setCurrentPackageVersions(versions, error = undefined) {
    return {
        type: SET_CURRENT_PACKAGE_VERSIONS,
        payload: versions,
        error: error,
    };
}

function setPackagesNextRange(payload: number) {
    return {
        type: SET_PACKAGES_NEXT_RANGE,
        payload,
    };
}

export function setPackagesSearchQuery(payload: string) {
    return {
        type: SET_PACKAGES_SEARCH_QUERY,
        payload,
    };
}

function setPackagesTotalCount(payload: number) {
    return {
        type: SET_PACKAGES_TOTAL_COUNT,
        payload,
    };
}

export function setVisiblePackages(params, error = undefined) {
    return {
        type: SET_VISIBLE_PACKAGES,
        payload: params,
        error: error,
    };
}
