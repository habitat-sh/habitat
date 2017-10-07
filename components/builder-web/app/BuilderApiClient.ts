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
import { parseKey } from "./util";
import { GitHubApiClient } from "./GitHubApiClient";

export class BuilderApiClient {
  private headers;
  private urlPrefix: string = `${config["habitat_api_url"]}/v1`;

    constructor(private token: string = "") {
        this.headers = token ? { "Authorization": `Bearer ${token}` } : {};
    }

    public acceptOriginInvitation(invitationId: string, originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}`, {
                headers: this.headers,
                method: "PUT",
            })
            .then(response => {
                if (response.ok) {
                    resolve(true);
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public deleteOriginInvitation(invitationId: string, originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}`, {
                headers: this.headers,
                method: "DELETE",
            })
            .then(response => {
                if (response.ok) {
                    resolve(true);
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public deleteOriginMember(origin: string, member: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin}/users/${member}`, {
                headers: this.headers,
                method: "DELETE",
            })
            .then(response => {
                if (response.ok) {
                    resolve(true);
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public ignoreOriginInvitation(invitationId: string, originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}/ignore`, {
                headers: this.headers,
                method: "PUT",
            })
            .then(response => {
                if (response.ok) {
                    resolve(true);
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public createOrigin(origin) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins`, {
                body: JSON.stringify(origin),
                headers: this.headers,
                method: "POST",
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public createOriginKey(key) {
        key = parseKey(key);
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${key.uploadPath}`, {
                body: key.text,
                headers: this.headers,
                method: "POST",
            }).then(response => {
                if (response.ok) {
                    resolve(true);
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public createProject(project) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects`, {
                body: JSON.stringify(project),
                headers: this.headers,
                method: "POST",
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public findFileInRepo(installationId: string, owner: string, repo: string, filename: string, page: number = 1, per_page: number = 100) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/ext/installations/${installationId}/search/code?q=repo:${owner}/${repo}+filename:${filename}&page=${page}&per_page=${per_page}`, {
                method: "GET",
                headers: this.headers,
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            });
        });
    }

    public updateProject(projectId, project) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${projectId}`, {
                body: JSON.stringify(project),
                headers: this.headers,
                method: "PUT",
            }).then(response => {
                if (response.ok) {
                    resolve();
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public deleteProject(projectId) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${projectId}`, {
                method: "DELETE",
                headers: this.headers
            }).then(response => {
                if (response.ok) {
                    resolve(response);
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public generateOriginKeys(origin: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin}/keys`, {
                method: "POST",
                headers: this.headers
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public getBuild(id: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/jobs/${id}`, {
                method: "GET",
                headers: this.headers
            })
                .then(response => {
                    if (response.ok) {
                        resolve(response.json());
                    } else {
                        reject(new Error(response.statusText));
                    }
                })
                .catch(error => reject(error));
        });
    }

    public getBuildLog(id: string, start = 0) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/jobs/${id}/log?start=${start}&color=true`, {
                method: "GET",
                headers: this.headers
            })
                .then(response => {
                    if (response.ok) {
                        resolve(response.json());
                    } else {
                        reject(new Error(response.statusText));
                    }
                })
                .catch(error => reject(error));
        });
    }

    public getBuilds(origin: string, name: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}/${name}/jobs`, {
                method: "GET",
                headers: this.headers
            })
                .then(response => {
                    if (response.ok) {
                        resolve(response.json());
                    } else {
                        reject(new Error(response.statusText));
                    }
                })
                .catch(error => reject(error));
        });
    }

    public getGitHubFileContent(installationId: string, owner: string, repo: string, path: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/ext/installations/${installationId}/repos/${repo}/contents/${encodeURIComponent(path)}`, {
                method: "GET",
                headers: this.headers
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            });
        });
    }

    public getProject(origin: string, name: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}/${name}`, {
                method: "GET",
                headers: this.headers
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public getProjects(origin: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}`, {
                method: "GET",
                headers: this.headers
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public getMyOriginInvitations() {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/user/invitations`, {
                headers: this.headers,
            }).then(response => {
                response.json().then(data => {
                    resolve(data["invitations"]);
                });
            }).catch(error => reject(error));
        });
    }

    public getMyOrigins() {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/user/origins`, {
                headers: this.headers,
            }).then(response => {
                response.json().then(data => {
                    if (response.ok) {
                        resolve(data["origins"]);
                    } else {
                        reject(new Error(response.statusText));
                    }
                });
            }).catch(error => reject(error));
        });
    }

    public getOrigin(originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}`, {
                headers: this.headers,
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public getOriginInvitations(originName: string) {
      return new Promise((resolve, reject) => {
        fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations`, {
          headers: this.headers,
        }).then(response => {
          if (response.ok) {
            response.json().then(data => {
              resolve(data["invitations"]);
            });
          } else {
            reject(new Error(response.statusText));
          }
        }).catch(error => reject(error));
      });
    }

    public getOriginMembers(originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/users`, {
                headers: this.headers,
            }).then(response => {
                if (response.ok) {
                    response.json().then(data => {
                        resolve(data["members"]);
                    });
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public getOriginPublicKeys(originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/keys`, {
                headers: this.headers,
            }).then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public inviteUserToOrigin(username: string, origin: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin}/users/${username}/invitations`, {
                headers: this.headers,
                method: "POST",
            }).then(response => {
                if (response.ok) {
                    resolve(true);
                } else if (response.status === 404) {
                    new GitHubApiClient(this.token).getUser(username).then(ghResponse => {
                        let msg = "This is a valid GitHub user but they have not signed into Builder yet. Once they sign in, you can invite them to your origin.";
                        reject(new Error(msg));
                    }).catch(error => reject(error));
                } else if (response.status === 409) {
                    reject(new Error(`An invitation already exists for '${username}'`));
                } else {
                    reject(new Error(response.statusText));
                }
            }).catch(error => reject(error));
        });
    }

    public isOriginAvailable(name: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${name}`, {
                headers: this.headers,
            }).then(response => {
                // Getting a 200 means it exists and is already taken.
                if (response.ok) {
                    reject(false);
                    // Getting a 404 means it does not exist and is available.
                } else if (response.status === 404) {
                    resolve(true);
                }
            }).catch(error => {
                // This happens when there is a network error. We'll say that it is
                // not available.
                reject(false);
            });
        });
    }

    public getDockerIntegration(originName: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/integrations/docker/names`, {
                headers: this.headers
            })
            .then(response => {
                if (response.ok) {
                    resolve(response.json());
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public setDockerIntegration(originName: string, credentials) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${originName}/integrations/docker/docker`, {
                headers: this.headers,
                method: "PUT",
                body: JSON.stringify(credentials)
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public getProjectIntegration(origin: string, name: string, integration: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}/${name}/integrations/${integration}/default`, {
                headers: this.headers
            })
            .then(response => {
                if (response.ok) {
                    resolve(response.json());
                }
                else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public setProjectIntegrationSettings(origin: string, name: string, integration: string, settings: any) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}/${name}/integrations/${integration}/default`, {
                headers: this.headers,
                method: "PUT",
                body: JSON.stringify(settings)
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                }
                else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public setProjectVisibility(origin: string, name: string, setting: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/projects/${origin}/${name}/${setting}`, {
                headers: this.headers,
                method: "PATCH"
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                }
                else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public deleteDockerIntegration(origin: string, name: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin}/integrations/docker/${name}`, {
                headers: this.headers,
                method: "DELETE",
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public updateOrigin(origin: any) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin.name}`, {
                headers: this.headers,
                method: "PUT",
                body: JSON.stringify(origin)
            })
            .then(response => {
                if (response.ok) {
                    resolve();
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }

    public getSigningKey(origin: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins/${origin}/secret_keys/latest`, {
                headers: this.headers
            })
            .then(response => {
                if (response.ok) {
                    resolve(response);
                } else {
                    reject(new Error(response.statusText));
                }
            })
            .catch(error => reject(error));
        });
    }
}
