// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import * as actionTypes from "./actions/index";
import {List, Record} from "immutable";
import initialState from "./initialState";
import query from "./query";

export function rootReducer(state = initialState, action) {
    // Since switch is the main block scope here, define some variables
    // that can be reused below
    let p, q;

    switch (action.type) {

        case actionTypes.ADD_NOTIFICATION:
            return state.set("notifications",
                state.get("notifications").push(action.payload));

        // When we're simulating streaming and adding to a build log
        case actionTypes.APPEND_TO_BUILD_LOG:
            p = state.get("currentProject");
            const id = action.payload.buildId;
            return state.setIn(["currentProject", "buildLogs", id],
                (p.buildLogs.get(id) || "") + action.payload.text + "\n");

        // Set a build to successful when its log is done streaming
        case actionTypes.FINISH_BUILD_STREAM:
            p = state.get("currentProject");
            const keyPath = List(["currentProject", "builds",
                p.builds.findIndex(x => x.id === action.payload.buildId)
            ]);
            let build = Object.assign({}, state.getIn(keyPath));

            build.status = "success";
            build.duration = action.payload.duration;

            return state.setIn(keyPath, build);

        case actionTypes.POPULATE_BUILD_LOG:
            return state.setIn(
                ["currentProject", "buildLogs", action.payload.id],
                action.payload.data
            );

        case actionTypes.POPULATE_BUILDS:
            return state.setIn(["currentProject", "builds"],
                List(action.payload));

        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore", "packages"], List(action.payload));

        case actionTypes.POPULATE_PROJECT:
            return state.set("addedProjects", state.get("addedProjects").unshift(action.payload));

        case actionTypes.REMOVE_NOTIFICATION:
            return state.set("notifications",
                state.get("notifications").delete(action.payload));

        case actionTypes.ROUTE_CHANGE:
            return state.set("route", action.payload).
                set("requestedRoute", null);

        case actionTypes.ROUTE_REQUESTED:
            return state.
                set("requestedRoute", action.payload);

        // Query the list of packages to set the currentPackage data.
        case actionTypes.SET_CURRENT_PACKAGE:
            q = query(state.get("packages"));
            p = null;
            const pkgEnumerable = q.fromParams(action.payload);

            if (pkgEnumerable.count() > 0) {
                p = pkgEnumerable.first();
                p.versions = q.allReleasesForPackageVersion(p).toArray();
                p.releases = q.allVersionsForPackage(p).toArray();
                p.dependencies = p.dependencies || [];
                p.buildDependencies = p.buildDependencies || [];
            }

            return state.set("currentPackage", p);

        case actionTypes.SET_CURRENT_PROJECT:
            return state.mergeIn(["currentProject"], Record(action.payload)());

        case actionTypes.SET_PACKAGES:
            return state.set("packages", action.payload);

        case actionTypes.SET_PROJECTS:
            return state.set("projects",
                state.get("addedProjects").concat(List(action.payload)));

        case actionTypes.SET_VISIBLE_PACKAGES:
            q = query(state.get("packages"));
            if (action.payload.filter === "mine") {
                p = q.allMostRecentForOrigin("smith");
            } else if (action.payload.origin) {
                p = q.allMostRecentForOrigin(action.payload.origin);
            } else if (action.payload.name) {
                p = q.allForNameByStars(action.payload.name);
            } else {
                p = q.allMostRecent();
            }
            return state.set("visiblePackages", p.toArray());

        case actionTypes.SIGN_IN_ATTEMPT:
            return state.
                set("username", action.payload.username).
                set("isSignedIn", true);

        case actionTypes.SIGN_OUT:
            return state.
                set("isSignUpFormSubmitted", false).
                set("isSignedIn", false);

        case actionTypes.SIGN_UP_ATTEMPT:
            return state.
                set("isSignUpFormSubmitted", true).
                set("username", action.payload.username).
                set("email", action.payload.email).
                set("password", action.payload.password);

        case actionTypes.TOGGLE_USER_NAV_MENU:
            return state.set("isUserNavOpen", !state.get("isUserNavOpen"));

        default:
            return state;
    }
}
