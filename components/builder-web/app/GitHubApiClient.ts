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

    // Checks to see if a file exists at a location
    public doesFileExist(owner: string, repo: string, path: string) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/repos/${owner}/${repo}/contents/${path}?access_token=${this.token}`, {
                method: "GET"
            }).then(response => {
                if (response.status === 404) {
                    reject(false);
                } else {
                    // Check to see if it's a file
                    response.json().then(data => {
                        if ("type" in data && data["type"] === "file") {
                            resolve(true);
                        } else {
                            reject(false);
                        }
                    });
                    resolve(true);
                }
            }).catch(error => reject(error));
        });
    }

    // Search for a filename within a repo
    public findFileInRepo(owner: string, repo: string, filename: string, page: number = 1, per_page: number = 100) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/search/code?q=repo:${owner}/${repo}+filename:${filename}&page=${page}&per_page=${per_page}`, {
                method: "GET",
                headers: {
                    "Authorization": `token ${this.token}`
                }
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            });
        });
    }

    public getFileContent(owner: string, repo: string, path: string) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/repos/${owner}/${repo}/contents/${path}`, {
                method: "GET",
                headers: {
                    "Authorization": `token ${this.token}`
                }
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            });
        });
    }

    public getUser(username: string) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/users/${username}?access_token=${this.token}`, {
                method: "GET"
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    if (response.status === 404) {
                        reject(new Error(`User '${username}' does not exist.`));
                    } else {
                        reject(new Error(response.statusText));
                    }
                }
            }).catch(error => reject(error));
        });
    }
}
