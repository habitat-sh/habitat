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

import * as fakeApi from "../fakeApi";
import {Observable} from "rxjs";
import {BuilderApiClient} from "../BuilderApiClient";
import {addNotification} from "./notifications";
import {DANGER, INFO, SUCCESS, WARNING} from "./notifications";
import {requestRoute} from "./router";
import {packageString} from "../util";

// The ansi_up module does not have TypeScript type definitions, so it needs to
// be loaded with a CommonJS require call, which will end up being handled by
// webpack.
const ansiToHtml = require("ansi_up").ansi_to_html;

export const APPEND_TO_BUILD_LOG = "APPEND_TO_BUILD_LOG";
export const FINISH_BUILD_STREAM = "FINISH_BUILD_STREAM";
export const POPULATE_BUILDS = "POPULATE_BUILDS";
export const POPULATE_BUILD_LOG = "POPULATE_BUILD_LOG";
export const POPULATE_PROJECT = "POPULATE_PROJECT";
export const SET_CURRENT_PROJECT = "SET_CURRENT_PROJECT";
export const SET_PROJECTS = "SET_PROJECTS";

export function addProject(project: Object, token: string) {
    return dispatch => {
        new BuilderApiClient(token).createProject(project).then(response => {
            dispatch(addNotification({
                title: "Project created",
                body: `Created ${project["name"]}.`,
                type: SUCCESS,
            }));
            console.log(response);
        }).catch(error => {
            dispatch(addNotification({
                title: "Failed to Create project",
                body: error.message,
                type: DANGER,
            }));
        });
    };
}

function appendToBuildLog(build, text) {
    return {
        type: APPEND_TO_BUILD_LOG,
        payload: { buildId: build.id, text: ansiToHtml(text) }
    };
}

// Fetch the list of builds for a package
export function fetchBuilds(pkg) {
    return dispatch => {
        fakeApi.get(`log/${packageString(pkg)}/builds.json`).then(response => {
            dispatch(populateBuilds(response));
            dispatch(fetchBuildLog(pkg, response));
        }).catch(error => {
            dispatch(populateBuilds([]));
        });
    };
}

// Fetch the build log for a package
function fetchBuildLog(pkg, builds) {
    return dispatch => {
        builds.forEach(build => {
            fakeApi.get(`log/${packageString(pkg)}/${build.id}.txt`).then(response => {
                if (build.status === "running") {
                    dispatch(simulateLogStream(build, response));
                } else {
                    dispatch(populateBuildLog(build.id, response));
                }
            }).catch(error => {
                dispatch(populateBuildLog(build.id, undefined));
            });
        });
    };
}

export function fetchProject(params) {
    return dispatch => {
        fakeApi.get(`projects/${params["origin"]}/${params["name"]}.json`).then(response => {
            dispatch(
                setCurrentProject(
                    Object.assign({
                        ui: { exists: true, loading: false }
                    }, response)
                )
            );
        }).catch(error => {
            dispatch(setCurrentProject({
                ui: { exists: false, loading: false }
            }));
        });
    };
}

export function fetchProjects() {
    return dispatch => {
        fakeApi.get("projects.json").then(response => {
            dispatch(setProjects(response));
        });
    };
}


function finishBuildStream(build) {
    return {
        type: FINISH_BUILD_STREAM,
        payload: { buildId: build.id, duration: 171 },
    };
}

function populateBuilds(data) {
    return {
        type: POPULATE_BUILDS,
        payload: data,
    };
}

export function populateBuildLog(id, data) {
    return {
        type: POPULATE_BUILD_LOG,
        payload: { id, data: data ? ansiToHtml(data) : undefined },
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

function simulateLogStream(build, response) {
    return dispatch => {
        // This is where we simulate a streaming build
        if (build.status === "running") {
            const o = Observable.fromArray(response.split("\n")).concatMap(x =>
                Observable.of(x).delay((() => Math.floor(Math.random() * 300))())
            );
            o.subscribe(
                x => dispatch(appendToBuildLog(build, x)),
                e => console.error(e),
                () => {
                    dispatch(finishBuildStream(build));
                    dispatch(addNotification({
                        title: "Build Complete",
                        type: SUCCESS,
                        body: `Build ${packageString(build)}#${build.id} completed successfully.`,
                    }));
                }
            );
        }

    };
}
