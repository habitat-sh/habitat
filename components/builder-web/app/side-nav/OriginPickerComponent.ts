// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["fetchMyOrigins", "isOpen", "isSignedIn", "myOrigins", "currentOrigin",
        "setCurrentOrigin", "toggleOriginPicker"],
    selector: "hab-origin-picker",
    template: `
    <div class="hab-origin-picker">
        <a *ngIf="isSignedIn && !currentOrigin.name"
            [routerLink]="['OriginCreate']">
            Add Origin
        </a>
        <a class="hab-origin-picker--toggle"
           *ngIf="isSignedIn && currentOrigin.name"
           href="#"
           (click)="toggleOriginPicker()">
            {{currentOrigin.name}}
            <span *ngIf="!isOpen"><img alt="icon arrow down" src="/node_modules/octicons/svg/chevron-down.svg" /></span>
            <span *ngIf="isOpen"><img alt="icon arrow up" src="/node_modules/octicons/svg/chevron-up.svg" /></span>
        </a>
    </div>
    <ul class="hab-origin-picker--list"
        *ngIf="isOpen">
        <li class="hab-origin-picker--list--item" *ngFor="#o of myOrigins">
            <a href="#" (click)="clickSetCurrentOrigin(o)"
               [class.hab-origin-picker--list--current]="o.name == currentOrigin.name">
                {{o.name}}
                <span *ngIf="o.name == currentOrigin.name"
                    class="hab-origin-picker--list--i">
                    <img alt="icon checkmark" src="/node_modules/octicons/svg/check.svg" />
                </span>
            </a>
        </li>
        <li class="hab-origin-picker--list--item">
            <a (click)="toggleOriginPicker()" [routerLink]="['Origins']">
                Manage Origins&hellip;
            </a>
        </li>
    </ul>`
})

export class OriginPickerComponent implements OnInit {
    private currentOrigin;
    private fetchMyOrigins: Function;
    private setCurrentOrigin: Function;
    private toggleOriginPicker: Function;

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
