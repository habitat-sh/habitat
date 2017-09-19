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

import { requestRoute, removeSessionStorage, resetAppState } from "./index";

export const SIGN_IN_ATTEMPT = "SIGN_IN_ATTEMPT";
export const TOGGLE_USER_NAV_MENU = "TOGGLE_USER_NAV_MENU";

export function attemptSignIn(username) {
    return {
        type: SIGN_IN_ATTEMPT,
        payload: { username: username },
    };
}

export function toggleUserNavMenu() {
    return {
        type: TOGGLE_USER_NAV_MENU
    };
}

export function signOut() {
    return dispatch => {
        dispatch(removeSessionStorage());
        dispatch(resetAppState());
        dispatch(requestRoute(["/sign-in"]));
    };
}
