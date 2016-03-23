// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function user(state = initialState["user"], action) {
    switch (action.type) {
        case actionTypes.SIGN_IN_ATTEMPT:
            return state.
                set("username", action.payload.username).
                set("isSignedIn", true);

        case actionTypes.SIGN_OUT:
            return state.
                set("isSignUpFormSubmitted", false).
                set("isSignedIn", false);

        case actionTypes.SIGN_UP_ATTEMPT:
            return state.
                set("isSignUpFormSubmitted", true).
                set("username", action.payload.username).
                set("email", action.payload.email).
                set("password", action.payload.password);

        case actionTypes.TOGGLE_USER_NAV_MENU:
            return state.set("isUserNavOpen", !state.get("isUserNavOpen"));

        default:
            return state;
    }
}
