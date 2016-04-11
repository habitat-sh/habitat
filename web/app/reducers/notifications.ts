// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function notifications(state = initialState["notifications"], action) {
    switch (action.type) {
        case actionTypes.ADD_NOTIFICATION:
            return state.set("all",
                state.get("all").push(action.payload));

        case actionTypes.REMOVE_NOTIFICATION:
            return state.set("all",
                state.get("all").delete(action.payload));

        default:
            return state;
    }
}
