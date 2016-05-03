// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";

export class GitHubApiClient {
    private urlPrefix: string = "https://api.github.com";

    constructor(private token: string) { }

    // Checks to see if a file exists at a location
    public doesFileExist(owner: string, repo: string, path: string) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/repos/${owner}/${repo}/contents/${path}?access_token=${this.token}`, {
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
}