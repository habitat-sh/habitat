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

import { addNotification, SUCCESS, DANGER } from "./notifications";
import { requestRoute } from "./router";
import * as depotApi from "../depotApi";
import { BuilderApiClient } from "../BuilderApiClient";
import { parseKey } from "../util";

export const CLEAR_DOCKER_INTEGRATIONS = "CLEAR_DOCKER_INTEGRATIONS";
export const DELETE_DOCKER_INTEGRATION = "DELETE_DOCKER_INTEGRATION";
export const POPULATE_MY_ORIGINS = "POPULATE_MY_ORIGINS";
export const POPULATE_MY_ORIGIN_INVITATIONS = "POPULATE_MY_ORIGIN_INVITATIONS";
export const POPULATE_ORIGIN_INVITATIONS = "POPULATE_ORIGIN_INVITATIONS";
export const POPULATE_ORIGIN_MEMBERS = "POPULATE_ORIGIN_MEMBERS";
export const POPULATE_ORIGIN_PUBLIC_KEYS = "POPULATE_ORIGIN_PUBLIC_KEYS";
export const POPULATE_ORIGIN_DOCKER_INTEGRATIONS = "POPULATE_ORIGIN_DOCKER_INTEGRATIONS";
export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";
export const SET_CURRENT_ORIGIN_CREATING_FLAG = "SET_CURRENT_ORIGIN_CREATING_FLAG";
export const SET_CURRENT_ORIGIN_LOADING = "SET_CURRENT_ORIGIN_LOADING";
export const SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY = "SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY";
export const SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY = "SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY";
export const SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE = "SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE";
export const SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE = "SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE";
export const SET_ORIGIN_USER_INVITE_ERROR_MESSAGE = "SET_ORIGIN_USER_INVITE_ERROR_MESSAGE";
export const SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE = "SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE";
export const TOGGLE_ORIGIN_PICKER = "TOGGLE_ORIGIN_PICKER";
export const SET_PACKAGE_COUNT_FOR_ORIGIN = "SET_PACKAGE_COUNT_FOR_ORIGIN";
export const UPDATE_ORIGIN = "UPDATE_ORIGIN";

export function acceptOriginInvitation(invitationId: string, originName: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).acceptOriginInvitation(invitationId, originName).
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

export function ignoreOriginInvitation(invitationId: string, originName: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).ignoreOriginInvitation(invitationId, originName).
            then(response => {
                dispatch(addNotification({
                    title: "Invitation Ignored",
                    type: SUCCESS,
                }));
                dispatch(fetchMyOriginInvitations(token));
            }).catch(error => {
                dispatch(addNotification({
                    title: "Invitation Ignore Failed",
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
            dispatch(fetchOriginsPackageCount(origins));
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

export function deleteDockerIntegration(origin: string, token: string, name: string) {
    return dispatch => {
        new BuilderApiClient(token).deleteDockerIntegration(origin, name)
            .then(response => {
                dispatch(fetchDockerIntegration(origin, token));
            })
            .catch(error => {
                dispatch(populateDockerIntegrations(undefined, error.message));
            });
    };
}

export function fetchDockerIntegration(origin: string, token: string) {
    return dispatch => {
        dispatch(clearDockerIntegration());
        new BuilderApiClient(token).getDockerIntegration(origin)
            .then(response => {
                dispatch(populateDockerIntegrations(response));
            })
            .catch(error => {
                dispatch(populateDockerIntegrations(undefined, error.message));
            });
    };
}

export function setDockerIntegration(origin: string, credentials, token: string) {
    return dispatch => {
        new BuilderApiClient(token).setDockerIntegration(origin, credentials)
            .then(() => {
                dispatch(fetchDockerIntegration(origin, token));
            })
            .catch(error => {
                dispatch(setOriginIntegrationSaveErrorMessage(error.message));
            });
    };
}

export function updateOrigin(origin: any, token: string) {
    return dispatch => {
        new BuilderApiClient(token).updateOrigin(origin)
            .then(() => {
                dispatch(addNotification({
                    title: "Origin settings saved",
                    type: SUCCESS
                }));
                dispatch(fetchOrigin(origin.name));
            })
            .catch(error => {
                dispatch(addNotification({
                    title: "Error saving origin settings",
                    body: error.message,
                    type: DANGER
                }));
            });
    };
}

export function fetchOriginsPackageCount(origins) {
    return dispatch => {
        origins.forEach(origin => {
            depotApi
                .getStats(origin)
                .then(response => {
                    response["origin"] = origin;
                    dispatch(populatePackageCountForOrigin(response));
                })
                .catch(error => {
                    dispatch(populatePackageCountForOrigin(error.message));
                });
        });
    };
}

function clearDockerIntegration() {
    return {
        type: CLEAR_DOCKER_INTEGRATIONS
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

function populateDockerIntegrations(payload, error = undefined) {
    return {
        type: POPULATE_ORIGIN_DOCKER_INTEGRATIONS,
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

function setOriginIntegrationSaveErrorMessage(payload: string) {
    return {
        type: SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE,
        payload,
    };
}

// ED TODO: uncomment this when the api endpoint is added for privacy settings
// function setCurrentOriginPrivacySetting(payload: string) {
//     return {
//         type: SET_ORIGIN_PRIVACY_SETTINGS,
//         payload,
//     };
// }

export function toggleOriginPicker() {
    return {
        type: TOGGLE_ORIGIN_PICKER,
    };
}

export function populatePackageCountForOrigin(payload) {
    return {
        type: SET_PACKAGE_COUNT_FOR_ORIGIN,
        payload
    };
}

export function uploadOriginPrivateKey(key: string, token: string) {
    return dispatch => {
        new BuilderApiClient(token).createOriginKey(key).then(() => {
            dispatch(setOriginPrivateKeyUploadErrorMessage(undefined));
            dispatch(fetchOrigin(parseKey(key).origin));  // we need this to make the keys appear after upload
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
