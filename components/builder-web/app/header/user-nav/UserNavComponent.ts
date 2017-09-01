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

import {Component, ElementRef, HostListener, Input} from "@angular/core";
import config from "../../config";

@Component({
    selector: "hab-user-nav",
    template: `
    <div class="main-nav--cta" *ngIf="!isSignedIn">
      <a md-button color="accent" class="button" href="{{ config['www_url'] }}/try">Get Started</a>
    </div>
    <div class="main-nav--profile" *ngIf="isSignedIn">
        <span class="main-nav--avatar">
            <img src="{{avatarUrl}}" />
        </span>
        <ul class="main-nav--dropdown" *ngIf="isOpen">
            <li>{{username}}</li>
            <li><a href="#" (click)="signOut()">
                Sign Out
            </a></li>
        </ul>
    </div>`,
})

export class UserNavComponent {
    @Input() isOpen: boolean;
    @Input() isSignedIn: boolean;
    @Input() username: string;
    @Input() avatarUrl: string;
    @Input() signOut: Function;
    @Input() toggleUserNavMenu: Function;

    constructor(private element: ElementRef) {}

    // This will be triggered on *any* click on the document. Toggle the menu
    // if it's already open and clicked on outside the dropdown, or if it's not
    // open and we click on this component's elements (the thing that opens the
    // dropdown)
    //
    // This makes it so the dropdown closes if you click somewhere you would
    // expect would make it close.
    @HostListener("document:click", ["$event"])
    private toggle(event) {
        if ((this.isOpen && !event.target.closest(".main-nav--dropdown")) ||
            (!this.isOpen && this.element.nativeElement.contains(event.target))) {
            this.toggleUserNavMenu();
        }
    }

    get config() {
        return config;
    }
}
