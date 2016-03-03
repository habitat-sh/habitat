// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function orgs(state = initialState["orgs"], action) {
    switch (action.type) {

        default:
            return state;
    }
}