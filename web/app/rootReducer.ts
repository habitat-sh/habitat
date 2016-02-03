// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as actionTypes from "./actions";
import {List} from "immutable";
import initialState from "./initialState";
import query from "./query";

export function rootReducer(state = initialState, action) {
    console.log("New action received", action);

    switch (action.type) {

        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore", "packages"], List(action.payload));

        case actionTypes.ROUTE_CHANGE:
            return state.set("route", action.payload).
                set("requestedRoute", null);

        case actionTypes.SIGN_UP_ATTEMPT:
            return state.
                set("isSignUpFormSubmitted", true).
                set("username", action.payload.username).
                set("email", action.payload.email).
                set("password", action.payload.password);

        case actionTypes.SIGN_IN_ATTEMPT:
            return state.
                set("username", action.payload.username).
                set("isSignedIn", true);

        case actionTypes.SIGN_OUT:
            return state.
                set("isSignUpFormSubmitted", false).
                set("isSignedIn", false);

        case actionTypes.ROUTE_REQUESTED:
            return state.
                set("requestedRoute", action.payload);

        // Query the list of packages to set the currentPackage data.
        case actionTypes.SET_CURRENT_PACKAGE:
            return state.set("currentPackage",
                query(state.get("packages")).
                    fromParams(action.payload).first());

        case actionTypes.SET_PACKAGES:
            return state.set("packages", action.payload);

        case actionTypes.SET_VISIBLE_PACKAGES:
            const q = query(state.get("packages"));
            let p;
            if (action.payload.filter === "mine") {
                p = q.allMostRecentForDerivation("smith");
            } else if (action.payload.derivation) {
                p = q.allMostRecentForDerivation(action.payload.derivation);
            } else if (action.payload.name) {
                p = q.allForNameByStars(action.payload.name);
            } else {
                p = q.allMostRecent();
            }
            return state.set("visiblePackages", p.toArray());

        case actionTypes.TOGGLE_USER_NAV_MENU:
            return state.set("isUserNavOpen", !state.get("isUserNavOpen"));

        default:
            return state;
    }
}
