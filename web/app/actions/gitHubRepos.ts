// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as api from "../api";
import {requestRoute} from "./router";

export const POPULATE_GITHUB_REPOS = "POPULATE_GITHUB_REPOS";
export const SET_SELECTED_GITHUB_ORG = "SET_SELECTED_GITHUB_ORG";

export function fetchGitHubRepos() {
    return dispatch => {
        api.get("github/user/repos.json").then(response => {
            dispatch(populateGitHubRepos(response));
        });
    };
}

export function onGitHubRepoSelect(repo) {
    return dispatch => {
        dispatch(requestRoute(
            ["ProjectCreate", { repo: encodeURIComponent(repo) }]
        ));
    };
}

function populateGitHubRepos(data) {
    return {
        type: POPULATE_GITHUB_REPOS,
        payload: data,
    };
}

export function setSelectedGitHubOrg(org) {
    return {
        type: SET_SELECTED_GITHUB_ORG,
        payload: org,
    };
}