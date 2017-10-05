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

import "whatwg-fetch";
import { URLSearchParams } from "@angular/http";
import * as cookies from "js-cookie";
import config from "../config";
import { setFeatureFlags } from "./index";
import { attemptSignIn, addNotification, goHome, fetchMyOrigins, fetchMyOriginInvitations, requestRoute, setFeatureFlag,
    signOut, setSigningInFlag } from "./index";
import { DANGER, WARNING } from "./notifications";
import { BuilderApiClient } from "../BuilderApiClient";
import { GitHubApiClient } from "../GitHubApiClient";
import { setBldrSessionToken } from "./sessions";

const parseLinkHeader = require("parse-link-header");
const uuid = require("uuid").v4;
const gitHubTokenAuthUrl = `${config["habitat_api_url"]}/v1/authenticate`;

export const CLEAR_GITHUB_FILES = "CLEAR_GITHUB_FILES";
export const CLEAR_GITHUB_INSTALLATIONS = "CLEAR_GITHUB_INSTALLATIONS";
export const CLEAR_GITHUB_REPOS = "CLEAR_GITHUB_REPOS";
export const LOAD_GITHUB_SESSION_STATE = "LOAD_GITHUB_SESSION_STATE";
export const POPULATE_GITHUB_FILES = "POPULATE_GITHUB_FILES";
export const POPULATE_GITHUB_INSTALLATIONS = "POPULATE_GITHUB_INSTALLATIONS";
export const POPULATE_GITHUB_INSTALLATION_REPOSITORIES = "POPULATE_GITHUB_INSTALLATION_REPOSITORIES";
export const POPULATE_GITHUB_REPOS = "POPULATE_GITHUB_REPOS";
export const POPULATE_GITHUB_USER_DATA = "POPULATE_GITHUB_USER_DATA";
export const SET_GITHUB_AUTH_STATE = "SET_GITHUB_AUTH_STATE";
export const SET_GITHUB_AUTH_TOKEN = "SET_GITHUB_AUTH_TOKEN";
export const SET_GITHUB_ORGS_LOADING_FLAG = "SET_GITHUB_ORGS_LOADING_FLAG";
export const SET_GITHUB_REPOS_LOADING_FLAG = "SET_GITHUB_REPOS_LOADING_FLAG";
export const SET_SELECTED_GITHUB_ORG = "SET_SELECTED_GITHUB_ORG";

export function authenticateWithGitHub(oauth_token = undefined, session_token = undefined) {
    const wasInitializedWithToken = !!oauth_token;

    return dispatch => {
        if (oauth_token) {
            setCookie("gitHubAuthToken", oauth_token);

            fetch(`${config["github_api_url"]}/user?access_token=${oauth_token}`).then(response => {
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
                // When we get an unauthorized response, our token is no
                // longer valid, so sign out.
                dispatch(signOut());
                dispatch(addNotification({
                    title: "GitHub Authorization Failed",
                    body: "Please sign in again.",
                    type: WARNING,
                }));
            });
        }
        if (session_token) {
            setCookie("bldrAuthSessionToken", session_token);
            dispatch(fetchMyOrigins(session_token));
            dispatch(fetchMyOriginInvitations(session_token));
        }
    };
}

export function fetchGitHubFiles(installationId: string, owner: string, repo: string, filename: string) {
    const token = cookies.get("gitHubAuthToken");

    return dispatch => {
        dispatch(clearGitHubFiles());
        const client = new BuilderApiClient(token);

        client.findFileInRepo(installationId, owner, repo, filename)
            .then((results) => {
                dispatch(populateGitHubFiles(results));
            });
    };
};

export function fetchGitHubInstallations() {
    const token = cookies.get("gitHubAuthToken");

    return dispatch => {
        const client = new GitHubApiClient(token);
        dispatch(clearGitHubInstallations());
        dispatch(clearGitHubRepos());

        client.getUserInstallations()
            .then((results) => {
                dispatch(populateGitHubInstallations(results));
            })
            .catch((error) => {
                console.error(error);
            });
    };
};


export function fetchGitHubInstallationRepositories(installationId: string) {
    const token = cookies.get("gitHubAuthToken");

    return dispatch => {
        dispatch(clearGitHubRepos());

        new GitHubApiClient(token).getUserInstallationRepositories(installationId)
            .then((results) => {
                dispatch(populateGitHubInstallationRepositories(results));
            });
    };
};

export function loadGitHubSessionState() {
    return {
        type: LOAD_GITHUB_SESSION_STATE,
        payload: {
            gitHubAuthToken: cookies.get("gitHubAuthToken"),
            gitHubAuthState: cookies.get("gitHubAuthState"),
        },
    };
}

function clearGitHubFiles() {
    return {
        type: CLEAR_GITHUB_FILES
    };
}

function clearGitHubInstallations() {
    return {
        type: CLEAR_GITHUB_INSTALLATIONS
    };
}

function populateGitHubInstallations(payload) {
    return {
        type: POPULATE_GITHUB_INSTALLATIONS,
        payload,
    };
}

function populateGitHubInstallationRepositories(payload) {
    return {
        type: POPULATE_GITHUB_INSTALLATION_REPOSITORIES,
        payload,
    };
}

function populateGitHubRepos(data) {
    return {
        type: POPULATE_GITHUB_REPOS,
        payload: data,
    };
}

function populateGitHubFiles(data) {
    return {
        type: POPULATE_GITHUB_FILES,
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
        cookies.remove("gitHubAuthState", { domain: cookieDomain() });
        cookies.remove("gitHubAuthToken", { domain: cookieDomain() });
        cookies.remove("featureFlags", { domain: cookieDomain() });
        cookies.remove("bldrSessionToken", { domain: cookieDomain() });
    };
}

export function requestGitHubAuthToken(params, stateKey = "") {
    params = new URLSearchParams(params.slice(1));

    return dispatch => {
        if (params.has("code") && params.get("state") === stateKey) {
            fetch(`${gitHubTokenAuthUrl}/${params.get("code")}`).then(response => {
                return response.json();
            }).catch(error => {
                dispatch(addNotification({
                    title: "Authentication Failed",
                    body: "Unable to retrieve GitHub token",
                    type: DANGER,
                }));
            }).then(data => {
                if (data["oauth_token"]) {
                    dispatch(authenticateWithGitHub(data["oauth_token"], data["token"]));
                    dispatch(setGitHubAuthToken(data["oauth_token"]));
                    dispatch(setBldrSessionToken(data["token"]));
                    if (data["flags"]) {
                        dispatch(setFeatureFlag("gitHub", data["flags"]));
                    }
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

function clearGitHubRepos() {
    return {
        type: CLEAR_GITHUB_REPOS,
    };
}

// Return up to two trailing segments of the current hostname
// for purposes of setting the cookie domain unless the domain
// is an IP address.
function cookieDomain() {
    let delim = ".";
    let hostname = currentHostname();
    let tld = hostname.split(delim).pop();

    if (isNaN(Number(tld))) {
        return hostname
            .split(delim)
            .splice(-2)
            .join(delim);
    } else {
        return hostname;
    }
}

export const currentHostname = () => {
    return location.hostname;
};

export function setCookie(key, value) {
    return cookies.set(key, value, {
        domain: cookieDomain(),
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
