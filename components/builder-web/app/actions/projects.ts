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

import * as fakeApi from "../fakeApi";
import { Observable } from "rxjs";
import { BuilderApiClient } from "../BuilderApiClient";
import { addNotification } from "./notifications";
import { DANGER, INFO, SUCCESS, WARNING } from "./notifications";
import { requestRoute, resetRedirectRoute } from "./router";
import { packageString } from "../util";

export const POPULATE_PROJECT = "POPULATE_PROJECT";
export const SET_CURRENT_PROJECT = "SET_CURRENT_PROJECT";
export const SET_PROJECTS = "SET_PROJECTS";
export const DELETE_PROJECT = "DELETE_PROJECT";
export const DEPOPULATE_PROJECT = "DEPOPULATE_PROJECT";
export const SET_PROJECT_HINT = "SET_PROJECT_HINT";
export const RESET_PROJECT_HINT = "RESET_PROJECT_HINT";

export function addProject(project: any, token: string, onComplete: Function = () => {}) {
  return dispatch => {
    dispatch(addNotification({
      title: "Adding plan",
      body: `To origin "${project.origin}"`,
      type: INFO,
    }));
    new BuilderApiClient(token).createProject(project).then(response => {
      dispatch(resetProjectHint());
      dispatch(addNotification({
        title: "Plan created",
        body: `Created ${response["id"]}.`,
        type: SUCCESS,
      }));
      onComplete({success: true, response});
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to create plan",
        body: error.message,
        type: DANGER,
      }));
      onComplete({success: false, error});
    });
  };
}

export function setProjectHint(hint: Object) {
  return {
    type: SET_PROJECT_HINT,
    payload: hint
  };
}

export function resetProjectHint() {
  return {
    type: RESET_PROJECT_HINT
  };
}

export function fetchProject(id: string, token: string, alert: boolean) {
  return dispatch => {
    dispatch(setCurrentProject({
      ui: {
        exists: false,
        loading: true
      }
    }));

    new BuilderApiClient(token).getProject(id).then(response => {
      dispatch(setCurrentProject(Object.assign({
        ui: {
          exists: true,
          loading: false
        }
      }, response)));
      dispatch(populateProject(response));
    }).catch(error => {
      dispatch(setCurrentProject({
        ui: {
          exists: false,
          loading: false
        }
      }));

      if (alert) {
        dispatch(addNotification({
          title: "Failed to fetch project",
          body: error.message,
          type: DANGER,
        }));
      }
    });
  };
}

export function fetchProjectsForPackages(packages: Array < Object > , token: string) {
  return dispatch => {
    for (let pkg of packages) {
      let id = `${pkg["origin"]}/${pkg["name"]}`;
      dispatch(fetchProject(id, token, false));
    }
  };
}

export function fetchProjects(token: string) {
  return dispatch => {
    new BuilderApiClient(token).getProjects().then(response => {
      dispatch(setProjects(response));
    });
  };
}

export function deleteProject(id: string, token: string) {
  return dispatch => {
    new BuilderApiClient(token).deleteProject(id).then(response => {
      dispatch(addNotification({
        title: "Plan link deleted",
        body: `Deleted ${id}.`,
        type: SUCCESS
      }));
      dispatch(actuallyDeleteProject(id));
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to delete plan",
        body: error.message,
        type: DANGER,
      }));
    });
  };
}

export function updateProject(projectId: string, project: Object, token: string, onComplete: Function = () => {} ) {
  return dispatch => {
    new BuilderApiClient(token).updateProject(projectId, project).then(response => {
      dispatch(resetProjectHint());
      dispatch(resetRedirectRoute());
      dispatch(addNotification({
        title: "Plan updated",
        body: `Updated ${projectId}.`,
        type: SUCCESS
      }));
      onComplete({success: true, response});
    }).catch(error => {
      dispatch(addNotification({
        title: "Failed to update plan",
        body: error.message,
        type: DANGER,
      }));
      onComplete({success: false, error});
    });
  };
}

function depopulateProject(projectId) {
  return {
    type: DEPOPULATE_PROJECT,
    payload: projectId
  };
}

function actuallyDeleteProject(projectId) {
  return {
    type: DELETE_PROJECT,
    payload: projectId,
  };
}

function populateProject(project) {
  return {
    type: POPULATE_PROJECT,
    payload: project,
  };
}

export function setCurrentProject(project) {
  return {
    type: SET_CURRENT_PROJECT,
    payload: project,
  };
}

function setProjects(projects) {
  return {
    type: SET_PROJECTS,
    payload: projects,
  };
}
