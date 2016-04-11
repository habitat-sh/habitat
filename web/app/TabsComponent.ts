// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {TabComponent} from "./TabComponent";

@Component({
    selector: "tabs",
    template: `
    <ul class="hab-tabs">
        <li *ngFor="#tab of tabs"
            [ngClass]="{ active: tab.active }"
            (click)="selectTab(tab)">{{tab.tabTitle}}</li>
    </ul>
    <ng-content></ng-content>`
})

export class TabsComponent {
    private tabs;

    constructor() {
        this.tabs = [];
    }

    addTab(tab: TabComponent) {
        if (this.tabs.length === 0) { tab.active = true; }
        this.tabs.push(tab);
    }

    selectTab(tab: TabComponent) {
        this.tabs.forEach(tab => tab.active = false);
        tab.active = true;
    }
}
