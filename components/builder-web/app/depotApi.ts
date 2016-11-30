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
import config from "./config";
import {packageString} from "./util";

const urlPrefix = config["habitat_api_url"] || "";

export function getUnique(origin: string, nextRange: number = 0) {
    const url = `${urlPrefix}/depot/${origin}/pkgs?range=${nextRange}`;

    return new Promise((resolve, reject) => {
        fetch(url).then(response => {
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }

            response.json().then(resultsObj => {
                let results;

                const endRange = parseInt(resultsObj.range_end, 10);
                const totalCount = parseInt(resultsObj.total_count, 10);
                const nextRange = totalCount > (endRange + 1) ? endRange + 1 : 0;

                if (resultsObj["package_list"]) {
                    results = resultsObj["package_list"];
                } else {
                    results = resultsObj;
                }

                resolve({ results, totalCount, nextRange });
            });
        }).catch(error => reject(error));
    });
}

export function get(params, nextRange: number = 0) {
    const url = `${urlPrefix}/depot/pkgs/` +
        (params["query"] ? `search/${params["query"]}`
                           : packageString(params)) +
        `?range=${nextRange}`;

    return new Promise((resolve, reject) => {
        fetch(url).then(response => {
            // Fail the promise if an error happens.
            //
            // If we're hitting the fake api, the 4xx response will show up
            // here, but if we're hitting the real depot, it will show up in the
            // catch below.
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }

            response.json().then(resultsObj => {
                let results;

                const endRange = parseInt(resultsObj.range_end, 10);
                const totalCount = parseInt(resultsObj.total_count, 10);
                const nextRange = totalCount > (endRange + 1) ? endRange + 1 : 0;

                if (resultsObj["package_list"]) {
                    results = resultsObj["package_list"];
                } else {
                    results = resultsObj;
                }

                resolve({ results, totalCount, nextRange });
            });

        }).catch(error => reject(error));
    });
};
