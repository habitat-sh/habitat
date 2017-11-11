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

import * as depotApi from '../client/depot-api';
import { BuilderApiClient } from '../client/builder-api';
import { addNotification } from './notifications';
import { DANGER, SUCCESS } from './notifications';

export const CLEAR_BUILD = 'CLEAR_BUILD';
export const CLEAR_BUILD_LOG = 'CLEAR_BUILD_LOG';
export const CLEAR_BUILDS = 'CLEAR_BUILDS';
export const POPULATE_BUILD = 'POPULATE_BUILD';
export const POPULATE_BUILDS = 'POPULATE_BUILDS';
export const POPULATE_BUILD_LOG = 'POPULATE_BUILD_LOG';
export const STREAM_BUILD_LOG = 'STREAM_BUILD_LOG';

export function clearBuild() {
  return {
    type: CLEAR_BUILD
  };
}

export function clearBuildLog() {
  return {
    type: CLEAR_BUILD_LOG
  };
}

export function clearBuilds() {
  return {
    type: CLEAR_BUILDS
  };
}

export function submitJob(origin: string, name: string, token: string) {
  return dispatch => {
    return depotApi.submitJob(origin, name, token)
      .then(response => {
        dispatch(addNotification({
          title: 'Build submitted',
          body: `A new build for ${origin}/${name} has been submitted.`,
          type: SUCCESS
        }));
        setTimeout(() => { dispatch(fetchBuilds(origin, name, token)); }, 5000);
      })
      .catch(error => {
        dispatch(addNotification({
          title: 'Build request failed',
          body: `Reason: ${error}`,
          type: DANGER
        }));
      });
  };
}

export function fetchBuilds(origin: string, name: string, token: string) {
  return dispatch => {
    dispatch(clearBuilds);

    new BuilderApiClient(token)
      .getBuilds(origin, name)
      .then((data) => dispatch(populateBuilds(data)))
      .catch((error) => dispatch(populateBuilds(null, error)));
  };
}

export function fetchBuild(id: string, token: string) {
  return dispatch => {
    dispatch(clearBuild());

    new BuilderApiClient(token)
      .getBuild(id)
      .then((data) => dispatch(populateBuild(data)))
      .catch((error) => dispatch(populateBuild(null, error)));
  };
}

export function fetchBuildLog(id: string, token: string, start = 0) {
  return (dispatch, getState) => {

    if (start === 0) {
      dispatch(clearBuildLog());
    }

    new BuilderApiClient(token)
      .getBuildLog(id, start)
      .then((data) => {
        dispatch(populateBuildLog(data));

        let complete = data['is_complete'];

        if (complete && data['start'] !== 0) {
          setTimeout(() => { dispatch(fetchBuild(id, token)); }, 5000);
        }
        else if (!complete && getState().builds.selected.stream) {
          setTimeout(() => { dispatch(fetchBuildLog(id, token, data['stop'])); }, 2000);
        }
      })
      .catch((error) => dispatch(populateBuildLog(null, error)));
  };
}

function populateBuild(data, error = undefined) {
  return {
    type: POPULATE_BUILD,
    payload: data,
    error: error
  };
}

function populateBuilds(data, error = undefined) {
  return {
    type: POPULATE_BUILDS,
    payload: data ? data.data : undefined,
    error: error
  };
}

function populateBuildLog(data, error = undefined) {
  return {
    type: POPULATE_BUILD_LOG,
    payload: data,
    error: error
  };
}

export function streamBuildLog(setting) {
  return {
    type: STREAM_BUILD_LOG,
    payload: setting
  };
}
