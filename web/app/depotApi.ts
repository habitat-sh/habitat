import "whatwg-fetch";
import config from "./config";
import {packageString} from "./util";

const urlPrefix = config["depotUrl"] || "";

// Get the JSON from a url from the fixtures directory.
export function get(ident) {
    return fetch(`${urlPrefix}/pkgs/${packageString(ident)}`).then(response => {
        const url = response.url;

        // Fail the promise if an error happens.
        if (response.status >= 400) {
            return Promise.reject(new Error(response.statusText));
        }

        return response.json();
    });
};
