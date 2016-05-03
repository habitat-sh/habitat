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
            dispatch(setCurrentPackage(response));
        }).catch(error => {
            dispatch(setCurrentPackage(undefined, error));
        });
    };
}

export function filterPackagesBy(params, defaultOrigin = {}) {
    if (params["filter"] === "mine" && "name" in defaultOrigin) {
        params["origin"] = defaultOrigin["name"];
    }

    return dispatch => {
        if ("origin" in params) {
            dispatch(clearPackages());
            depotApi.get(params).then(response => {
                dispatch(setVisiblePackages(response));
            }).catch(error => {
                dispatch(setVisiblePackages(undefined, error));
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

export function setCurrentPackage(pkg, error = undefined) {
    return {
        type: SET_CURRENT_PACKAGE,
        payload: pkg,
        error: error,
    };
}

export function setVisiblePackages(params, error = undefined) {
    return {
        type: SET_VISIBLE_PACKAGES,
        payload: params,
        error: error,
    };
}
