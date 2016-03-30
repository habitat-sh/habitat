// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as depotApi from "../depotApi";
import * as fakeApi from "../fakeApi";

export const POPULATE_EXPLORE = "POPULATE_EXPLORE";
export const SET_CURRENT_PACKAGE = "SET_CURRENT_PACKAGE";
export const SET_VISIBLE_PACKAGES = "SET_VISIBLE_PACKAGES";

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
        depotApi.get(pkg.ident).then(response => {
            dispatch(setCurrentPackage(response));
        });
    };
}

export function filterPackagesBy(params) {
    return dispatch => {
        if ("origin" in params) {
            depotApi.get(params).then(response => {
                dispatch(setVisiblePackages(response));
            });
        } else {
            fakeApi.get("packages.json").then(response => {
                dispatch(setVisiblePackages(response));
            });
        }
    };
}

export function populateExplore(data) {
    return {
        type: POPULATE_EXPLORE,
        payload: data,
    };
}

export function setCurrentPackage(pkg) {
    return {
        type: SET_CURRENT_PACKAGE,
        payload: pkg,
    };
}

export function setVisiblePackages(params) {
    return {
        type: SET_VISIBLE_PACKAGES,
        payload: params,
    };
}
