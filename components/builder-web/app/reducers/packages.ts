// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as marked from "marked";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import {Package} from "../records/Package";
import {fromJS, List, Record} from "immutable";

export default function packages(state = initialState["packages"], action) {
    switch (action.type) {
        case actionTypes.CLEAR_PACKAGES:
            return state.set("current", Package()).
                set("nextRange", 0).
                set("visible", List()).
                set("totalCount", 0).
                setIn(["ui", "current", "loading"], true).
                setIn(["ui", "current", "exists"], false).
                setIn(["ui", "visible", "loading"], true).
                setIn(["ui", "visible", "exists"], false);

        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore"], List(action.payload));

        case actionTypes.SET_CURRENT_PACKAGE:
            if (action.error) {
                return state.set("current", Package()).
                    setIn(["ui", "current", "errorMessage"],
                    action.error.message).
                    setIn(["ui", "current", "loading"], false).
                    setIn(["ui", "current", "exists"], false);
            } else {
                let p = Object.assign({}, action.payload);
                p.manifest = marked(p.manifest);
                return state.set("current", Package(p)).
                    setIn(["ui", "current", "errorMessage"], undefined).
                    setIn(["ui", "current", "exists"], true).
                    setIn(["ui", "current", "loading"], false);
            }

        case actionTypes.SET_PACKAGES_NEXT_RANGE:
            return state.set("nextRange", action.payload);

        case actionTypes.SET_PACKAGES_SEARCH_QUERY:
            return state.set("searchQuery", action.payload);

        case actionTypes.SET_PACKAGES_TOTAL_COUNT:
            return state.set("totalCount", action.payload);

        case actionTypes.SET_VISIBLE_PACKAGES:
            if (action.error) {
                return state.set("visible", List()).
                    setIn(["ui", "visible", "errorMessage"],
                    action.error.message).
                    setIn(["ui", "visible", "exists"], false).
                    setIn(["ui", "visible", "loading"], false);
            } else {
                return state.set("visible",
                    state.get("visible").concat(List(action.payload))).
                    setIn(["ui", "visible", "errorMessage"], undefined).
                    setIn(["ui", "visible", "exists"], true).
                    setIn(["ui", "visible", "loading"], false);
            }

        default:
            return state;
    }
}
