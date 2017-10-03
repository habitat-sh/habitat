// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import {fromJS, List} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function gitHub(state = initialState["gitHub"], action) {
    switch (action.type) {

        case actionTypes.CLEAR_GITHUB_FILES:
            return state.set("files", List());

        case actionTypes.CLEAR_GITHUB_REPOS:
            return state.set("repos", List());

        case actionTypes.LOAD_SESSION_STATE:
            return state.set("authState", action.payload.gitHubAuthState).
                set("authToken", action.payload.gitHubAuthToken);

        case actionTypes.POPULATE_GITHUB_INSTALLATIONS:
            return state.set("installations", fromJS(action.payload.installations));

        case actionTypes.POPULATE_GITHUB_INSTALLATION_REPOSITORIES:
            return state.set("installationRepositories", fromJS(action.payload.repositories));

        case actionTypes.POPULATE_GITHUB_REPOS:
            return state.set("repos",
                state.get("repos").concat(fromJS(action.payload)).
                    sortBy(repo => repo.get("name")));

        case actionTypes.POPULATE_GITHUB_FILES:
            return state.set("files", fromJS(action.payload.items.sort((a, b) => {
                if (a.path < b.path) { return -1; }
                if (a.path > b.path) { return 1; }
                return 0;
            })));

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
