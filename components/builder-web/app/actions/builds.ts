import { BuilderApiClient } from "../BuilderApiClient";

export const CLEAR_BUILD = "CLEAR_BUILD";
export const CLEAR_BUILD_LOG = "CLEAR_BUILD_LOG";
export const CLEAR_BUILDS = "CLEAR_BUILDS";
export const FETCH_BUILDS = "FETCH_BUILDS";
export const POPULATE_BUILD = "POPULATE_BUILD";
export const POPULATE_BUILDS = "POPULATE_BUILDS";
export const POPULATE_BUILD_LOG = "POPULATE_BUILD_LOG";
export const STREAM_BUILD_LOG = "STREAM_BUILD_LOG";

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

export function fetchBuilds(origin: string, name: string, token: string) {
  return dispatch => {
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

          if (data["is_complete"] && start !== 0) {
            setTimeout(() => { dispatch(fetchBuild(id, token)); }, 5000);
          }
          else if (getState().builds.selected.stream) {
            setTimeout(() => { dispatch(fetchBuildLog(id, token, data["stop"])); }, 1000);
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
    payload: data ? data.jobs : undefined,
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
