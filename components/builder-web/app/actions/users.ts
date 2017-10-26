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

import { requestRoute, removeSessionStorage, resetAppState } from './index';
import { addNotification, SUCCESS, DANGER } from './notifications';
import { BuilderApiClient } from '../client/builder-api';

export const POPULATE_PROFILE = 'POPULATE_PROFILE';
export const SET_PRIVILEGES = 'SET_PRIVILEGES';
export const SET_SIGNING_IN_FLAG = 'SET_SIGNING_IN_FLAG';
export const SIGN_IN_ATTEMPT = 'SIGN_IN_ATTEMPT';
export const TOGGLE_USER_NAV_MENU = 'TOGGLE_USER_NAV_MENU';

export function fetchProfile(token: string) {
  return dispatch => {
    new BuilderApiClient(token).getProfile()
      .then(data => {
        dispatch(populateProfile(data));
        notifySegment(data);
      })
      .catch(err => { });
  };
}

export function saveProfile(profile: any, token: string) {
  return dispatch => {
    new BuilderApiClient(token).saveProfile(profile)
      .then(() => {
        dispatch(addNotification({
          title: 'Profile saved',
          type: SUCCESS
        }));
        dispatch(fetchProfile(token));
      })
      .catch(err => {
        dispatch(addNotification({
          title: 'Error saving profile',
          body: `${err.message}`,
          type: DANGER
        }));
      });
  };
}

function notifySegment(data: any) {
  const segment = window['analytics'];

  if (segment && typeof segment.identify === 'function') {
    segment.identify(data.id, { email: data.email, name: data.name });
  }
}

function populateProfile(payload) {
  return {
    type: POPULATE_PROFILE,
    payload
  };
}

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

export function setPrivileges(payload) {
  return {
    type: SET_PRIVILEGES,
    payload
  };
}

export function setSigningInFlag(payload) {
  return {
    type: SET_SIGNING_IN_FLAG,
    payload,
  };
}

export function signOut() {
  return dispatch => {
    dispatch(removeSessionStorage());
    dispatch(resetAppState());
    dispatch(requestRoute(['/sign-in']));
  };
}
