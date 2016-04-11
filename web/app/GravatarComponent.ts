// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
const md5 = require("blueimp-md5");

@Component({
    selector: "gravatar",
    inputs: ["defaultStyle", "email", "size"],
    template: `
    <img class="hab-gravatar"
        width="{{size || DEFAULT_SIZE}}" height="{{size || DEFAULT_SIZE}}"
        src='{{gravatarUrl(defaultStyle, email, size)}}'>`
})

export class GravatarComponent {
    private DEFAULT_SIZE = 80;

    private gravatarUrl(defaultStyle: string = "retro",
        email: string, size: number = this.DEFAULT_SIZE) {

        defaultStyle = encodeURIComponent(defaultStyle || "retro");
        return `https://secure.gravatar.com/avatar/
            ${md5(email.toLowerCase().trim())}?
            d=${defaultStyle}&
            s=${size}`.replace(/\s/g, "");
    }
}
