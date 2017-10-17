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

import 'whatwg-fetch';
import config from './config';
import { parseKey } from './util';
import { GitHubApiClient } from './GitHubApiClient';
import { AppStore } from './app.store';
import { requestRoute, addNotification } from './actions/index';
import { WARNING } from './actions/notifications';

export class BuilderApiClient {
  private headers;
  private urlPrefix: string = `${config['habitat_api_url']}/v1`;
  private store: AppStore;

  constructor(private token: string = '') {
    this.headers = token ? { 'Authorization': `Bearer ${token}` } : {};
    this.store = new AppStore();
  }

  public acceptOriginInvitation(invitationId: string, originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}`, {
        headers: this.headers,
        method: 'PUT',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public deleteOriginInvitation(invitationId: string, originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}`, {
        headers: this.headers,
        method: 'DELETE',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public deleteOriginMember(origin: string, member: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin}/users/${member}`, {
        headers: this.headers,
        method: 'DELETE',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public ignoreOriginInvitation(invitationId: string, originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations/${invitationId}/ignore`, {
        headers: this.headers,
        method: 'PUT',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public createOrigin(origin) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins`, {
        body: JSON.stringify(origin),
        headers: this.headers,
        method: 'POST',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public createOriginKey(key) {
    key = parseKey(key);
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${key.uploadPath}`, {
        body: key.text,
        headers: this.headers,
        method: 'POST',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public createProject(project) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects`, {
        body: JSON.stringify(project),
        headers: this.headers,
        method: 'POST',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public findFileInRepo(installationId: string, owner: string, repo: string, path: string, page: number = 1, per_page: number = 100) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/ext/installations/${installationId}/repos/${repo}/contents/${encodeURIComponent(path)}`, {
        method: 'GET',
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public updateProject(projectId, project) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${projectId}`, {
        body: JSON.stringify(project),
        headers: this.headers,
        method: 'PUT',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public deleteProject(projectId) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${projectId}`, {
        method: 'DELETE',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public generateOriginKeys(origin: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin}/keys`, {
        method: 'POST',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getBuild(id: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/jobs/${id}`, {
        method: 'GET',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getBuildLog(id: string, start = 0) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/jobs/${id}/log?start=${start}&color=true`, {
        method: 'GET',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getBuilds(origin: string, name: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}/${name}/jobs`, {
        method: 'GET',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getProject(origin: string, name: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}/${name}`, {
        method: 'GET',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getProjects(origin: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}`, {
        method: 'GET',
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getMyOriginInvitations() {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/user/invitations`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          response.json().then(data => {
            resolve(data['invitations']);
          });
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getMyOrigins() {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/user/origins`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          response.json().then(data => {
            if (response.ok) {
              resolve(data['origins']);
            } else {
              reject(new Error(response.statusText));
            }
          });
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getOrigin(originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getOriginInvitations(originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/invitations`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            response.json().then(data => {
              resolve(data['invitations']);
            });
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getOriginMembers(originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/users`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            response.json().then(data => {
              resolve(data['members']);
            });
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getOriginPublicKeys(originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/keys`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public inviteUserToOrigin(username: string, origin: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin}/users/${username}/invitations`, {
        headers: this.headers,
        method: 'POST',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(true);
          } else if (response.status === 404) {
            new GitHubApiClient(this.token).getUser(username).then(ghResponse => {
              let msg = 'This is a valid GitHub user but they have not signed into Builder yet. Once they sign in, you can invite them to your origin.';
              reject(new Error(msg));
            }).catch(error => reject(error));
          } else if (response.status === 409) {
            reject(new Error(`An invitation already exists for '${username}'`));
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public isOriginAvailable(name: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${name}`, {
        headers: this.headers,
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          // Getting a 200 means it exists and is already taken.
          if (response.ok) {
            reject(false);
            // Getting a 404 means it does not exist and is available.
          } else if (response.status === 404) {
            resolve(true);
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getDockerIntegration(originName: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/integrations/docker/names`, {
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public setDockerIntegration(originName: string, credentials) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${originName}/integrations/docker/docker`, {
        headers: this.headers,
        method: 'PUT',
        body: JSON.stringify(credentials)
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getProjectIntegration(origin: string, name: string, integration: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}/${name}/integrations/${integration}/default`, {
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response.json());
          }
          else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public setProjectIntegrationSettings(origin: string, name: string, integration: string, settings: any) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}/${name}/integrations/${integration}/default`, {
        headers: this.headers,
        method: 'PUT',
        body: JSON.stringify(settings)
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          }
          else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public setProjectVisibility(origin: string, name: string, setting: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/projects/${origin}/${name}/${setting}`, {
        headers: this.headers,
        method: 'PATCH'
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          }
          else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public deleteDockerIntegration(origin: string, name: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin}/integrations/docker/${name}`, {
        headers: this.headers,
        method: 'DELETE',
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public updateOrigin(origin: any) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin.name}`, {
        headers: this.headers,
        method: 'PUT',
        body: JSON.stringify(origin)
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve();
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public getSigningKey(origin: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/depot/origins/${origin}/secret_keys/latest`, {
        headers: this.headers
      })
        .then(response => this.handleUnauthorized(response, reject))
        .then(response => {
          if (response.ok) {
            resolve(response);
          } else {
            reject(new Error(response.statusText));
          }
        })
        .catch(error => this.handleError(error, reject));
    });
  }

  public validateDockerCredentials(username: string, password: string) {
    return new Promise((resolve, reject) => {
      fetch(`${this.urlPrefix}/ext/integrations/docker/credentials/validate`, {
        headers: this.headers,
        method: 'POST',
        body: JSON.stringify({ username, password })
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

  private handleError(error, reject) {
    const store = this.store;
    const state = store.getState();
    store.dispatch(requestRoute(['/sign-in']));
    reject(error);

    if (state.session.token) {
      setTimeout(() => {
        store.dispatch(addNotification({
          title: 'Session Expired',
          body: 'Please sign in again.',
          type: WARNING
        }));
      }, 1000);
    }
  }

  private handleUnauthorized(response, reject) {
    if (response.status === 401) {
      throw new Error('Unauthorized');
    }

    return response;
  }
}
