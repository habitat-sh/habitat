// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, Input} from "angular2/core";
import {List} from "immutable";

@Component({
    selector: "hab-key-list",
    template: `
    <p *ngIf="keys.size === 0">
        No {{type}} keys found.
    </p>
    <ul class="hab-key-list--list" *ngIf="keys.size > 0">
        <li *ngFor="#key of keys" class="hab-item-list--all-link hab-item-list">
            <a href="{{key.location}}">{{key.name}}</a>
        </li>
    </ul>`,
})

export class KeyListComponent {
    @Input() keys: List<any>;
    @Input() type: string;
}