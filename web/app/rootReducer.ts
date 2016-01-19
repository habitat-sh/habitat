///<reference path='../node_modules/immutable/dist/immutable.d.ts'/>

import * as Immutable from "immutable";
import * as actionTypes from "./actions";

const initialState = Immutable.Record({
  currentYear: new Date().getFullYear(),
  isUserNavOpen: false,
  isSignedIn: true,
  isSignUpFormSubmitted: false,
  username: "smith",
  email: null,
  password: null,
  route: null,
  packages: [
    {
      identifier: "smith/openssl",
      name: "openssl",
      derivation: "smith",
      maintainer: "Jamie Winsor <reset@chef.io>",
      license: "BSD",
      version: "1.0.2e",
      release: "20160111220549",
      source: "https://www.openssl.org/source/openssl-1.0.2e.tar.gz",
      sha: "e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff",
      buildDependencies: {},
      deps: {
        "chef/glibc": {
          version: "2.19",
          release: "20160111220307",
        },
        "chef/zlib": {
          version: "1.2.8",
          release: "20160111220313",
        },
        "chef/cacerts": {
          version: "2016.01.11",
          release: "20160111220317",
        }
      }
    },
    {
      identifier: "smith/runit",
      name: "runit",
      derivation: "smith",
      description: "It cannot be stopped.",
      version: "2.1.2",
      release: "20160111220840",
      deps: {
        "chef/glibc": {
          version: "2.19",
          release: "20160111220307",
        }
      },
      maintainer: "Joshua Timberman <jtimberman@chef.io>",
      license: "BSD",
      source: "http://smarden.org/runit/runit-2.1.2.tar.gz",
      sha: "6fd0160cb0cf1207de4e66754b6d39750cff14bb0aa66ab49490992c0c47ba18",
      buildDependencies: {},
    },
  ] 
})();

export function rootReducer(state = initialState, action) {
  switch (action.type) {
  case actionTypes.ROUTE_CHANGE:
    return state.set("route", action.payload);
  case actionTypes.SIGN_UP_ATTEMPT:
    return state.
      set("isSignUpFormSubmitted", true).
      set("username", action.payload.username).
      set("email", action.payload.email).
      set("password", action.payload.password);
  case actionTypes.SIGN_IN_ATTEMPT:
    return state.
      set("username", action.payload.username).
      set("isSignedIn", true);
  case actionTypes.SIGN_OUT:
    return state.
      set("isSignUpFormSubmitted", false).
      set("isSignedIn", false);
  case actionTypes.TOGGLE_USER_NAV_MENU:
    return state.set("isUserNavOpen", !state.get("isUserNavOpen"));
  default:
    return state;
  }
}
