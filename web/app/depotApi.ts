import "whatwg-fetch";
import config from "./config";
import {packageString} from "./util";

const urlPrefix = config["depot_url"] || "";

// Get the JSON from a url from the fixtures directory.
export function get(ident) {
    return new Promise((resolve, reject) => {
        fetch(`${urlPrefix}/pkgs/${packageString(ident)}`).then(response => {
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
