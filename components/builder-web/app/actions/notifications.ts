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
