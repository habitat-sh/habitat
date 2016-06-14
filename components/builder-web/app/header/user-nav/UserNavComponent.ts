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
