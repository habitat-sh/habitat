// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import config from "./config";

export class BuilderApiClient {
    private headers;
    private urlPrefix: string = config["habitat_api_url"];

    constructor(private token: string = "") {
        this.headers = token ? { "Authorization": `Bearer ${token}` } : {};
    }

    public createOrigin(origin) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/depot/origins`, {
                body: JSON.stringify(origin),
                headers: this.headers,
                method: "POST",
            }).then(response => {
                resolve(response.json());
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