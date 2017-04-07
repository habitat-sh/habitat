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

@Component({
    selector: "hab-notifications",
    template: `
    <ul class="hab-notifications">
        <li *ngFor="let n of notifications; let i = index" class="{{n.type}}">
            <a class="dismiss" href="#" (click)="removeNotification(i)">&times;</a>
            <small>{{n.type}}</small>
            <h1>{{n.title}}</h1>
            <p>{{n.body}}</p>
        </li>
    </ul>`
})
export class NotificationsComponent {
    @Input() notifications;
    @Input() removeNotification: Function;
}
