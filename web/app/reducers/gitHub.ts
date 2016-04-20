// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {fromJS, List} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function gitHub(state = initialState["gitHub"], action) {
    switch (action.type) {
        case actionTypes.LOAD_SESSION_STATE:
            return state.set("authState", action.payload.gitHubAuthState).
                set("authToken", action.payload.gitHubAuthToken);

        case actionTypes.POPULATE_GITHUB_ORGS:
            return state.set("orgs",
                state.get("orgs").concat(fromJS(action.payload)).
                    sortBy(org => org.get("login")
                )
            );

        case actionTypes.POPULATE_GITHUB_REPOS:
            return state.set("repos",
                state.get("repos").concat(fromJS(action.payload)).
                    sortBy(repo => repo.get("name")));

        case actionTypes.RESET_GITHUB_ORGS:
            return state.set("orgs", List());

        case actionTypes.RESET_GITHUB_REPOS:
            return state.set("repos", List());

        case actionTypes.SET_GITHUB_AUTH_STATE:
            return state.set("authState", action.payload);

        case actionTypes.SET_GITHUB_AUTH_TOKEN:
            return state.set("authToken", action.payload);

        case actionTypes.SET_GITHUB_ORGS_LOADING_FLAG:
            return state.setIn(["ui", "orgs", "loading"], action.payload);

        case actionTypes.SET_GITHUB_REPOS_LOADING_FLAG:
            return state.setIn(["ui", "repos", "loading"], action.payload);

        case actionTypes.SET_SELECTED_GITHUB_ORG:
            return state.set("selectedOrg", action.payload);

        default:
            return state;
    }
}
