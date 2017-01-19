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

import {List, Record} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

// Record structure for a blank org
const Org = Record({
    namespace: undefined,
    name: undefined,
    email: undefined,
    website: undefined,
    members: List(),
});

const User = Record({
    username: undefined,
    name: undefined,
    email: undefined,
    status: "",
    canBeAdded: true,
    ui: Record({
        isActionsMenuOpen: false
    })(),
});

export default function orgs(state = initialState["orgs"], action) {
    switch (action.type) {
        case actionTypes.FINISH_CREATING_ORG:
            return state.mergeIn(
                ["current"], state.get("current").clear()
            ).setIn(["ui", "create", "saved"], false);

        case actionTypes.ORG_INVITATION_CANCELLED:
            return state.setIn(["current", "members"],
                state.getIn(["current", "members"]).delete(action.payload));

        case actionTypes.ORG_INVITATION_CREATED:
            return state.
                setIn(
                ["current", "memberSearchResults", action.payload.index,
                    "canBeAdded"],
                false
                ).
                setIn(
                ["current", "memberSearchResults", action.payload.index,
                    "status"],
                "Invitation Sent"
                ).
                setIn(
                ["current", "members"],
                state.getIn(["current", "members"]).push(
                    User(action.payload.member).set("status", "Invitation sent")
                )
                ).
                setIn(
                ["current", "memberSearchResults"],
                List()
                );

        case actionTypes.PERFORM_ORG_MEMBER_SEARCH:
            if (action.payload === "") {
                return state.setIn(["current", "memberSearchResults"], List());
            } else {
                return state.setIn(["current", "memberSearchResults"],
                    state.getIn(["current", "availableMemberSearchResults"]));
            }

        case actionTypes.POPULATE_ORG:
            return state.mergeIn(["current"], Org(action.payload)).
                setIn(["ui", "create", "saved"], true).
                set("added",
                state.get("added").push(Org(action.payload))
                ).
                set("all",
                state.get("all").concat(
                    state.get("added").push(Org(action.payload))
                )
                );

        case actionTypes.TOGGLE_MEMBER_ACTION_MENU:
            const keyPath = ["current", "members", action.payload, "ui",
                "isActionsMenuOpen"];

            return state.setIn(keyPath, !state.getIn(keyPath));

        default:
            return state;
    }
}
