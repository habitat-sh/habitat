// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {UserNavComponent} from "./user-nav/UserNavComponent";

@Component({
    directives: [UserNavComponent],
    inputs: ["appName"],
    selector: "hab-header",
    template: `
    <header class="hab-header">
        <h1 class="logo">{{appName}}</h1>
        <nav>
            <user-nav></user-nav>
        </nav>
    </header>`,
})

export class HeaderComponent { }
