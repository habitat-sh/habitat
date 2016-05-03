// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";

// Get the JSON from a url from the fixtures directory.
export function get(url: string) {
    return fetch(`/fixtures/${url}`).then(response => {
        const url = response.url;

        // Fail the promise if an error happens.
        if (response.status >= 400) {
            return Promise.reject(new Error(response.statusText));
        }

        if (url.endsWith(".json")) {
            return response.json();
        } else if (url.endsWith(".txt")) {
            return response.text();
        }
    });
};
