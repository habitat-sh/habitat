// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {requestRoute, resetAppState} from "./index";

export const SIGN_IN_ATTEMPT = "SIGN_IN_ATTEMPT";
export const TOGGLE_USER_NAV_MENU = "TOGGLE_USER_NAV_MENU";

export function attemptSignIn(username) {
    return {
        type: SIGN_IN_ATTEMPT,
        payload: { username: username },
    };
}

export function toggleUserNavMenu() {
    return {
        type: TOGGLE_USER_NAV_MENU
    };
}

export function signOut() {
    sessionStorage.removeItem("gitHubAuthToken");

    return dispatch => {
        dispatch(resetAppState());
        dispatch(requestRoute(["SignIn"]));
    };
}