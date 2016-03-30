// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as marked from "marked";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import {Package} from "../records/Package";
import {fromJS, List, Record} from "immutable";
import query from "../query";

export default function packages(state = initialState["packages"], action) {
    let p, q;

    switch (action.type) {
        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore"], List(action.payload));

        // Query the list of packages to set the currentPackage data.
        case actionTypes.SET_CURRENT_PACKAGE:
            p = Object.assign({}, action.payload);
            p.manifest = marked(p.manifest);
            return state.set("current", Package(p));

        case actionTypes.SET_PACKAGES:
            return state.set("all", action.payload);

        case actionTypes.SET_VISIBLE_PACKAGES:
            q = query(state.get("all"));
            if (action.payload.filter === "mine") {
                p = q.allMostRecentForOrigin("smith");
            } else if (action.payload.origin) {
                p = q.allMostRecentForOrigin(action.payload.origin);
            } else if (action.payload.name) {
                p = q.allForNameByStars(action.payload.name);
            } else {
                p = q.allMostRecent();
            }
            return state.set("visible", p.toArray());

        default:
            return state;
    }
}
