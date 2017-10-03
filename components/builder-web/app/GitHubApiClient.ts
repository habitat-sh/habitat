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

import "whatwg-fetch";
import config from "./config";

export interface File {
    name: string;
    path: string;
    sha: string;
    url: string;
    git_url: string;
    html: string;
    repository: object;
    score: number;
}

export interface FileResponse {
    total_count: number;
    incomplete_results: boolean;
    items?: Array<File>;
}

export class GitHubApiClient {
    constructor(private token: string) { }

    public getUser(username: string) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/users/${username}?access_token=${this.token}`, {
                method: "GET"
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    if (response.status === 404) {
                        reject(new Error(`GitHub user '${username}' does not exist.`));
                    } else {
                        reject(new Error(response.statusText));
                    }
                }
            }).catch(error => reject(error));
        });
    }

    public getUserInstallations() {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/user/installations?access_token=${this.token}`, {
                method: "GET",
                headers: {
                    "Accept": [
                        "application/vnd.github.v3+json",
                        "application/vnd.github.machine-man-preview+json"
                    ]
                }
            })
            .then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => {
                reject(error);
            });
        });
    }

    public getUserInstallationRepositories(installationId: string) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/user/installations/${installationId}/repositories?access_token=${this.token}`, {
                method: "GET",
                headers: {
                    "Accept": [
                        "application/vnd.github.v3+json",
                        "application/vnd.github.machine-man-preview+json"
                    ]
                }
            })
            .then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => {
                reject(error);
            });
        });
    }
}
