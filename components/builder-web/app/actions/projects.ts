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

import { BuilderApiClient } from "../BuilderApiClient";
import { addNotification } from "./notifications";
import { DANGER, INFO, SUCCESS, WARNING } from "./notifications";

export const CLEAR_PROJECTS = "CLEAR_PROJECTS";
export const CLEAR_CURRENT_PROJECT = "CLEAR_CURRENT_PROJECT";
export const CLEAR_CURRENT_PROJECT_INTEGRATION = "CLEAR_CURRENT_PROJECT_SETTINGS";
export const DELETE_PROJECT = "DELETE_PROJECT";
export const SET_CURRENT_PROJECT = "SET_CURRENT_PROJECT";
export const SET_CURRENT_PROJECT_INTEGRATION = "SET_CURRENT_PROJECT_INTEGRATION";
export const SET_PROJECTS = "SET_PROJECTS";

function clearProjects() {
    return {
        type: CLEAR_PROJECTS
    };
}

export function addProject(project: any, token: string, onComplete: Function = () => {}) {
  return dispatch => {
    new BuilderApiClient(token).createProject(project).then(response => {
      dispatch(addNotification({
        title: "Plan connection saved",
        type: SUCCESS,
      }));
      onComplete({success: true, response});
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to save plan connection",
        body: (error.message === "Conflict" ? `The plan you selected is already connected in this origin.` : error.message),
        type: DANGER,
      }));
      onComplete({success: false, error});
    });
  };
}

export function setProjectIntegrationSettings(origin: string, name: string, integration: string, settings: any, token: string) {
  return dispatch => {
    new BuilderApiClient(token).setProjectIntegrationSettings(origin, name, integration, settings)
      .then(response => {
        dispatch(addNotification({
          title: "Integration settings saved",
          type: SUCCESS
        }));
      })
      .catch(error => {
        dispatch(addNotification({
          title: "Failed to save integration settings",
          body: error.message,
          type: DANGER
        }));
      });
  };
}

export function setProjectVisibility(origin: string, name: string, setting: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).setProjectVisibility(origin, name, setting)
      .then(response => {
        dispatch(fetchProject(origin, name, token, false));
        dispatch(addNotification({
          title: "Privacy settings saved",
          type: SUCCESS
        }));
      })
      .catch(error => {
        dispatch(addNotification({
          title: "Failed to save privacy settings",
          body: error.message,
          type: DANGER
        }));
      });
  };
}

export function fetchProject(origin: string, name: string, token: string, alert: boolean) {
  return dispatch => {
    dispatch(clearCurrentProject());
    dispatch(clearCurrentProjectIntegration());

    new BuilderApiClient(token).getProject(origin, name)
      .then(response => {
        dispatch(setCurrentProject(response, null));
        dispatch(fetchProjectIntegration(origin, name, "docker", token));
      })
      .catch((error) => {
        dispatch(setCurrentProject(null, error));
      });
  };
}

export function fetchProjects(origin: string, token: string) {
  return dispatch => {
    dispatch(clearProjects());

    new BuilderApiClient(token).getProjects(origin).then(response => {
        if (Array.isArray(response) && response.length > 0) {
          dispatch(setProjects(response));
        }
    });
  };
}

export function fetchProjectIntegration(origin: string, name: string, integration: string, token: string) {
  return dispatch => {
    dispatch(clearCurrentProjectIntegration());

    new BuilderApiClient(token).getProjectIntegration(origin, name, integration)
      .then(response => {
        dispatch(setCurrentProjectIntegration(response));
      })
      .catch(error => {});
  };
}

export function deleteProject(id: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).deleteProject(id).then(response => {
      dispatch(clearCurrentProject());
      dispatch(addNotification({
        title: "Plan connection deleted",
        type: SUCCESS
      }));
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to delete plan connection",
        body: error.message,
        type: DANGER,
      }));
    });
  };
}

export function updateProject(projectId: string, project: Object, token: string, onComplete: Function = () => {} ) {
  return dispatch => {
    new BuilderApiClient(token).updateProject(projectId, project).then(response => {
      dispatch(addNotification({
        title: "Plan connection saved",
        type: SUCCESS
      }));
      onComplete({success: true, response});
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to save plan connection",
        body: error.message,
        type: DANGER,
      }));
      onComplete({success: false, error});
    });
  };
}

function clearCurrentProject() {
  return {
    type: CLEAR_CURRENT_PROJECT
  };
}

function clearCurrentProjectIntegration() {
  return {
    type: CLEAR_CURRENT_PROJECT_INTEGRATION
  };
}

export function setCurrentProject(project, error = undefined) {
  return {
    type: SET_CURRENT_PROJECT,
    payload: project,
    error: error
  };
}

function setCurrentProjectIntegration(settings) {
  return {
    type: SET_CURRENT_PROJECT_INTEGRATION,
    payload: settings
  };
}

function setProjects(projects) {
  return {
    type: SET_PROJECTS,
    payload: projects,
  };
}
