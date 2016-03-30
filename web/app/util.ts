import * as moment from "moment";

// Pretty print a time
// Print a number of seconds as minutes and seconds
export function duration(s) {
    return moment.utc(s * 1000).format("m [min] s [sec]");
}

// Pretty-printed time
export function friendlyTime(t) {
    return moment(t).fromNow();
}

// get an icon's path
export function icon(x: string): string {
    return `/node_modules/octicons/svg/${x}.svg`;
}

// Compare the identifying attributes of two packages to see if they are the
// same
export function isPackage(x = {}, y = {}) {
    return packageString(x["ident"]) === packageString(y["ident"]);
}

// Take a package and make a string separated by slashes of its identifying
// attributes
export function packageString(o = {}) {
    return ["origin", "name", "version", "release"]
        .map(part => o[part])
        .filter(part => part).join("/");
}
