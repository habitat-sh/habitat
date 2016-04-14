// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {addNotification, SUCCESS, DANGER} from "./notifications";
import {requestRoute} from "./router";
import {createOrigin} from "../builderApi";

export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";
export const SET_CURRENT_ORIGIN_CREATING_FLAG =
    "SET_CURRENT_ORIGIN_CREATING_FLAG";

export function createOrigin(name) {
    return dispatch => {
        dispatch(setCurrentOriginCreatingFlag(true));

        createOrigin(name).then(origin => {
            dispatch(setCurrentOrigin(origin));
            dispatch(setCurrentOriginCreatingFlag(false));
            dispatch(requestRoute(["Packages"]));
            dispatch(addNotification({
                title: "Origin Created",
                body: `'${name}' is now your default origin.`,
                type: SUCCESS,
            }));
        }).catch(error => {
            dispatch(setCurrentOriginCreatingFlag(false));
            dispatch(addNotification({
                title: "Failed to Create Origin",
                body: error.message,
                type: DANGER,
            }));
        });
    };
}

export function setCurrentOrigin(payload) {
    return {
        type: SET_CURRENT_ORIGIN,
        payload,
    };
}

function setCurrentOriginCreatingFlag(payload) {
    return {
        type: SET_CURRENT_ORIGIN_CREATING_FLAG,
        payload,
    };
}
