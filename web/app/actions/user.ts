// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

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
