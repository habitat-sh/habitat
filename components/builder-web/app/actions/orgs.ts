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
        dispatch(requestRoute(["/orgs"]));
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
