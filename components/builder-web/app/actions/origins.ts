// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import {addNotification, SUCCESS, DANGER} from "./notifications";
import {requestRoute} from "./router";
import * as depotApi from "../depotApi";
import {BuilderApiClient} from "../BuilderApiClient";
import {parseKey} from "../util";

export const POPULATE_MY_ORIGINS = "POPULATE_MY_ORIGINS";
export const POPULATE_MY_ORIGIN_INVITATIONS = "POPULATE_MY_ORIGIN_INVITATIONS";
export const POPULATE_ORIGIN_INVITATIONS = "POPULATE_ORIGIN_INVITATIONS";
export const POPULATE_ORIGIN_MEMBERS = "POPULATE_ORIGIN_MEMBERS";
export const POPULATE_ORIGIN_PUBLIC_KEYS = "POPULATE_ORIGIN_PUBLIC_KEYS";
export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";
export const SET_CURRENT_ORIGIN_CREATING_FLAG =
    "SET_CURRENT_ORIGIN_CREATING_FLAG";
export const SET_CURRENT_ORIGIN_LOADING = "SET_CURRENT_ORIGIN_LOADING";
export const SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY =
    "SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY";
export const SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY =
    "SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY";
export const SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE =
    "SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE";
export const SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE =
    "SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE";
export const SET_ORIGIN_USER_INVITE_ERROR_MESSAGE =
    "SET_ORIGIN_USER_INVITE_ERROR_MESSAGE";
export const TOGGLE_ORIGIN_PICKER = "TOGGLE_ORIGIN_PICKER";

export function acceptOriginInvitation(invitationId: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).acceptOriginInvitation(invitationId).
            then(response => {
                dispatch(addNotification({
                    title: "Invitation Accepted",
                    body: "You are now a member",
                    type: SUCCESS,
                }));
                dispatch(fetchMyOriginInvitations(token));
            }).catch(error => {
                dispatch(addNotification({
                    title: "Invitation Acceptance Failed",
                    body: error.message,
                    type: DANGER,
                }));
            });
    };
}

export function createOrigin(origin, token, isFirstOrigin = false) {
    return dispatch => {
        dispatch(setCurrentOriginCreatingFlag(true));

        new BuilderApiClient(token).createOrigin(origin).then(origin => {
            if (isFirstOrigin || origin["default"]) {
                dispatch(setCurrentOrigin(origin));
            }

            dispatch(fetchMyOrigins(token));
            dispatch(setCurrentOriginCreatingFlag(false));
            dispatch(requestRoute(["/origins"]));
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

export function fetchMyOriginInvitations(token) {
    return dispatch => {
        new BuilderApiClient(token).getMyOriginInvitations().
            then(invitations => {
                dispatch(populateMyOriginInvitations(invitations));
            }).catch(error => {
                dispatch(populateMyOriginInvitations(undefined, error));
            });
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

export function fetchOriginInvitations(originName: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).getOriginInvitations(originName).
            then(response => {
                dispatch(populateOriginInvitations(response));
            }).catch(error => {
                dispatch(populateOriginInvitations(undefined, error));
            });
    };
}

export function fetchOriginMembers(originName: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).getOriginMembers(originName).
            then(response => {
                dispatch(populateOriginMembers(response));
            }).catch(error => {
                dispatch(populateOriginMembers(undefined, error));
            });
    };
}

export function fetchOriginPublicKeys(originName: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).getOriginPublicKeys(originName).
            then(response => {
                dispatch(populateOriginPublicKeys(response));
            }).catch(error => {
                dispatch(populateOriginPublicKeys(undefined, error));
            });
    };
}

export function inviteUserToOrigin(username: string, origin: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).inviteUserToOrigin(username, origin).
            then(response => {
                dispatch(setOriginUserInviteErrorMessage(undefined));
                dispatch(fetchOriginInvitations(origin, token));
            }).catch(error => {
                dispatch(setOriginUserInviteErrorMessage(error.message));
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

function populateMyOriginInvitations(payload, error = undefined) {
    return {
        type: POPULATE_MY_ORIGIN_INVITATIONS,
        payload,
        error,
    };
}

function populateOriginInvitations(payload, error = undefined) {
    return {
        type: POPULATE_ORIGIN_INVITATIONS,
        payload,
        error,
    };
}

function populateOriginMembers(payload, error = undefined) {
    return {
        type: POPULATE_ORIGIN_MEMBERS,
        payload,
        error,
    };
}

function populateOriginPublicKeys(payload, error = undefined) {
    return {
        type: POPULATE_ORIGIN_PUBLIC_KEYS,
        payload,
        error,
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

function setOriginPrivateKeyUploadErrorMessage(payload: string) {
    return {
        type: SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE,
        payload,
    };
}

function setOriginPublicKeyUploadErrorMessage(payload: string) {
    return {
        type: SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE,
        payload,
    };
}

function setOriginUserInviteErrorMessage(payload: string) {
    return {
        type: SET_ORIGIN_USER_INVITE_ERROR_MESSAGE,
        payload,
    };
}

export function toggleOriginPicker() {
    return {
        type: TOGGLE_ORIGIN_PICKER,
    };
}

export function uploadOriginPrivateKey(key: string , token: string) {
    return dispatch => {
        new BuilderApiClient(token).createOriginKey(key).then(() => {
            dispatch(setOriginPrivateKeyUploadErrorMessage(undefined));
            dispatch(setCurrentOriginAddingPrivateKey(false));
            dispatch(addNotification({
                title: "Origin Private Key Uploaded",
                body: `'${parseKey(key).name}' has been uploaded`,
                type: SUCCESS,
            }));
        }).catch(error => {
            dispatch(setOriginPrivateKeyUploadErrorMessage(error.message));
        });
    };
}

export function uploadOriginPublicKey(key: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).createOriginKey(key).then(() => {
            dispatch(setOriginPublicKeyUploadErrorMessage(undefined));
            dispatch(setCurrentOriginAddingPublicKey(false));
            dispatch(fetchOriginPublicKeys(parseKey(key).origin, token));
            dispatch(addNotification({
                title: "Origin Public Key Uploaded",
                body: `'${parseKey(key).name}' has been uploaded`,
                type: SUCCESS,
            }));
        }).catch(error => {
            dispatch(setOriginPublicKeyUploadErrorMessage(error.message));
        });
    };
}
