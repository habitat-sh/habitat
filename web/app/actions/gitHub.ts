// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as fakeApi from "../fakeApi";
import {requestRoute} from "./router";

export const LINK_GITHUB_ACCOUNT = "LINK_GITHUB_ACCOUNT";
export const LINK_GITHUB_ACCOUNT_SUCCESS = "LINK_GITHUB_ACCOUNT_SUCCESS";
export const POPULATE_GITHUB_REPOS = "POPULATE_GITHUB_REPOS";
export const SET_SELECTED_GITHUB_ORG = "SET_SELECTED_GITHUB_ORG";
export const UNLINK_GITHUB_ACCOUNT = "UNLINK_GITHUB_ACCOUNT";
export const UNLINK_GITHUB_ACCOUNT_SUCCESS = "UNLINK_GITHUB_ACCOUNT";

export function fetchGitHubRepos() {
    return dispatch => {
        fakeApi.get("github/user/repos.json").then(response => {
            dispatch(populateGitHubRepos(response));
        });
    };
}

export function linkGitHubAccount(username) {
    return dispatch => {
        window["githubWindow"] = window.open("/fixtures/authorizeGitHub.html");
        dispatch(linkGitHubAccountSuccess(username));
    };
}

function linkGitHubAccountSuccess(username) {
    return {
        type: LINK_GITHUB_ACCOUNT_SUCCESS,
        payload: username,
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

export function unlinkGitHubAccount() {
    return dispatch => {
        dispatch(unlinkGitHubAccountSuccess());
    };
}

function unlinkGitHubAccountSuccess() {
    return {
        type: UNLINK_GITHUB_ACCOUNT,
    };
}
