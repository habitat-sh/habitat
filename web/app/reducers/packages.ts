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

export default function packages(state = initialState["packages"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore"], List(action.payload));

        case actionTypes.SET_CURRENT_PACKAGE:
            let p = Object.assign({}, action.payload);
            p.manifest = marked(p.manifest);
            return state.set("current", Package(p));

        case actionTypes.SET_VISIBLE_PACKAGES:
            return state.set("visible", List(action.payload));

        default:
            return state;
    }
}
