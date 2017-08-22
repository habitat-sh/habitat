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

import { Component, Input, OnInit } from "@angular/core";

@Component({
    selector: "hab-origin-picker",
    template: `
    <div class="hab-origin-picker">
        <a *ngIf="isSignedIn && !currentOrigin.name"
            [routerLink]="['/origins', 'create']">
            Add Origin
        </a>
        <a class="hab-origin-picker--toggle"
           *ngIf="isSignedIn && currentOrigin.name"
           href="#"
           (click)="toggleOriginPicker()">
            {{currentOrigin.name}}
            <span *ngIf="!isOpen">
                <hab-icon symbol="chevron-down"></hab-icon>
            </span>
            <span *ngIf="isOpen">
                <hab-icon symbol="chevron-up"></hab-icon>
            </span>
        </a>
    </div>
    <ul class="hab-origin-picker--list"
        *ngIf="isOpen">
        <li class="hab-origin-picker--list--item" *ngFor="let o of myOrigins">
            <a href="#" (click)="clickSetCurrentOrigin(o)"
               [class.hab-origin-picker--list--current]="o.name == currentOrigin.name">
                {{o.name}}
                <span *ngIf="o.name == currentOrigin.name"
                    class="hab-origin-picker--list--i">
                    <hab-icon symbol="check"></hab-icon>
                </span>
            </a>
        </li>
        <li class="hab-origin-picker--list--item">
            <a (click)="toggleOriginPicker()" [routerLink]="['/origins']">
                Manage Origins&hellip;
            </a>
        </li>
    </ul>`
})

export class OriginPickerComponent implements OnInit {
    @Input() currentOrigin: Function;
    @Input() fetchMyOrigins;
    @Input() isOpen;
    @Input() isSignedIn;
    @Input() myOrigins;
    @Input() setCurrentOrigin: Function;
    @Input() toggleOriginPicker: Function;

    private clickSetCurrentOrigin(origin) {
        if (origin.name !== this.currentOrigin.name) {
            this.setCurrentOrigin(origin);
            this.toggleOriginPicker();
        }
        return false;
    }

    public ngOnInit() {
        this.fetchMyOrigins();
    }
}
