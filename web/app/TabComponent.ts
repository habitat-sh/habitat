// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component, Host} from "angular2/core";
import {TabsComponent} from "./TabsComponent";

@Component({
    selector: "tab",
    inputs: ["tabTitle"],
    template: `
    <div [hidden]="!active">
        <ng-content></ng-content>
    </div>`
})

export class TabComponent {
    public active: boolean;

    constructor(tabs: TabsComponent) {
        tabs.addTab(this);
    }
}
