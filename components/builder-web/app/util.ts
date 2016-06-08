// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as moment from "moment";
import {requestRoute} from "./actions/index";
import config from "./config";

// Create a GitHub login URL
export function createGitHubLoginUrl(state) {
    const params = {
        client_id: config["github_client_id"],
        redirect_uri: `${window.location.protocol}//${window.location.host}`,
        scope: "user:email,read:org",
        state
    };
    const urlPrefix = "https://github.com/login/oauth/authorize";
    const queryString = Object.keys(params).map((k) =>
        `${k}=${encodeURIComponent(params[k])}`).
        join("&");
    return `${urlPrefix}?${queryString}`;
}

// Pretty print a time
// Print a number of seconds as minutes and seconds
export function duration(s) {
    return moment.utc(s * 1000).format("m [min] s [sec]");
}

// Pretty-printed time
export function friendlyTime(t) {
    return moment(t).fromNow();
}

// get an icon's path
export function icon(x: string): string {
    return `/node_modules/octicons/svg/${x}.svg`;
}

// Compare the identifying attributes of two packages to see if they are the
// same
export function isPackage(x = {}, y = {}) {
    return packageString(x["ident"]) === packageString(y["ident"]);
}

// Take a package and make a string separated by slashes of its identifying
// attributes
export function packageString(o = {}) {
    return ["origin", "name", "version", "release"]
        .map(part => o[part])
        .filter(part => part).join("/");
}

// Take a habitat encryption key and return an object containing data about it
export function parseKey(key) {
    const lines = key.trim().split("\n");
    const type = lines[0];
    const name = lines[1] || "";
    const origin = name.split("-")[0]; // TODO: make work for non-origin keys
    const blankLine = lines[2];
    const body = lines[3];
    const valid = type !== "" && origin !== "" && blankLine.trim() === "" &&
        body !== "";

    return {
        valid,
        origin,
        type,
    };
}

// Given a page component, check if the user is signed in and redirect if not
export function requireSignIn(pageComponent) {
    const store = pageComponent.store;
    const state = store.getState();
    const hasToken = !!state.gitHub.authToken;
    const currentOrigin = state.origins.current.name;

    if (!hasToken) { store.dispatch(requestRoute(["SignIn"])); }

    if (hasToken && !currentOrigin) {
        store.dispatch(requestRoute(["OriginCreate"]));
    }
}
