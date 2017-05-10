// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import {List, Record} from "immutable";

export default function projects(state = initialState["projects"], action) {
    // Since switch is the main block scope here, define some variables
    // that can be reused below
    let p;

    switch (action.type) {

        case actionTypes.POPULATE_PROJECT:
            let padded = state.get("added");

            if (padded.size === 0) {
                return state.set("added", padded.unshift(action.payload));
            } else {
                let index = padded.findIndex(proj => { return proj["id"] === action.payload["id"]; });

                if (index === -1) {
                    return state.set("added", padded.unshift(action.payload));
                }
            }

        case actionTypes.SET_CURRENT_PROJECT:
            return state.mergeIn(["current"], Record(action.payload)());

        case actionTypes.SET_PROJECT_HINT:
            return state.set("hint", action.payload);

        case actionTypes.RESET_PROJECT_HINT:
            return state.set("hint", {});

        case actionTypes.DELETE_PROJECT:
            return state.remove("current");

        case actionTypes.DEPOPULATE_PROJECT:
            let added = state.get("added");
            let index = added.findIndex(proj => { return proj["id"] === action.payload; });

            if (index === -1) {
                return state.set("added", added);
            } else {
                return state.set("added", added.delete(index));
            }

        case actionTypes.SET_PROJECTS:
            return state.set("all",
                state.get("added").concat(List(action.payload)));

        default:
            return state;
    }
}
