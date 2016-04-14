// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {fromJS} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function users(state = initialState["users"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_GITHUB_USER_DATA:
            return state.
                setIn(["current", "gitHub"], fromJS(action.payload));

        case actionTypes.SET_SIGNING_IN_FLAG:
            return state.setIn(["current", "isSigningIn"], action.payload);

        case actionTypes.SIGN_IN_ATTEMPT:
            return state.
                setIn(["current", "username"], action.payload.username).
                setIn(["current", "isSignedIn"], true);

        case actionTypes.TOGGLE_USER_NAV_MENU:
            return state.setIn(["current", "isUserNavOpen"],
                !state.getIn(["current", "isUserNavOpen"]));

        default:
            return state;
    }
}
