// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {requestRoute, removeSessionStorage, resetAppState} from "./index";

export const SET_SIGNING_IN_FLAG = "SET_SIGNING_IN_FLAG";
export const SIGN_IN_ATTEMPT = "SIGN_IN_ATTEMPT";
export const TOGGLE_USER_NAV_MENU = "TOGGLE_USER_NAV_MENU";

export function attemptSignIn(username) {
    return {
        type: SIGN_IN_ATTEMPT,
        payload: { username: username },
    };
}

export function setSigningInFlag(payload) {
    return {
        type: SET_SIGNING_IN_FLAG,
        payload,
    };
}

export function toggleUserNavMenu() {
    return {
        type: TOGGLE_USER_NAV_MENU
    };
}

export function signOut() {
    return dispatch => {
        dispatch(removeSessionStorage());
        dispatch(resetAppState());
        dispatch(requestRoute(["SignIn"]));
    };
}
