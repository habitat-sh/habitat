// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

///<reference path="../node_modules/angular2/typings/browser.d.ts"/>
///<reference path='../node_modules/immutable/dist/immutable.d.ts'/>

// Include the nav control from the main website
require("./zepto-custom");
import "./nav";

import "angular2/bundles/angular2-polyfills";
import { bind, enableProdMode } from "angular2/core";
import { bootstrap } from "angular2/platform/browser";
import { LocationStrategy, HashLocationStrategy, ROUTER_PROVIDERS }
    from "angular2/router";

import { AppComponent } from "./AppComponent";
import { AppStore } from "./AppStore";
import config from "./config";

// This mess can be taken out once we're live.
let goingToBoot = true;

if (config["environment"] === "production") {
    enableProdMode();

    // Don't load if we're on habitat.sh and the "friends" cookie is not set
    if (config["friends_only"] &&
        window.location.host.endsWith("habitat.sh") &&
        !document.cookie.includes("habitat_is_not_bldr")) {
        goingToBoot = false;
    }
}

if (goingToBoot) {
    bootstrap(AppComponent, [
        AppStore,
        ROUTER_PROVIDERS,
        // Temporarily adding this until we have nginx handle routing non-existent
        // pages.
        bind(LocationStrategy).toClass(HashLocationStrategy)
    ]);
}

