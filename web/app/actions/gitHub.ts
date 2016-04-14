// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import config from "../config";
import * as fakeApi from "../fakeApi";
import {attemptSignIn, addNotification, goHome, requestRoute, setCurrentOrigin,
    signOut} from "./index";
import {DANGER} from "./notifications";

const uuid = require("node-uuid").v4;
const gitHubTokenAuthUrl = config["github_token_auth_url"];

export const POPULATE_GITHUB_REPOS = "POPULATE_GITHUB_REPOS";
export const POPULATE_GITHUB_USER_DATA = "POPULATE_GITHUB_USER_DATA";
export const SET_GITHUB_AUTH_STATE = "SET_GITHUB_AUTH_STATE";
export const SET_SELECTED_GITHUB_ORG = "SET_SELECTED_GITHUB_ORG";

export function authenticateWithGitHub(token = undefined) {
    const wasInitializedWithToken = !!token;
    token = token || sessionStorage.getItem("gitHubAuthToken");

    return dispatch => {
        if (token) {
            sessionStorage.setItem("gitHubAuthToken", token);

            fetch(`https://api.github.com/user?access_token=${token}`).then(response => {
                if (response["status"] === 401) {
                    // When we get an unauthorized response, out token is no
                    // longer valid, so sign out.
                    dispatch(signOut());
                    return false;
                } else {
                    return response.json();
                }
            }).then(data => {
                dispatch(setCurrentOrigin({ name: data["login"]}));
                dispatch(populateGitHubUserData(data));
                dispatch(attemptSignIn(data["login"]));

                // If we started off with a token, that means we're in the
                // process of signing in and should be redirected home. If not,
                // it means we don't need to redirect
                if (wasInitializedWithToken) { dispatch(goHome()); }
            });
        }
    };
}

export function fetchGitHubRepos() {
    return dispatch => {
        fakeApi.get("github/user/repos.json").then(response => {
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

function populateGitHubUserData(payload) {
    return {
        type: POPULATE_GITHUB_USER_DATA,
        payload,
    };
}

export function requestGitHubAuthToken(params = {}, stateKey = "") {
    return dispatch => {
        if (params["code"] && params["state"] === stateKey) {
            fetch(`${gitHubTokenAuthUrl}/${params["code"]}`).then(response => {
                return response.json();
            }).catch(error => {
                console.error(error);
                dispatch(addNotification({
                    title: "Authentication Failed",
                    body: "Unable to retrieve GitHub token",
                    type: DANGER,
                }));
            }).then(data => {
                if (data["token"]) {
                    dispatch(authenticateWithGitHub(data["token"]));
                } else {
                    dispatch(addNotification({
                        title: "Authentication Failed",
                        body: `[err=${data["code"]}] ${data["msg"]}`,
                        type: DANGER,
                    }));
                }
            });
        }
    };
}

export function setGitHubAuthState() {
    let payload = sessionStorage.getItem("gitHubAuthState") || uuid();
    sessionStorage.setItem("gitHubAuthState", payload);

    return {
        type: SET_GITHUB_AUTH_STATE,
        payload
    };
}

export function setSelectedGitHubOrg(org) {
    return {
        type: SET_SELECTED_GITHUB_ORG,
        payload: org,
    };
}
