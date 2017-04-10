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

import {addOrg, cancelOrgInvitation, inviteMemberToOrg, finishAddingOrg,
    performOrgMemberSearch, toggleMemberActionMenu} from "../actions/index";

import {AppStore} from "../AppStore";
import {Component} from "@angular/core";
import {FormGroup, FormBuilder, Validators} from "@angular/forms";
import {requireSignIn} from "../util";

// This shows up on both steps. It could be broken out into a Component
// is really too simple for that.
const sidebar = `
    <nav>
        <ul>
            <li [class.active]="!saved">1. Create your organization</li>
            <li [class.active]="saved">2. Invite organization members</li>
        </ul>
        <p>
            As an organization <em>owner</em>, you'll have access to all
            projects and settings.
        <p>
            You'll be able to change this info at any time.
        </p>
    </nav>`;

@Component({
    template: `
    <div class="hab-organization-create">
        <div class="page-title">
            <h2>
                Add Organization
                <span *ngIf="saved">Members</span>
            </h2>
            <p>
                A namespace, name, and email are required to create an
                organization. All organization projects are public.
            </p>
        </div>
        <div class="page-body">
            <div class="step1" *ngIf="!saved">
                <form [formGroup]="form"
                    (ngSubmit)="addOrg(form.value)"
                    #formValues="ngForm">
                    <div class="ns">
                        <label for="namespace">Namespace</label>
                        <input ngControl="namespace" required id="namespace">
                    </div>
                    <div class="name">
                        <label for="name">Full Name</label>
                        <input ngControl="name" required id="name">
                    </div>
                    <label for="email">Email Address</label>
                    <small>Default is your user email address</small>
                    <input ngControl="email" id="email" type="email" required>
                    <label for="website">Website</label>
                    <input ngControl="website" id="website" type="url">
                    <button>Save & Add Members</button>
                </form>
                ${sidebar}
            </div>
            <div class="step2" *ngIf="saved">
                <form (ngSubmit)="finishAddingOrg()">
                    <hab-org-members [org]="org"
                                     [cancelInvitation]="cancelOrgInvitation"
                                     [inviteMemberToOrg]="inviteMemberToOrg"
                                     [performSearch]="performOrgMemberSearch"
                                     [toggleMemberActionMenu]="toggleMemberActionMenu">
                    </hab-org-members>
                    <button>Finish</button>
                </form>
                ${sidebar}
            </div>
        </div>
    </div>`
})

export class OrganizationCreatePageComponent {
    private form: FormGroup;
    private cancelOrgInvitation: Function;
    private inviteMemberToOrg: Function;
    private performOrgMemberSearch: Function;
    private toggleMemberActionMenu: Function;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        requireSignIn(this);

        this.form = formBuilder.group({
            namespace: ["", Validators.required],
            name: ["", Validators.required],
            email: [this.store.getState().email, Validators.required],
            website: ["", Validators.nullValidator],
        });

        this.cancelOrgInvitation = (index) =>
            this.store.dispatch(cancelOrgInvitation(index));

        this.inviteMemberToOrg = (member, index) =>
            this.store.dispatch(inviteMemberToOrg(member, index));

        this.performOrgMemberSearch = (index) =>
            this.store.dispatch(performOrgMemberSearch(index));

        this.toggleMemberActionMenu = (index) =>
            this.store.dispatch(toggleMemberActionMenu(index));
    }

    get org() {
        return this.store.getState().orgs.current;
    }

    get saved() {
        return this.store.getState().orgs.ui.create.saved;
    }

    private addOrg(values) {
        this.store.dispatch(addOrg(values));
        return false;
    }

    private finishAddingOrg() {
        this.store.dispatch(finishAddingOrg(
            this.store.getState().orgs.current
        ));
        return false;
    }
}
