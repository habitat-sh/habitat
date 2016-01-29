// Compare the identifying attributes of two packages to see if they are the
// same
export function isPackage(x = {}, y = {}) {
  return packageString(x) === packageString(y);
};

// Take a package and make a string separated by slashes of its identifying
// attributes
export function packageString (o = {}) {
  return `${o["derivation"]}/${o["name"]}/${o["version"]}/${o["release"]}`;
};
