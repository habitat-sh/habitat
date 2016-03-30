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
