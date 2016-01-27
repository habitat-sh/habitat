// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

///<reference path='../node_modules/immutable/dist/immutable.d.ts'/>

import * as Immutable from "immutable";
import * as actionTypes from "./actions";
import packages from "../fixtures/packages.ts";

const initialState = Immutable.Record({
  appName: "bldr",
  currentYear: new Date().getFullYear(),
  email: null,
  isSignUpFormSubmitted: false,
  isSignedIn: true,
  isUserNavOpen: false,
  packages,
  password: null,
  requestedRoute: null,
  route: null,
  username: "smith",
})();

export function rootReducer(state = initialState, action) {
  switch (action.type) {
  case actionTypes.ROUTE_CHANGE:
    return state.set("route", action.payload).
      set("requestedRoute", null);
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
  case actionTypes.ROUTE_REQUESTED:
    return state.set("requestedRoute", action.payload);
  case actionTypes.TOGGLE_USER_NAV_MENU:
    return state.set("isUserNavOpen", !state.get("isUserNavOpen"));
  default:
    return state;
  }
}
