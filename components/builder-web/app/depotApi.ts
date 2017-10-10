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
import config from "./config";
import { packageString } from "./util";
import { AppStore } from "./AppStore";
import { requestRoute, addNotification } from "./actions/index";
import { WARNING } from "./actions/notifications";

const urlPrefix = `${config["habitat_api_url"]}/v1` || "v1";

function opts() {
    const store = new AppStore();
    let token = store.getState().session.token;
    let o: any = {};

    if (token) {
        o.headers = {
            Authorization: `Bearer ${token}`
        };
    }

    return o;
}

function handleError(error, reject) {
    const store = new AppStore();
    const state = store.getState();
    store.dispatch(requestRoute(["/sign-in"]));
    reject(error);

    if (state.session.token) {
        setTimeout(() => {
            store.dispatch(addNotification({
                title: "Session Expired",
                body: "Please sign in again.",
                type: WARNING
            }));
        }, 1000);
    }
}

export function getUnique(origin: string, nextRange: number = 0, token: string = "") {
    const url = `${urlPrefix}/depot/${origin}/pkgs?range=${nextRange}`;

    return new Promise((resolve, reject) => {
        fetch(url, opts())
        .then(response => {
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }
            else {
                response.json().then(resultsObj => {
                    let results;

                    const endRange = parseInt(resultsObj.range_end, 10);
                    const totalCount = parseInt(resultsObj.total_count, 10);
                    const nextRange = totalCount > (endRange + 1) ? endRange + 1 : 0;

                    if (resultsObj["data"]) {
                        results = resultsObj["data"];
                    } else {
                        results = resultsObj;
                    }

                    resolve({ results, totalCount, nextRange });
                });
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function getLatest(origin: string, pkg: string) {
    const url = `${urlPrefix}/depot/pkgs/${origin}/${pkg}/latest`;

    return new Promise((resolve, reject) => {
        fetch(url, opts())
        .then(response => {
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }
            else {
                response.json().then(results => {
                    resolve(results);
                });
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function getLatestInChannel(origin: string, name: string, channel: string, version: string = undefined) {
    const url = `${urlPrefix}/depot/channels/${origin}/${channel}/pkgs/${name}/${version ? version + "/" : ""}latest`;

    return new Promise((resolve, reject) => {
        fetch(url, opts())
        .then(response => {
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }
            else {
                response.json().then(results => {
                    resolve(results);
                });
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function get(params, nextRange: number = 0) {
    let url = `${urlPrefix}/depot/pkgs/` +
        (params["query"] ? `search/${params["query"]}`
                           : packageString(params)) +
        `?range=${nextRange}`;

    if (params["distinct"]) {
        url += "&distinct=true";
    }

    return new Promise((resolve, reject) => {
        fetch(url, opts())
        .then(response => {
            // Fail the promise if an error happens.
            //
            // If we're hitting the fake api, the 4xx response will show up
            // here, but if we're hitting the real Builder, it will show up in the
            // catch below.
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }
            else {
                response.json().then(resultsObj => {
                    let results;

                    const endRange = parseInt(resultsObj.range_end, 10);
                    const totalCount = parseInt(resultsObj.total_count, 10);
                    const nextRange = totalCount > (endRange + 1) ? endRange + 1 : 0;

                    if (resultsObj["data"]) {
                        results = resultsObj["data"];
                    } else {
                        results = resultsObj;
                    }

                    resolve({ results, totalCount, nextRange });
                });
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function getPackageVersions(origin: string, pkg: string) {
    const url = `${urlPrefix}/depot/pkgs/${origin}/${pkg}/versions`;

    return new Promise((resolve, reject) => {
        fetch(url, opts())
        .then(response => {
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }
            else {
                response.json().then(results => {
                    resolve(results);
                });
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function submitJob(origin: string, pkg: string, token: string) {
    const url = `${urlPrefix}/depot/pkgs/schedule/${origin}/${pkg}`;

    return new Promise((resolve, reject) => {
        fetch(url, {
            headers: {
                "Authorization": `Bearer ${token}`,
            },
            method: "POST",
        })
        .then(response => {
            if (response.ok) {
                resolve(true);
            } else {
                reject(new Error(response.statusText));
            }
        })
        .catch(error => handleError(error, reject));
    });
}

export function getStats(origin: string) {
    const url = `${urlPrefix}/depot/pkgs/origins/${origin}/stats`;

    return new Promise((resolve, reject) => {
        fetch(url)
        .then(response => {
            if (response.ok) {
                response.json().then(data => resolve(data));
            } else {
                reject(new Error(response.statusText));
            }
        })
        .catch(error => handleError(error, reject));
    });
}
