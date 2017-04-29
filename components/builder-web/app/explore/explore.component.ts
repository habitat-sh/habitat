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

import { AppStore } from "../AppStore";
import { Component, OnInit, OnDestroy } from "@angular/core";
import { Router } from "@angular/router";
import { fetchExplore, setLayout } from "../actions/index";

@Component({
    selector: "hab-explore",
    template: require("./explore.component.html"),
})
export class ExploreComponent implements OnInit, OnDestroy {

    constructor(
        private store: AppStore,
        private router: Router
    ) { }

    ngOnInit() {
        this.store.dispatch(fetchExplore());
        this.store.dispatch(setLayout("full"));
    }

    ngOnDestroy() {
        this.store.dispatch(setLayout("default"));
    }

    search(term) {
        this.router.navigate(["pkgs", "search", encodeURIComponent(term.trim())]);
    }
}
