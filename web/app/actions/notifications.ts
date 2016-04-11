// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

export const ADD_NOTIFICATION = "ADD_NOTIFICATION";
export const REMOVE_NOTIFICATION = "REMOVE_NOTIFICATION";

export const DANGER = "danger";
export const INFO = "info";
export const SUCCESS = "success";
export const WARNING = "warning";

export function addNotification(n) {
    return dispatch => {
        dispatch(populateNotification(n));
        setTimeout(() => dispatch(removeNotification(0)), 5000);
    };
}

export function removeNotification(i) {
    return {
        type: REMOVE_NOTIFICATION,
        payload: i,
    };
}

function populateNotification(n) {
    return {
        type: ADD_NOTIFICATION,
        payload: n,
    };
}
