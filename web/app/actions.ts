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

export const POPULATE_BUILDS = "POPULATE_BUILDS";
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

// Fetch the list of builds for a package
export function fetchBuilds(pkg) {
    return dispatch => {
        api.get(`log/${packageString(pkg)}/builds.json`).then(response => {
            dispatch(populateBuilds(response));
            dispatch(fetchBuildLog(pkg, response));
        }).catch(error => {
            dispatch(populateBuilds([]));
        });
    };
}

// Fetch the build log for a package
export function fetchBuildLog(pkg, builds) {
    return dispatch => {
        builds.forEach(build => {
            api.get(`log/${packageString(pkg)}/${build.id}.txt`).then(response => {
                dispatch(populateBuildLog(build.id, response));
            }).catch(error => {
                dispatch(populateBuildLog(build.id, undefined));
            });
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

export function fetchPackage(pkg) {
    return dispatch => {
        api.get("packages.json").then(response => {
            dispatch(setPackages(response));
            dispatch(setCurrentPackage(pkg));
        });
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

export function populateBuilds(data) {
    return {
        type: POPULATE_BUILDS,
        payload: data,
    };
}

export function populateBuildLog(id, data) {
    return {
        type: POPULATE_BUILD_LOG,
        payload: { id, data: data ? ansiToHtml(data) : undefined },
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
