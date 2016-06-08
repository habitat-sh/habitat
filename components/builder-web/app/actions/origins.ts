// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {addNotification, SUCCESS, DANGER} from "./notifications";
import {requestRoute} from "./router";
import * as api from "../builderApi";
import {BuilderApiClient} from "../BuilderApiClient";

export const POPULATE_MY_ORIGINS = "POPULATE_MY_ORIGINS";
export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";
export const SET_CURRENT_ORIGIN_CREATING_FLAG =
    "SET_CURRENT_ORIGIN_CREATING_FLAG";
export const SET_ORIGIN_ADDING_PRIVATE_KEY = "SET_ORIGIN_ADDING_PRIVATE_KEY";
export const SET_ORIGIN_ADDING_PUBLIC_KEY = "SET_ORIGIN_ADDING_PUBLIC_KEY";
export const TOGGLE_ORIGIN_PICKER = "TOGGLE_ORIGIN_PICKER";

export function createOrigin(origin, token, isFirstOrigin = false) {
    return dispatch => {
        dispatch(setCurrentOriginCreatingFlag(true));

        new BuilderApiClient(token).createOrigin(origin).then(origin => {
            if (isFirstOrigin || origin["default"]) {
                dispatch(setCurrentOrigin(origin));
            }

            dispatch(fetchMyOrigins());
            dispatch(setCurrentOriginCreatingFlag(false));
            dispatch(requestRoute(["Origins"]));
            dispatch(addNotification({
                title: "Origin Created",
                body: origin["default"] ?
                    `'${origin["name"]}' is now the default origin` : "",
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

export function deleteOrigin(origin) {
    return dispatch => {
        api.deleteOrigin(origin).then(() => {
            dispatch(fetchMyOrigins());
            dispatch(addNotification({
                title: "Origin Deleted",
                body: `'${origin["name"]}' has been deleted`,
                type: SUCCESS,
            }));
        }).catch(error => {
            dispatch(addNotification({
                title: "Failed to Delete Origin",
                body: error.message,
                type: DANGER,
            }));
        });
    };
}

export function fetchMyOrigins() {
    return dispatch => {
        api.getMyOrigins().then(origins => {
            dispatch(populateMyOrigins(origins));
        });
    };
}

function populateMyOrigins(payload) {
    return {
        type: POPULATE_MY_ORIGINS,
        payload,
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

export function setOriginAddingPrivateKey(payload: boolean) {
    return {
        type: SET_ORIGIN_ADDING_PRIVATE_KEY,
        payload,
    };
}

export function setOriginAddingPublicKey(payload: boolean) {
    return {
        type: SET_ORIGIN_ADDING_PUBLIC_KEY,
        payload,
    };
}

export function toggleOriginPicker() {
    return {
        type: TOGGLE_ORIGIN_PICKER,
    };
}
