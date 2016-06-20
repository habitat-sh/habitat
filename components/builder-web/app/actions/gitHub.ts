// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import "whatwg-fetch";
import {URLSearchParams} from "angular2/http";
import * as cookies from "js-cookie";
import config from "../config";
import {attemptSignIn, addNotification, goHome, requestRoute, setSigningInFlag,
    signOut} from "./index";
import {DANGER, WARNING} from "./notifications";

const parseLinkHeader = require("parse-link-header");
const uuid = require("node-uuid").v4;
const gitHubTokenAuthUrl = `${config["habitat_api_url"]}/authenticate`;

export const LOAD_SESSION_STATE = "LOAD_SESSION_STATE";
export const POPULATE_GITHUB_ORGS = "POPULATE_GITHUB_ORGS";
export const POPULATE_GITHUB_REPOS = "POPULATE_GITHUB_REPOS";
export const POPULATE_GITHUB_USER_DATA = "POPULATE_GITHUB_USER_DATA";
export const RESET_GITHUB_ORGS = "RESET_GITHUB_ORGS";
export const RESET_GITHUB_REPOS = "RESET_GITHUB_REPOS";
export const SET_GITHUB_AUTH_STATE = "SET_GITHUB_AUTH_STATE";
export const SET_GITHUB_AUTH_TOKEN = "SET_GITHUB_AUTH_TOKEN";
export const SET_GITHUB_ORGS_LOADING_FLAG = "SET_GITHUB_ORGS_LOADING_FLAG";
export const SET_GITHUB_REPOS_LOADING_FLAG = "SET_GITHUB_REPOS_LOADING_FLAG";
export const SET_SELECTED_GITHUB_ORG = "SET_SELECTED_GITHUB_ORG";

export function authenticateWithGitHub(token = undefined) {
    const wasInitializedWithToken = !!token;
    const isCodeInQueryString = new URLSearchParams(
        window.location.search.slice(1)
    ).has("code");

    return dispatch => {
        if (isCodeInQueryString) {
            dispatch(setSigningInFlag(true));
        }

        if (token) {
            setCookie("gitHubAuthToken", token);

            fetch(`https://api.github.com/user?access_token=${token}`).then(response => {
                dispatch(setSigningInFlag(false));

                if (response.ok) {
                    return response.json();
                } else {
                    // If the response is not ok, throw an error from the
                    // promise to be handled below.
                    return response.json().then(error => { throw error; });
                }
            }).then(data => {
                dispatch(populateGitHubUserData(data));
                dispatch(attemptSignIn(data["login"]));
            }).catch(error => {
                // We can assume an error from the response is a 401; anything
                // else is probably a transient failure on GitHub's end, which
                // we can expect to clear when we try to sign in again.
                //
                // When we get an unauthorized response, out token is no
                // longer valid, so sign out.
                dispatch(signOut());
                dispatch(addNotification({
                    title: "GitHub Authorization Failed",
                    body: "Please sign in again.",
                    type: WARNING,
                }));
            });
        }
    };
}

export function fetchGitHubOrgs(page = 1) {
    const token = cookies.get("gitHubAuthToken");

    return dispatch => {
        fetch(`https://api.github.com/user/orgs?access_token=${token}&per_page=100&page=${page}`).then(response => {
            const links = parseLinkHeader(response.headers.get("Link"));

            // When we get the first page, clear everything out
            if (page === 1) { dispatch(resetGitHubOrgs()); }

            if (links && links.next && links.next.page) {
                dispatch(setGitHubOrgsLoadingFlag(true));
                dispatch(fetchGitHubOrgs(links.next.page));
            } else {
                dispatch(setGitHubOrgsLoadingFlag(false));
            }

            response.json().then(data => dispatch(populateGitHubOrgs(data)));
        });
    };
};

export function fetchGitHubRepos(org, page = 1, username) {
    const token = cookies.get("gitHubAuthToken");
    const urlPath = username ? `users/${username}/repos` : `orgs/${org}/repos`;

    return dispatch => {
        if (page === 1) {
            dispatch(setGitHubReposLoadingFlag(true));
        }

        fetch(`https://api.github.com/${urlPath}?access_token=${token}&per_page=100&page=${page}`).then(response => {
            const links = parseLinkHeader(response.headers.get("Link"));

            // When we get the first page, clear everything out
            if (page === 1) { dispatch(resetGitHubRepos()); }

            if (links && links.next && links.next.page) {
                dispatch(fetchGitHubRepos(org, links.next.page, username));
            } else {
                dispatch(setGitHubReposLoadingFlag(false));
            }

            response.json().then(data => dispatch(populateGitHubRepos(data)));
        });
    };
}

export function loadSessionState() {
    return {
        type: LOAD_SESSION_STATE,
        payload: {
            gitHubAuthToken: cookies.get("gitHubAuthToken"),
            gitHubAuthState: cookies.get("gitHubAuthState"),
        },
    };
}

export function onGitHubOrgSelect(org, username) {
    return dispatch => {
        dispatch(setSelectedGitHubOrg(org));
        dispatch(fetchGitHubRepos(org, 1, username));
    };
}

export function onGitHubRepoSelect(repo) {
    return dispatch => {
        dispatch(requestRoute(
            ["ProjectCreate", { repo: encodeURIComponent(repo) }]
        ));
    };
}

function populateGitHubOrgs(payload) {
    return {
        type: POPULATE_GITHUB_ORGS,
        payload,
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

export function removeSessionStorage() {
    return dispatch => {
        cookies.remove("gitHubAuthState");
        cookies.remove("gitHubAuthToken");
    };
}

export function requestGitHubAuthToken(params, stateKey = "") {
    params = new URLSearchParams(params.slice(1));

    return dispatch => {
        if (params.has("code") && params.get("state") === stateKey) {
            fetch(`${gitHubTokenAuthUrl}/${params.get("code")}`).then(response => {
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
                    dispatch(setGitHubAuthToken(data["token"]));
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

function resetGitHubOrgs() {
    return {
        type: RESET_GITHUB_ORGS,
    };
}

function resetGitHubRepos() {
    return {
        type: RESET_GITHUB_REPOS,
    };
}

function setCookie (key, value) {
    return cookies.set(key, value, {
        secure: window.location.protocol === "https"
    });
}

export function setGitHubAuthState() {
    let payload = cookies.get("gitHubAuthState") || uuid();
    setCookie("gitHubAuthState", payload);

    return {
        type: SET_GITHUB_AUTH_STATE,
        payload
    };
}

export function setGitHubAuthToken(payload) {
    setCookie("gitHubAuthToken", payload);

    return {
        type: SET_GITHUB_AUTH_TOKEN,
        payload
    };
}

function setGitHubOrgsLoadingFlag(payload) {
    return {
        type: SET_GITHUB_ORGS_LOADING_FLAG,
        payload,
    };
}

function setGitHubReposLoadingFlag(payload) {
    return {
        type: SET_GITHUB_REPOS_LOADING_FLAG,
        payload,
    };
}

export function setSelectedGitHubOrg(org) {
    return {
        type: SET_SELECTED_GITHUB_ORG,
        payload: org,
    };
}
