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

import {Component, Input} from "@angular/core";
import {List} from "immutable";
import config from "../config";

@Component({
    selector: "hab-key-list",
    template: `
    <p *ngIf="keys.size === 0">
        No {{type}} keys found.
    </p>
    <p *ngIf="keys.size > 0">
        Select any key to download the file.
    </p>
    <ul class="hab-key-list--list" *ngIf="keys.size > 0">
        <li *ngFor="let key of keys" class="hab-item-list--all-link hab-item-list">
            <h3><a href="{{apiUrl}}/depot{{key.location}}">
                {{key.origin}}-{{key.revision}}
            </a></h3>
        </li>
    </ul>`,
})

export class KeyListComponent {
    @Input() keys: List<any>;
    @Input() type: string;

    get apiUrl() { return config["habitat_api_url"]; }
}
