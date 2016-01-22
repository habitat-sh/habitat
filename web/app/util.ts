export function packageString (o) {
  return `${o["derivation"]}/${o["name"]}/${o["version"]}/${o["release"]}`;
};
