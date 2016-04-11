// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {addNotification, SUCCESS} from "./notifications";
import {requestRoute} from "./router";

export const FINISH_CREATING_ORG = "FINISH_CREATING_ORG";
export const ORG_INVITATION_CANCELLED = "ORG_INVITATION_CANCELLED";
export const ORG_INVITATION_CREATED = "ORG_INVITATION_CREATED";
export const PERFORM_ORG_MEMBER_SEARCH = "PERFORM_ORG_MEMBER_SEARCH";
export const POPULATE_ORG = "POPULATE_ORG";
export const TOGGLE_MEMBER_ACTION_MENU = "TOGGLE_MEMBER_ACTION_MENU";

export function addOrg(values) {
    return dispatch => {
        dispatch(populateOrg(values));
    };
}

export function cancelOrgInvitation(index) {
    return dispatch => {
        dispatch(toggleMemberActionMenu(index));
        dispatch(removeOrgMember(index));
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

export function inviteMemberToOrg(member, index) {
    return {
        type: ORG_INVITATION_CREATED,
        payload: { member, index },
    };
}

export function performOrgMemberSearch(value) {
    return {
        type: PERFORM_ORG_MEMBER_SEARCH,
        payload: value,
    };
}

function populateOrg(values) {
    return {
        type: POPULATE_ORG,
        payload: values,
    };
}

function removeOrgMember(index) {
    return {
        type: ORG_INVITATION_CANCELLED,
        payload: index,
    };
}

export function toggleMemberActionMenu(index) {
    return {
        type: TOGGLE_MEMBER_ACTION_MENU,
        payload: index,
    };
}
