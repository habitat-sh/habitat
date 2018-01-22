// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

import { Browser } from '../browser';

export const SET_BLDR_SESSION_TOKEN = 'SET_BLDR_SESSION_TOKEN';
export const LOAD_BLDR_SESSION_STATE = 'LOAD_BLDR_SESSION_STATE';

export function setBldrSessionToken(payload) {
  Browser.setCookie('bldrSessionToken', payload);

  return {
    type: SET_BLDR_SESSION_TOKEN,
    payload
  };
}

export function loadBldrSessionState() {
  return {
    type: LOAD_BLDR_SESSION_STATE,
    payload: Browser.getCookie('bldrSessionToken')
  };
}
