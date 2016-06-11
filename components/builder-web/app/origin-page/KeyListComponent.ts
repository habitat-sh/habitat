// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, Input} from "angular2/core";
import {List} from "immutable";
import config from "../config";

@Component({
    selector: "hab-key-list",
    template: `
    <p *ngIf="keys.size === 0">
        No {{type}} keys found.
    </p>
    <p *ngIf="keys.size > 0">
        Click on any key to download the file.
    </p>
    <ul class="hab-key-list--list" *ngIf="keys.size > 0">
        <li *ngFor="#key of keys" class="hab-item-list--all-link hab-item-list">
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
