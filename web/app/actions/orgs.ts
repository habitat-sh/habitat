// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {addNotification, SUCCESS} from "./notifications";
import {requestRoute} from "./router";

export const FINISH_CREATING_ORG = "FINISH_CREATING_ORG";
export const POPULATE_ORG = "POPULATE_ORG";

export function addOrg(values) {
    return dispatch => {
        dispatch(populateOrg(values));
    };
}

export function finishAddingOrg(values) {
    return dispatch => {
        dispatch(finishCreatingOrg());
        dispatch(addNotification({
            title: "Organization Created",
            body: values.namespace,
            type: SUCCESS,
        }));
        dispatch(requestRoute(["Organizations"]));
    };
}

function finishCreatingOrg() {
    return {
        type: FINISH_CREATING_ORG,
    };
}

function populateOrg(values) {
    return {
        type: POPULATE_ORG,
        payload: values,
    };
}
