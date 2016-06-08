// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {List} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function origins(state = initialState["origins"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_MY_ORIGINS:
            return state.setIn(["mine"], List(action.payload));

        case actionTypes.SET_CURRENT_ORIGIN:
            return state.setIn(["current", "name"], action.payload.name);

        case actionTypes.SET_CURRENT_ORIGIN_CREATING_FLAG:
            return state.setIn(["ui", "current", "creating"], action.payload);

        case actionTypes.SET_ORIGIN_ADDING_PRIVATE_KEY:
            return state.setIn(["ui", "current", "addingPrivateKey"],
                action.payload);

        case actionTypes.SET_ORIGIN_ADDING_PUBLIC_KEY:
            return state.setIn(["ui", "current", "addingPublicKey"],
                action.payload);

        case actionTypes.TOGGLE_ORIGIN_PICKER:
            return state.setIn(["ui", "isPickerOpen"],
                !state.getIn(["ui", "isPickerOpen"]));

        default:
            return state;
    }
}
