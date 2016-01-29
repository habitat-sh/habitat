import "whatwg-fetch";

// Get the JSON from a url from the fixtures directory.
export function get(url: string) {
    return fetch(`/fixtures/${url}.json`).then(response => {
        return response.json();
    });
};
