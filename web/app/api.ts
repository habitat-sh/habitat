import "whatwg-fetch";

// Get the JSON from a url from the fixtures directory.
export function get(url: string) {
    return fetch(`/fixtures/${url}`).then(response => {
        const url = response.url;

        if (url.endsWith(".json")) {
            return response.json();
        } else if (url.endsWith(".txt")) {
            return response.text();
        }
        debugger;

    });
};
