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
import * as async from "async";
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
                    response.json().then((data) => {
                        let repos = [];

                        // Fetch all installations for the signed-in user, then all repositories
                        // for each installation, including subsequent pages if there are any.

                        data.installations.forEach((install) => {
                            repos.push((done) => {
                                this.getUserInstallationRepositories(install.id, 1)
                                    .then((firstPage: any) => {
                                        const totalCount = firstPage.total_count;
                                        const thisPage = firstPage.repositories;

                                        if (totalCount > thisPage.length) {
                                            const pageCount = Math.ceil(totalCount / thisPage.length);
                                            let pages = [];

                                            for (let page = 2; page <= pageCount; page++) {
                                                pages.push((done) => {
                                                    this.getUserInstallationRepositories(install.id, page)
                                                        .then((pageResults: any) => {
                                                            done(null, pageResults.repositories);
                                                        })
                                                        .catch((err) => {
                                                            done(err);
                                                        });
                                                });
                                            }

                                            async.parallel(pages, (err, additionalPages) => {
                                                if (err) {
                                                    done(err);
                                                }
                                                else {
                                                    additionalPages.forEach((p) => {
                                                        firstPage.repositories = firstPage.repositories.concat(p);
                                                    });

                                                    done(null, {
                                                        id: install.id,
                                                        app_id: install.app_id,
                                                        repos: firstPage.repositories
                                                    });
                                                }
                                            });
                                        }
                                        else {
                                            done(null, {
                                                id: install.id,
                                                app_id: install.app_id,
                                                repos: firstPage.repositories
                                            });
                                        }
                                    })
                                    .catch((err) => {
                                        done(err);
                                    });
                            });
                        });

                        async.parallel(repos, (err, installations) => {
                            if (err) {
                                reject(err);
                            }
                            else {
                                let results = [];

                                installations.map((install) => {
                                    install.repos.forEach((repo) => {
                                        results.push({
                                            repo_id: repo.id,
                                            app_id: install.app_id,
                                            installation_id: install.id,
                                            full_name: repo.full_name,
                                            org: repo.owner.login,
                                            name: repo.name,
                                            url: repo.url
                                        });
                                    });
                                });

                                resolve(Promise.resolve(results));
                            }
                        });
                    });
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => {
                reject(error);
            });
        });
    }

    private getUserInstallationRepositories(installationId: string, page: number) {
        return new Promise((resolve, reject) => {
            fetch(`${config["github_api_url"]}/user/installations/${installationId}/repositories?access_token=${this.token}&page=${page}`, {
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
