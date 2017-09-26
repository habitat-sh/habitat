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

import { Component, OnInit } from "@angular/core";
import { Router } from "@angular/router";
import { acceptOriginInvitation, fetchMyOriginInvitations, fetchMyOrigins, ignoreOriginInvitation } from "../../actions/index";
import { AppStore } from "../../AppStore";
import config from "../../config";

@Component({
    template: require("./origins-page.component.html")
})

export class OriginsPageComponent implements OnInit {
    constructor(private store: AppStore, private router: Router) { }

    get config() {
        return config;
    }

    get invitations() {
         return this.store.getState().origins.myInvitations;
    }

    get origins() {
        return this.store.getState().origins.mine;
    }

    get ui() {
        return this.store.getState().origins.ui.mine;
    }

    acceptInvitation(invitationId, originName) {
        this.store.dispatch(acceptOriginInvitation(
            invitationId,
            originName,
            this.store.getState().gitHub.authToken
        ));
    }

    ignoreInvitation(invitationId, originName) {
        this.store.dispatch(ignoreOriginInvitation(
            invitationId,
            originName,
            this.store.getState().github.authToken
        ));
    }

    ngOnInit() {
        this.store.dispatch(fetchMyOrigins(
            this.store.getState().gitHub.authToken
        ));
        this.store.dispatch(fetchMyOriginInvitations(
            this.store.getState().gitHub.authToken
        ));
    }

    routeToOrigin(origin) {
        this.router.navigate(["/origins", origin]);
    }
}
