// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

///<reference path="../node_modules/angular2/typings/browser.d.ts"/>
///<reference path='../node_modules/immutable/dist/immutable.d.ts'/>

import "angular2/bundles/angular2-polyfills";
import {AppComponent} from "./AppComponent";
import {AppStore} from "./AppStore";
import {bind, enableProdMode} from "angular2/core";
import {LocationStrategy, HashLocationStrategy, ROUTER_PROVIDERS} from "angular2/router";
import {bootstrap} from "angular2/platform/browser";
import config from "./config";

if (config["environment"] === "production") {
    enableProdMode();
}

bootstrap(AppComponent, [
    AppStore,
    ROUTER_PROVIDERS,
    // Temporarily adding this until we have nginx handle routing non-existent
    // pages.
    bind(LocationStrategy).toClass(HashLocationStrategy)
]);
