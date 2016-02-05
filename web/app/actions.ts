// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as api from "./api";
import {packageString} from "./util";

// The ansi_up module does not have TypeScript type definitions, so it needs to
// be loaded with a CommonJS require call, which will end up being handled by
// webpack.
const ansiToHtml = require("ansi_up").ansi_to_html;

export const POPULATE_BUILD_LOG = "POPULATE_BUILD_LOG";
export const POPULATE_EXPLORE = "POPULATE_EXPLORE";
export const ROUTE_CHANGE = "ROUTE_CHANGE";
export const ROUTE_REQUESTED = "ROUTE_REQUESTED";
export const SET_CURRENT_PACKAGE = "SET_CURRENT_PACKAGE";
export const SET_PACKAGES = "SET_PACKAGES";
export const SET_VISIBLE_PACKAGES = "SET_VISIBLE_PACKAGES";
export const SIGN_IN_ATTEMPT = "SIGN_IN_ATTEMPT";
export const SIGN_UP_ATTEMPT = "SIGN_UP_ATTEMPT";
export const SIGN_OUT = "SIGN_OUT";
export const TOGGLE_USER_NAV_MENU = "TOGGLE_USER_NAV_MENU";

export function attemptSignIn(username) {
    return {
        type: SIGN_IN_ATTEMPT,
        payload: { username: username },
    };
}

export function attemptSignUp(username, email, password) {
    return {
        type: SIGN_UP_ATTEMPT,
        payload: {
            username: username,
            email: email,
            password: password,
        }
    };
}

// Fetch the build log for a package
export function fetchBuildLog(pkg) {
    return dispatch => {
        api.get(`log/${packageString(pkg)}.txt`).then(response => {
            dispatch(populateBuildLog(response));
        }).catch(error => {
            // If the request comes back as a 404, that means we don't have
            // for this package.
            //
            // Right now I'm getting back "SyntaxError: Unexpected token C",
            // and I'm not sure if that's from the lite-server we're using
            // or something else. Ideally we want to check the error object to
            // make sure it's a 404, then log/reraise if it's not.
            dispatch(populateBuildLog(undefined));
        });
    };
}
// Fetch the explore endpoint
export function fetchExplore() {
    return dispatch => {
        api.get("explore.json").then(response => {
            dispatch(populateExplore(response));
        }).catch(error => console.error(error));
    };
}

export function filterPackagesBy(params) {
    return dispatch => {
        api.get("packages.json").then(response => {
            dispatch(setPackages(response));
            dispatch(setVisiblePackages(params));
        });
    };
}

export function populateBuildLog(data) {
    return {
        type: POPULATE_BUILD_LOG,
        payload: ansiToHtml(data)
    };
}
export function populateExplore(data) {
    return {
        type: POPULATE_EXPLORE,
        payload: data,
    };
}
export function routeChange(newRoute) {
    return {
        type: ROUTE_CHANGE,
        payload: newRoute,
    };
}

export function requestRoute(requestedRoute: Array<any>) {
    return {
        type: ROUTE_REQUESTED,
        payload: requestedRoute
    };
}

export function setPackages(packages) {
    return {
        type: SET_PACKAGES,
        payload: packages,
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

export function toggleUserNavMenu() {
    return {
        type: TOGGLE_USER_NAV_MENU
    };
}

export function signOut() {
    return {
        type: SIGN_OUT
    };
}
