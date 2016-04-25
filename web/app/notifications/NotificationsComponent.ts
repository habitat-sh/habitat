// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";

@Component({
    inputs: ["notifications", "removeNotification"],
    selector: "hab-notifications",
    template: `
    <ul class="hab-notifications">
        <li *ngFor="#n of notifications; #i = index" class="{{n.type}}">
            <a class="dismiss" href="#" (click)="removeNotification(i)">&times;</a>
            <small>{{n.type}}</small>
            <h1>{{n.title}}</h1>
            <p>{{n.body}}</p>
        </li>
    </ul>`,
})

export class NotificationsComponent { }
