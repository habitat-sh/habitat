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

import 'whatwg-fetch';
import config from '../config';
import {
  setCurrentUsername,
  addNotification,
  fetchMyOrigins,
  fetchMyOriginInvitations,
  fetchProfile,
  setPrivileges,
  signingIn,
  signInFailed,
  signOut
} from './index';
import { DANGER, WARNING } from './notifications';
import { GitHubApiClient } from '../client/github-api';
import { setBldrSessionToken } from './sessions';
import { Browser } from '../browser';

const uuid = require('uuid').v4;
const gitHubTokenAuthUrl = `${config['habitat_api_url']}/v1/authenticate`;

export const CLEAR_GITHUB_INSTALLATIONS = 'CLEAR_GITHUB_INSTALLATIONS';
export const LOAD_GITHUB_SESSION_STATE = 'LOAD_GITHUB_SESSION_STATE';
export const POPULATE_GITHUB_INSTALLATIONS = 'POPULATE_GITHUB_INSTALLATIONS';
export const POPULATE_GITHUB_USER_DATA = 'POPULATE_GITHUB_USER_DATA';
export const SET_GITHUB_AUTH_STATE = 'SET_GITHUB_AUTH_STATE';
export const SET_GITHUB_AUTH_TOKEN = 'SET_GITHUB_AUTH_TOKEN';

export function authenticate(gitHubToken: string, bldrToken: string) {

  return (dispatch, getState) => {

    if (gitHubToken) {
      dispatch(setGitHubAuthToken(gitHubToken));

      fetch(`${config['github_api_url']}/user?access_token=${gitHubToken}`).then(response => {
        if (response.ok) {
          return response.json();
        } else {
          // If the response is not ok, throw an error from the
          // promise to be handled below.
          return response.json().then(error => { throw error; });
        }
      })
        .then(data => {
          dispatch(populateGitHubUserData(data));
          dispatch(setCurrentUsername(data.login));
        })
        .catch(error => {
          // We can assume an error from the response is a 401; anything
          // else is probably a transient failure on GitHub's end, which
          // we can expect to clear when we try to sign in again.
          //
          // When we get an unauthorized response, our token is no
          // longer valid, so sign out.
          dispatch(signOut(true, getState().router.route.url));
          dispatch(addNotification({
            title: 'GitHub Authorization Failed',
            body: 'Please sign in again.',
            type: WARNING,
          }));
        });
    }

    if (bldrToken) {
      dispatch(setBldrSessionToken(bldrToken));
      dispatch(fetchMyOrigins(bldrToken));
      dispatch(fetchMyOriginInvitations(bldrToken));
      dispatch(fetchProfile(bldrToken));
    }
  };
}

export function fetchGitHubInstallations() {
  const token = Browser.getCookie('gitHubAuthToken');

  return dispatch => {
    const client = new GitHubApiClient(token);
    dispatch(clearGitHubInstallations());

    client.getUserInstallations()
      .then((results) => {
        dispatch(populateGitHubInstallations(results));
      })
      .catch((error) => {
        console.error(error);
      });
  };
}

export function loadGitHubSessionState() {
  return {
    type: LOAD_GITHUB_SESSION_STATE,
    payload: {
      gitHubAuthToken: Browser.getCookie('gitHubAuthToken'),
      gitHubAuthState: Browser.getCookie('gitHubAuthState')
    },
  };
}

function clearGitHubInstallations() {
  return {
    type: CLEAR_GITHUB_INSTALLATIONS
  };
}

function populateGitHubInstallations(payload) {
  return {
    type: POPULATE_GITHUB_INSTALLATIONS,
    payload,
  };
}

function populateGitHubUserData(payload) {
  return {
    type: POPULATE_GITHUB_USER_DATA,
    payload,
  };
}

export function removeSession() {
  return dispatch => {
    Browser.removeCookie('gitHubAuthState');
    Browser.removeCookie('gitHubAuthToken');
    Browser.removeCookie('bldrSessionToken');
  };
}

export function exchangeGitHubAuthCode(code: string, state: string) {

  return (dispatch, getState) => {
    dispatch(setGitHubAuthState());

    if (state === getState().gitHub.authState) {
      dispatch(signingIn(true));

      fetch(`${gitHubTokenAuthUrl}/${code}`).then(response => {
        return response.json();
      })
        .then(data => {
          dispatch(signingIn(false));

          if (data.oauth_token && data.token) {
            dispatch(authenticate(data.oauth_token, data.token));
            dispatch(setPrivileges(data.flags));
          } else {
            dispatch(signInFailed());
            dispatch(addNotification({
              title: 'Authentication Failed',
              body: `[err=${data.code}] ${data.msg}`,
              type: DANGER
            }));
          }
        })
        .catch(error => {
          dispatch(signingIn(false));
          dispatch(signInFailed());
          dispatch(addNotification({
            title: 'Authentication Failed',
            body: 'Unable to retrieve GitHub token',
            type: DANGER
          }));
        });
    }
    else {
      dispatch(signInFailed());
    }
  };
}

export function setGitHubAuthState() {
  let payload = Browser.getCookie('gitHubAuthState') || uuid();
  Browser.setCookie('gitHubAuthState', payload);

  return {
    type: SET_GITHUB_AUTH_STATE,
    payload
  };
}

export function setGitHubAuthToken(payload) {
  Browser.setCookie('gitHubAuthToken', payload);

  return {
    type: SET_GITHUB_AUTH_TOKEN,
    payload
  };
}
