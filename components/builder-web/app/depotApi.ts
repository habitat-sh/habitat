// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import config from "./config";
import {packageString} from "./util";

const urlPrefix = config["habitat_api_url"] || "";

export function get(ident) {
    return new Promise((resolve, reject) => {
        fetch(`${urlPrefix}/depot/pkgs/${packageString(ident)}`).then(response => {
            // Fail the promise if an error happens.
            //
            // If we're hitting the fake api, the 4xx response will show up
            // here, but if we're hitting the real depot, it will show up in the
            // catch below.
            if (response.status >= 400) {
                reject(new Error(response.statusText));
            }

            resolve(response.json());
        }).catch(error => reject(error));
    });
};
