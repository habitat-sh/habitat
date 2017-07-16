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

import * as marked from "marked";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import { Package } from "../records/Package";
import { fromJS, List, Record } from "immutable";

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

        case actionTypes.POPULATE_DASHBOARD_RECENT:
            return state.setIn(["dashboard", "recent"], List(action.payload));

        case actionTypes.CLEAR_PACKAGE_CHANNELS:
            return state.setIn(["current", "channels"], []);

        case actionTypes.CLEAR_PACKAGE_VERSIONS:
            return state.set("versions", undefined);

        case actionTypes.POPULATE_EXPLORE:
            return state.setIn(["explore", "popular"], List(action.payload["popular"])).
            setIn(["explore", "your_app"], List(action.payload["your_app"])).
            setIn(["explore", "community"], List(action.payload["community"]));

        case actionTypes.POPULATE_EXPLORE_STATS:
            return state.setIn(["explore", "stats"], Record(action.payload)());

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

        case actionTypes.SET_CURRENT_PACKAGE_CHANNELS:
            if (action.error) {
                return state.setIn(["current", "channels"], []);
            } else {
                return state.setIn(["current", "channels"], action.payload);
            }

        case actionTypes.SET_CURRENT_PACKAGE_VERSIONS:
            if (action.error) {
                return state.set("versions", undefined).
                    setIn(["ui", "versions", "errorMessage"],
                    action.error.message).
                    setIn(["ui", "versions", "loading"], false).
                    setIn(["ui", "versions", "exists"], false);
            } else {
                return state.set("versions", action.payload).
                    setIn(["ui", "versions", "errorMessage"], undefined).
                    setIn(["ui", "versions", "exists"], true).
                    setIn(["ui", "versions", "loading"], false);
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

        case actionTypes.SET_VISIBLE_PACKAGE_CHANNELS:
            let visible = state.get("visible");

            let i = visible.findIndex(p => {
                return action.payload.pkg.origin === p.origin &&
                    action.payload.pkg.name === p.name &&
                    action.payload.pkg.version === p.version &&
                    action.payload.pkg.release === p.release;
            });

            if (i >= 0) {
                let pkg = visible.get(i);
                pkg.channels = action.payload.channels;
                return state.setIn(["visible", i], pkg);
            }
            else {
                return state;
            }

        default:
            return state;
    }
}
