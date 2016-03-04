// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {fromJS} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function gitHub(state = initialState["gitHub"], action) {
    switch (action.type) {
        case actionTypes.LINK_GITHUB_ACCOUNT_SUCCESS:
            return state.
                set("isLinked", true).
                set("username", action.payload);

        case actionTypes.POPULATE_GITHUB_REPOS:
            return state.set("repos", fromJS(action.payload));

        case actionTypes.SET_SELECTED_GITHUB_ORG:
            return state.set("selectedOrg", action.payload);

        case actionTypes.UNLINK_GITHUB_ACCOUNT_SUCCESS:
            return state.
                set("isLinked", false).
                set("username", undefined);

        default:
            return state;
    }
}