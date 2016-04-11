// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {fromJS} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function gitHub(state = initialState["gitHub"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_GITHUB_REPOS:
            return state.set("repos", fromJS(action.payload));

        case actionTypes.SET_GITHUB_AUTH_STATE:
            return state.set("authState", action.payload);

        case actionTypes.SET_SELECTED_GITHUB_ORG:
            return state.set("selectedOrg", action.payload);

        default:
            return state;
    }
}
