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
