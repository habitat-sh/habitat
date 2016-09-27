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

import * as moment from "moment";
import {requestRoute} from "./actions/index";
import config from "./config";
import {Project} from "./records/Project";
import {AppStore} from "./AppStore";

// Create a GitHub login URL
export function createGitHubLoginUrl(state) {
    const params = {
        client_id: config["github_client_id"],
        redirect_uri: `${window.location.protocol}//${window.location.host}`,
        scope: "user,read:org",
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

// Take some params and return a project
export function projectFromParams(p = {}) {
    let id = undefined;

    if (p["id"]) {
        id = p["id"];
    } else if (p["origin"] && p["name"]) {
        id = `${p["origin"]}/${p["name"]}`;
    }

    return Project({
        id: id,
        plan_path: p["plan_path"]
    });
}

// Compare the identifying attributes of two projects to see if they are the same
export function isProject(x = {}, y = {}) {
    return x["id"] === y["id"];
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
    const text = key;
    const lines = key.trim().split("\n");
    const type = lines[0];
    const name = lines[1] || "";
    const origin = name.split("-")[0]; // TODO: make work for non-origin keys
    const revision = name.split("-")[1];
    const blankLine = lines[2];
    const body = lines[3];

    let uploadPathFragment;
    if (type === "SIG-PUB-1") {
        uploadPathFragment = "keys";
    } else if (type === "SIG-SEC-1") {
        uploadPathFragment = "secret_keys";
    }

    const uploadPath = [origin, uploadPathFragment, revision].join("/");
    const valid = type !== "" && origin !== "" && blankLine.trim() === "" &&
        body !== "";

    return {
        name,
        valid,
        origin,
        text,
        type,
        uploadPath,
    };
}

export function isSignedIn() {
    const store = new AppStore();
    return !!store.getState().gitHub.authToken;
}

// Given a page component, check if the user is signed in and redirect if not
export function requireSignIn(pageComponent) {
    const store = pageComponent.store;
    const hasToken = !!store.getState().gitHub.authToken;

    if (!hasToken) { store.dispatch(requestRoute(["/sign-in"])); }
}
