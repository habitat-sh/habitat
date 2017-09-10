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

import { Component, Input } from "@angular/core";
import { List } from "immutable";
import config from "../../../../config";

type KeyType = "public" | "private";

@Component({
    selector: "hab-key-list",
    template: require("./key-list.component.html")
})

export class KeyListComponent {
    @Input() keys: List<any>;
    @Input() keyType: KeyType;

    get apiUrl() { return config["habitat_api_url"]; }

    get publicKey() {
        return this.keyType === "public";
    }

    get icon() {
        if (this.publicKey) {
            return "visibility";
        } else {
            return "visibility-off";
        }
    }
}
