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

import {Component, OnInit} from "@angular/core";
import {RouterLink} from "@angular/router";
import {acceptOriginInvitation, fetchMyOriginInvitations, fetchMyOrigins}
    from "../actions/index";
import {AppStore} from "../AppStore";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-origins">
        <div class="page-title">
            <h2>
                Origins
            </h2>
            <a class="button create"
               [routerLink]="['/origins', 'create']">Add Origin</a>
        </div>
        <div *ngIf="!ui.loading" class="page-body">
            <p *ngIf="ui.errorMessage">
                Failed to load origins: {{ui.errorMessage}}
            </p>
            <div *ngIf="origins.size === 0 && !ui.errorMessage">
                <div class="hero">
                    <h3>You don't currently have any origins. Let's add one now.</h3>
                    <p>
                        <a class="button cta" [routerLink]="['/origins', 'create']">
                            Add Origin
                        </a>
                    </p>
                </div>
            </div>
            <div *ngIf="origins.size > 0">
                <ul class="hab-origins-list">
                    <li *ngFor="let origin of origins">
                        <a [routerLink]="['/origins', origin.name]"
                           class="hab-item-list">
                            <div class="hab-item-list--title">
                                <h3>{{origin.name}}</h3>
                            </div>
                        </a>
                    </li>
                </ul>
            </div>
            <div *ngIf="invitations.size > 0">
                <h3>Invitations</h3>
                <ul>
                    <li *ngFor="let invitation of invitations" class="hab-item-list hab-no-select">
                       <h3 class="hab-item-list--title">{{invitation.origin_name}}</h3>
                       <button
                           class="count"
                           (click)="acceptInvitation(invitation.id)">
                           Accept Invitation
                        </button>
                    </li>
                </ul>
            </div>
        </div>
    </div>`,
})

export class OriginsPageComponent implements OnInit {
    constructor(private store: AppStore) { }

    get invitations() { return this.store.getState().origins.myInvitations; }

    get origins() { return this.store.getState().origins.mine; }

    get ui() { return this.store.getState().origins.ui.mine; }

    private acceptInvitation(invitationId) {
        this.store.dispatch(acceptOriginInvitation(
            invitationId,
            this.store.getState().gitHub.authToken
        ));
    }

    public ngOnInit() {
        requireSignIn(this);
        this.store.dispatch(fetchMyOrigins(
            this.store.getState().gitHub.authToken
        ));
        this.store.dispatch(fetchMyOriginInvitations(
            this.store.getState().gitHub.authToken
        ));
    }
}
