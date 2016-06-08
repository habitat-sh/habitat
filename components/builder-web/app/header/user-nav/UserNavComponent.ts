// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["isOpen", "isSignedIn", "username", "avatarUrl", "signOut",
        "toggleUserNavMenu"],
    selector: "user-nav",
    template: `
    <div class="main-nav--cta" *ngIf="!isSignedIn">
      <a class="button" [routerLink]="['SignIn']">Sign In</a>
    </div>
    <div class="main-nav--profile" *ngIf="isSignedIn">
        <a class="main-nav--avatar" href="#" (click)="toggleUserNavMenu()">
            <img height=24 src="{{avatarUrl}}" />
        </a>
        <ul class="main-nav--dropdown" *ngIf="isOpen">
            <li>{{username}}</li>
            <li><a href="#" (click)="signOut()">
                Sign Out
            </a></li>
        </ul>
    </div>`,
})

export class UserNavComponent { }
