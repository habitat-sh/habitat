// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import {URLSearchParams} from "angular2/http";
import config from "../config";
import {attemptSignIn, addNotification, goHome, requestRoute, setCurrentOrigin,
    setSigningInFlag, signOut} from "./index";
import {DANGER} from "./notifications";

const parseLinkHeader = require("parse-link-header");
const uuid = require("node-uuid").v4;
const gitHubTokenAuthUrl = config["github_token_auth_url"];

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
            sessionStorage.setItem("gitHubAuthToken", token);

            fetch(`https://api.github.com/user?access_token=${token}`).then(response => {
                dispatch(setSigningInFlag(false));

                if (response["status"] === 401) {
                    // When we get an unauthorized response, out token is no
                    // longer valid, so sign out.
                    dispatch(signOut());
                    return false;
                } else {
                    return response.json();
                }
            }).then(data => {
                dispatch(populateGitHubUserData(data));
                dispatch(attemptSignIn(data["login"]));
            });
        }
    };
}

export function fetchGitHubOrgs(page = 1) {
    const token = sessionStorage.getItem("gitHubAuthToken");

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
    const token = sessionStorage.getItem("gitHubAuthToken");
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

export function loadSessionState(sessionStorage) {
    return {
        type: LOAD_SESSION_STATE,
        payload: {
            gitHubAuthToken: sessionStorage.getItem("gitHubAuthToken"),
            gitHubAuthState: sessionStorage.getItem("gitHubAuthState"),
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
        sessionStorage.removeItem("gitHubAuthState");
        sessionStorage.removeItem("gitHubAuthToken");
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

export function setGitHubAuthState() {
    let payload = sessionStorage.getItem("gitHubAuthState") || uuid();
    sessionStorage.setItem("gitHubAuthState", payload);

    return {
        type: SET_GITHUB_AUTH_STATE,
        payload
    };
}

export function setGitHubAuthToken(payload) {
    sessionStorage.setItem("gitHubAuthToken", payload);

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
