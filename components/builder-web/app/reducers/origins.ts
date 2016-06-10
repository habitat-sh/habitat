// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {List} from "immutable";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import {Origin} from "../records/Origin";

export default function origins(state = initialState["origins"], action) {
    switch (action.type) {
        case actionTypes.POPULATE_MY_ORIGINS:
            if (action.error) {
                return state.setIn(["mine"], List()).
                    setIn(["ui", "mine", "errorMessage"], action.error.message).
                    setIn(["ui", "mine", "loading"], false);
            } else {
                return state.setIn(["mine"], List(action.payload.map(name =>
                    Origin({ name })
                ))).setIn(["ui", "mine", "errorMessage"], undefined).
                    setIn(["ui", "mine", "loading"], false);
            }

        case actionTypes.POPULATE_MY_ORIGIN_INVITATIONS:
            return state.setIn(["myInvitations"],
                List(action.payload));

        case actionTypes.POPULATE_ORIGIN_INVITATIONS:
            return state.setIn(["currentPendingInvitations"],
                List(action.payload));

        case actionTypes.POPULATE_ORIGIN_MEMBERS:
            return state.setIn(["currentMembers"],
                List(action.payload));

        case actionTypes.POPULATE_ORIGIN_PUBLIC_KEYS:
            if (action.error) {
                return state.setIn(
                    ["ui", "current", "publicKeyListErrorMessage"],
                    action.error.message
                );
            } else {
                return state.setIn(["currentPublicKeys"], List(action.payload)).
                    setIn(
                        ["ui", "current", "publicKeyListErrorMessage"],
                        undefined
                    );
            }

        case actionTypes.SET_CURRENT_ORIGIN:
            if (action.error) {
                return state.set("current", Origin()).
                    setIn(["ui", "current", "errorMessage"],
                    action.error.message).
                    setIn(["ui", "current", "loading"], false).
                    setIn(["ui", "current", "exists"], false);
            } else {
                return state.set("current", Origin(action.payload)).
                    setIn(["ui", "current", "errorMessage"], undefined).
                    setIn(["ui", "current", "exists"], true).
                    setIn(["ui", "current", "loading"], false);
            }
        case actionTypes.SET_CURRENT_ORIGIN_CREATING_FLAG:
            return state.setIn(["ui", "current", "creating"], action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY:
            return state.setIn(["ui", "current", "addingPrivateKey"],
                action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY:
            return state.setIn(["ui", "current", "addingPublicKey"],
                action.payload);

        case actionTypes.SET_CURRENT_ORIGIN_LOADING:
            return state.setIn(["ui", "current", "loading"],
                action.payload);

        case actionTypes.SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "privateKeyErrorMessage"],
                action.payload);

        case actionTypes.SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "publicKeyErrorMessage"],
                action.payload);

        case actionTypes.SET_ORIGIN_USER_INVITE_ERROR_MESSAGE:
            return state.setIn(["ui", "current", "userInviteErrorMessage"],
                action.payload);

        case actionTypes.TOGGLE_ORIGIN_PICKER:
            return state.setIn(["ui", "isPickerOpen"],
                !state.getIn(["ui", "isPickerOpen"]));

        default:
            return state;
    }
}
