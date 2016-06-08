// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {addNotification, SUCCESS, DANGER} from "./notifications";
import {requestRoute} from "./router";
import * as depotApi from "../depotApi";
import {BuilderApiClient} from "../BuilderApiClient";

export const POPULATE_MY_ORIGINS = "POPULATE_MY_ORIGINS";
export const POPULATE_PACKAGE_COUNT_FOR_ORIGIN = "POPULATE_PACKAGES_FOR_ORIGIN";
export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";
export const SET_CURRENT_ORIGIN_CREATING_FLAG =
    "SET_CURRENT_ORIGIN_CREATING_FLAG";
export const SET_CURRENT_ORIGIN_LOADING = "SET_CURRENT_ORIGIN_LOADING";
export const SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY = "SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY";
export const SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY = "SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY";
export const TOGGLE_ORIGIN_PICKER = "TOGGLE_ORIGIN_PICKER";

export function createOrigin(origin, token, isFirstOrigin = false) {
    return dispatch => {
        dispatch(setCurrentOriginCreatingFlag(true));

        new BuilderApiClient(token).createOrigin(origin).then(origin => {
            if (isFirstOrigin || origin["default"]) {
                dispatch(setCurrentOrigin(origin));
            }

            dispatch(fetchMyOrigins(token));
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

export function fetchMyOrigins(token) {
    return dispatch => {
        new BuilderApiClient(token).getMyOrigins().then(origins => {
            dispatch(populateMyOrigins(origins));
        }).catch(error => dispatch(populateMyOrigins(undefined, error)));
    };
}

export function fetchOrigin(originName: string) {
    return dispatch => {
        dispatch(setCurrentOriginLoading(true));
        new BuilderApiClient().getOrigin(originName).then(response => {
            dispatch(setCurrentOrigin(response));
        }).catch(error => {
            dispatch(setCurrentOrigin(undefined, error));
        });
    };
}

function populateMyOrigins(payload, error = undefined) {
    return {
        type: POPULATE_MY_ORIGINS,
        payload,
        error
    };
}

export function setCurrentOrigin(payload, error = undefined) {
    return {
        type: SET_CURRENT_ORIGIN,
        payload,
        error,
    };
}

export function setCurrentOriginAddingPrivateKey(payload: boolean) {
    return {
        type: SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY,
        payload,
    };
}

export function setCurrentOriginAddingPublicKey(payload: boolean) {
    return {
        type: SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY,
        payload,
    };
}

function setCurrentOriginLoading(payload: boolean) {
    return {
        type: SET_CURRENT_ORIGIN_LOADING,
        payload,
    };
}

function setCurrentOriginCreatingFlag(payload) {
    return {
        type: SET_CURRENT_ORIGIN_CREATING_FLAG,
        payload,
    };
}

export function toggleOriginPicker() {
    return {
        type: TOGGLE_ORIGIN_PICKER,
    };
}
