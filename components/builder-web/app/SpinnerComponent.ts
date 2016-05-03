// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";

@Component({
    inputs: ["isSpinning", "onClick"],
    selector: "hab-spinner",
    template: `
    <span (click)="onClick()" [class.spinning]="isSpinning"
        class="hab-spinner"></span>
    `
})

export class SpinnerComponent { }
