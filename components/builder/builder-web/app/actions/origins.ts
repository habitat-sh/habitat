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

import { addNotification, SUCCESS, DANGER } from './notifications';
import * as depotApi from '../client/depot-api';
import { BuilderApiClient } from '../client/builder-api';
import { parseKey } from '../util';

export const CLEAR_MY_ORIGINS = 'CLEAR_MY_ORIGINS';
export const CLEAR_MY_ORIGIN_INVITATIONS = 'CLEAR_MY_ORIGIN_INVITATIONS';
export const CLEAR_INTEGRATION = 'CLEAR_INTEGRATION';
export const CLEAR_INTEGRATIONS = 'CLEAR_INTEGRATIONS';
export const DELETE_INTEGRATION = 'DELETE_INTEGRATION';
export const POPULATE_MY_ORIGINS = 'POPULATE_MY_ORIGINS';
export const POPULATE_MY_ORIGIN_INVITATIONS = 'POPULATE_MY_ORIGIN_INVITATIONS';
export const POPULATE_ORIGIN_INVITATIONS = 'POPULATE_ORIGIN_INVITATIONS';
export const POPULATE_ORIGIN_MEMBERS = 'POPULATE_ORIGIN_MEMBERS';
export const POPULATE_ORIGIN_PUBLIC_KEYS = 'POPULATE_ORIGIN_PUBLIC_KEYS';
export const POPULATE_ORIGIN_INTEGRATION = 'POPULATE_ORIGIN_INTEGRATION';
export const POPULATE_ORIGIN_INTEGRATIONS = 'POPULATE_ORIGIN_INTEGRATIONS';
export const SET_CURRENT_ORIGIN = 'SET_CURRENT_ORIGIN';
export const SET_CURRENT_ORIGIN_CREATING_FLAG = 'SET_CURRENT_ORIGIN_CREATING_FLAG';
export const SET_CURRENT_ORIGIN_LOADING = 'SET_CURRENT_ORIGIN_LOADING';
export const SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY = 'SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY';
export const SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY = 'SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY';
export const SET_INTEGRATION_CREDS_VALIDATION = 'SET_INTEGRATION_CREDS_VALIDATION';
export const SET_ORIGIN_USER_INVITE_ERROR_MESSAGE = 'SET_ORIGIN_USER_INVITE_ERROR_MESSAGE';
export const SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE = 'SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE';
export const TOGGLE_ORIGIN_PICKER = 'TOGGLE_ORIGIN_PICKER';
export const SET_PACKAGE_COUNT_FOR_ORIGIN = 'SET_PACKAGE_COUNT_FOR_ORIGIN';
export const UPDATE_ORIGIN = 'UPDATE_ORIGIN';

export function acceptOriginInvitation(invitationId: string, originName: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).acceptOriginInvitation(invitationId, originName).
      then(response => {
        dispatch(addNotification({
          title: 'Invitation accepted',
          body: `You are now a member of ${originName}.`,
          type: SUCCESS,
        }));
        dispatch(fetchMyOriginInvitations(token));
        dispatch(fetchMyOrigins(token));
      }).catch(error => {
        dispatch(addNotification({
          title: 'Invitation acceptance failed',
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
          title: 'Invitation ignored',
          type: SUCCESS,
        }));
        dispatch(fetchMyOriginInvitations(token));
        dispatch(fetchMyOrigins(token));
      }).catch(error => {
        dispatch(addNotification({
          title: 'Invitation ignore failed',
          body: error.message,
          type: DANGER,
        }));
      });
  };
}

export function deleteOriginInvitation(invitationId: string, originName: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).deleteOriginInvitation(invitationId, originName).
      then(response => {
        dispatch(addNotification({
          title: 'Invitation rescinded',
          type: SUCCESS,
        }));
        dispatch(fetchOriginInvitations(originName, token));
      }).catch(error => {
        dispatch(addNotification({
          title: 'Failed to rescind invitation',
          body: error.message,
          type: DANGER,
        }));
      });
  };
}

export function deleteOriginMember(origin: string, member: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).deleteOriginMember(origin, member).
      then(response => {
        dispatch(addNotification({
          title: 'Member removed',
          body: `${member} is no longer a member of ${origin}.`,
          type: SUCCESS,
        }));
        dispatch(fetchOriginMembers(origin, token));
      }).catch(error => {
        dispatch(addNotification({
          title: 'Failed to remove member',
          body: error.message,
          type: DANGER,
        }));
      });
  };
}

export function createOrigin(body: object, token: string, isFirstOrigin = false, callback: Function = (origin) => { }) {
  return dispatch => {
    dispatch(setCurrentOriginCreatingFlag(true));

    new BuilderApiClient(token).createOrigin(body).then(origin => {
      dispatch(setCurrentOriginCreatingFlag(false));
      dispatch(fetchMyOrigins(token));

      if (isFirstOrigin || origin['default']) {
        dispatch(setCurrentOrigin(origin));
      }

      dispatch(addNotification({
        title: 'Origin created',
        body: origin['default'] ? `'${origin['name']}' is now the default origin` : '',
        type: SUCCESS,
      }));

      dispatch(generateOriginKeys(origin['name'], token));
      callback(origin);
    }).catch(error => {
      dispatch(setCurrentOriginCreatingFlag(false));
      dispatch(addNotification({
        title: 'Failed to Create Origin',
        body: error.message,
        type: DANGER,
      }));
    });
  };
}

