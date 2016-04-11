// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import {List, Record} from "immutable";

export default function projects(state = initialState["projects"], action) {
    // Since switch is the main block scope here, define some variables
    // that can be reused below
    let p;

    switch (action.type) {
        // When we're simulating streaming and adding to a build log
        case actionTypes.APPEND_TO_BUILD_LOG:
            p = state.get("current");
            const id = action.payload.buildId;
            return state.setIn(["current", "buildLogs", id],
                (p.buildLogs.get(id) || "") + action.payload.text + "\n");

        // Set a build to successful when its log is done streaming
        case actionTypes.FINISH_BUILD_STREAM:
            p = state.get("current");
            const keyPath = List(["current", "builds",
                p.builds.findIndex(x => x.id === action.payload.buildId)
            ]);
            let build = Object.assign({}, state.getIn(keyPath));

            build.status = "success";
            build.duration = action.payload.duration;

            return state.setIn(keyPath, build);

        case actionTypes.POPULATE_BUILD_LOG:
            return state.setIn(
                ["current", "buildLogs", action.payload.id],
                action.payload.data
            );

        case actionTypes.POPULATE_BUILDS:
            return state.setIn(["current", "builds"],
                List(action.payload));

        case actionTypes.POPULATE_PROJECT:
            return state.set("added", state.get("added").unshift(action.payload));

        case actionTypes.SET_CURRENT_PROJECT:
            return state.mergeIn(["current"], Record(action.payload)());

        case actionTypes.SET_PROJECTS:
            return state.set("all",
                state.get("added").concat(List(action.payload)));

        default:
            return state;
    }
}
