// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import config from "./config";
import {parseKey} from "./util";

export class BuilderApiClient {
    private headers;
    private urlPrefix: string = config["habitat_api_url"];

    constructor(private token: string = "") {
        this.headers = token ? { "Authorization": `Bearer ${token}` } : {};
    }

    public acceptOriginInvitation(invitationId: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/user/invitations/${invitationId}`, {
                headers: this.headers,
                method: "PUT",
            }).then(response => {
                resolve(true);
            }).catch(error => reject(error));
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
                if  (response.ok) {
                    resolve(true);
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
                    resolve(data["origins"]);
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
                // Getting a 404 means the user does not exist.
                } else if (response.status === 404) {
                    reject(new Error(`User '${username}' does not exist`));
                } else {
                    reject(response.error);
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
}