export function fetchMyOrigins(token) {
  return dispatch => {
    dispatch(clearMyOrigins());

    new BuilderApiClient(token).getMyOrigins()
      .then(origins => {
        dispatch(populateMyOrigins(origins));
        dispatch(fetchOriginsPackageCount(origins));
      })
      .catch(error => dispatch(populateMyOrigins(undefined, error)));
  };
}

export function fetchMyOriginInvitations(token) {
  return dispatch => {
    dispatch(clearMyOriginInvitations());

    new BuilderApiClient(token).getMyOriginInvitations()
      .then(invitations => {
        dispatch(populateMyOriginInvitations(invitations));
      })
      .catch(error => {
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

export function generateOriginKeys(origin: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).generateOriginKeys(origin).
      then(response => {
        dispatch(fetchOrigin(origin));
        dispatch(fetchOriginPublicKeys(origin, token));
        dispatch(addNotification({
          title: 'Origin keys generated',
          body: 'Your public and private keys have been created and are available for download.',
          type: SUCCESS
        }));
      }).catch(error => {
        dispatch(addNotification({
          title: 'Error generating origin keys',
          body: error.message,
          type: DANGER
        }));
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

export function deleteIntegration(origin: string, token: string, name: string, type: string) {
  return dispatch => {
    new BuilderApiClient(token).deleteIntegration(origin, name, type)
      .then(response => {
        dispatch(fetchIntegrations(origin, token));
      })
      .catch(error => {
        dispatch(populateIntegrations(undefined, error.message));
      });
  };
}

export function fetchIntegration(origin: string, type: string, name: string, token: string) {
  return dispatch => {
    dispatch(clearIntegration());
    new BuilderApiClient(token).getIntegration(origin, type, name)
      .then(response => {
        dispatch(populateIntegration(response));
      })
      .catch(error => {
        dispatch(populateIntegration(undefined, error.message));
      });
  };
}

export function fetchIntegrations(origin: string, token: string) {
  return dispatch => {
    dispatch(clearIntegrations());
    new BuilderApiClient(token).getIntegrations(origin)
      .then(response => {
        dispatch(populateIntegrations(response));
      })
      .catch(error => {
        dispatch(populateIntegrations(undefined, error.message));
      });
  };
}

export function setIntegration(origin: string, credentials, token: string, type: string, name: string) {
  return dispatch => {
    new BuilderApiClient(token).setIntegration(origin, credentials, type, name)
      .then(() => {
        dispatch(fetchIntegrations(origin, token));
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
          title: 'Origin settings saved',
          type: SUCCESS
        }));
        dispatch(fetchOrigin(origin.name));
      })
      .catch(error => {
        dispatch(addNotification({
          title: 'Error saving origin settings',
          body: error.message,
          type: DANGER
        }));
      });
  };
}

export function validateIntegrationCredentials(username: string, password: string, token: string, type: string, url?: string) {
  return dispatch => {

    dispatch(setIntegrationCredsValidation({
      validating: true,
      validated: false,
      valid: false,
      message: 'Verifying...'
    }));

    new BuilderApiClient(token).validateIntegrationCredentials(username, password, type, url)
      .then(response => {
        dispatch(setIntegrationCredsValidation({
          validating: false,
          validated: true,
          valid: true,
          message: 'Verified'
        }));
      })
      .catch(error => {
        dispatch(setIntegrationCredsValidation({
          validating: false,
          validated: true,
          valid: false,
          message: 'Username and password combination is not valid.'
        }));
      });
  };
}

export function clearIntegrationCredsValidation() {
  return dispatch => {
    dispatch(setIntegrationCredsValidation({
      validating: false,
      validated: false,
      valid: false,
      message: undefined
    }));
  };
}

export function fetchOriginsPackageCount(origins) {
  return dispatch => {
    origins.forEach(origin => {
      depotApi
        .getStats(origin)
        .then(response => {
          response['origin'] = origin;
          dispatch(populatePackageCountForOrigin(response));
        })
        .catch(error => {
          dispatch(populatePackageCountForOrigin(error.message));
        });
    });
  };
}

function clearMyOrigins() {
  return {
    type: CLEAR_MY_ORIGINS
  };
}

function clearMyOriginInvitations() {
  return {
    type: CLEAR_MY_ORIGIN_INVITATIONS
  };
}

function clearIntegration() {
  return {
    type: CLEAR_INTEGRATION
  };
}

function clearIntegrations() {
  return {
    type: CLEAR_INTEGRATIONS
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

function populateIntegration(payload, error = undefined) {
  return {
    type: POPULATE_ORIGIN_INTEGRATION,
    payload,
    error
  };
}

function populateIntegrations(payload, error = undefined) {
  return {
    type: POPULATE_ORIGIN_INTEGRATIONS,
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

function setIntegrationCredsValidation(payload: any) {
  return {
    type: SET_INTEGRATION_CREDS_VALIDATION,
    payload
  };
}

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
      dispatch(fetchOrigin(parseKey(key).origin));
      dispatch(addNotification({
        title: 'Private key uploaded',
        body: `${parseKey(key).name} has been uploaded.`,
        type: SUCCESS,
      }));
    }).catch(error => {
      dispatch(addNotification({
        title: 'Failed to save private key',
        body: error.message,
        type: DANGER,
      }));
    });
  };
}

export function uploadOriginPublicKey(key: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).createOriginKey(key).then(() => {
      dispatch(fetchOriginPublicKeys(parseKey(key).origin, token));
      dispatch(addNotification({
        title: 'Public key uploaded',
        body: `${parseKey(key).name} has been uploaded.`,
        type: SUCCESS,
      }));
    }).catch(error => {
      dispatch(addNotification({
        title: 'Failed to save public key',
        body: error.message,
        type: DANGER,
      }));
    });
  };
}
