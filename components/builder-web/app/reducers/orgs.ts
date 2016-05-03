// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
