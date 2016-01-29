// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as api from "./api";

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

// Fetch the explore endpoint
export function fetchExplore() {
    return dispatch => {
        api.get("explore").then(response => {
            dispatch(populateExplore(response));
        }).catch(error => console.error(error));
    };
}

export function filterPackagesBy(params) {
    return dispatch => {
        api.get("packages").then(response => {
            dispatch(setPackages(response));
            dispatch(setVisiblePackages(params));
        });
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
