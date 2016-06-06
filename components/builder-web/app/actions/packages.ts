// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as depotApi from "../depotApi";
import * as fakeApi from "../fakeApi";

export const CLEAR_PACKAGES = "CLEAR_PACKAGES";
export const POPULATE_EXPLORE = "POPULATE_EXPLORE";
export const SET_CURRENT_PACKAGE = "SET_CURRENT_PACKAGE";
export const SET_PACKAGES_NEXT_RANGE = "SET_PACKAGES_NEXT_RANGE";
export const SET_PACKAGES_SEARCH_QUERY = "SET_PACKAGES_SEARCH_QUERY";
export const SET_PACKAGES_TOTAL_COUNT = "SET_PACKAGES_TOTAL_COUNT";
export const SET_VISIBLE_PACKAGES = "SET_VISIBLE_PACKAGES";

function clearPackages() {
    return {
        type: CLEAR_PACKAGES,
    };
}

// Fetch the explore endpoint
export function fetchExplore() {
    return dispatch => {
        fakeApi.get("explore.json").then(response => {
            dispatch(populateExplore(response));
        }).catch(error => console.error(error));
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

export function filterPackagesBy(params, query: string, nextRange: number = 0) {
    return dispatch => {
        if (nextRange === 0) {
            dispatch(clearPackages());
        }

        if (query) {
            params = { query };
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

export function populateExplore(data) {
    return {
        type: POPULATE_EXPLORE,
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
