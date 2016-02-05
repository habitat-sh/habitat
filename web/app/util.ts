// Compare the identifying attributes of two packages to see if they are the
// same
export function isPackage(x = {}, y = {}) {
    return packageString(x) === packageString(y);
}

// Take a package and make a string separated by slashes of its identifying
// attributes
export function packageString(o = {}) {
    return ["derivation", "name", "version", "release"]
        .map(part => o[part])
        .filter(part => part).join("/");
}

// Given a string that looks like "pkgs?show=mine", parse it into an object
// that behaves like window.location.
//
// This is needed because the route object we have gives the same value
// regardless of whether or not we use the HashLocationStrategy. This function
// lets us treat the url the same independent of which strategy we use.
export function url(path) {
    let a = window.document.createElement("a");
    a.href = `/${path}`;
    return { pathname: a.pathname, search: a.search };
}
