// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import "whatwg-fetch";
import config from "./config";

export class BuilderApiClient {
    private urlPrefix: string = config["habitat_api_url"];

    constructor(private token: string) { }

    public createOrigin(origin) {
        return new Promise((resolve, reject) => {
            fetch(`${this.urlPrefix}/origins`, {
                body: JSON.stringify(origin),
                headers: {
                    "Authorization": `Bearer ${this.token}`,
                },
                method: "POST",
            }).then(response => {
                resolve(response.json());
            }).catch(error => reject(error));
        });
    }
}