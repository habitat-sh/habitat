// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["isOpen", "isSignedIn", "username", "avatarUrl", "signOut",
        "toggleUserNavMenu"],
    selector: "user-nav",
    template: `
    <li><a *ngIf="!isSignedIn" [routerLink]="['SignIn']">Sign In</a></li>
    <li *ngIf="isSignedIn">
        <a class="username" href="#" (click)="toggleUserNavMenu()">
            <img height=24 src="{{avatarUrl}}" />
            {{username}}
            <span *ngIf="!isOpen">▼</span>
            <span *ngIf="isOpen">▲</span>
        </a>
        <ul *ngIf="isOpen">
            <li><a href="#" (click)="signOut()">
                Sign Out
            </a></li>
        </ul>
    </li>`,
})

export class UserNavComponent { }
