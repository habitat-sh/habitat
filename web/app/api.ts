import "whatwg-fetch";
import config from "./config";

debugger;
const urlPrefix = config["depotUrl"] || "";

// Get the JSON from a url from the fixtures directory.
export function get(url: string) {
    return fetch(`${urlPrefix}/fixtures/${url}`).then(response => {
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
