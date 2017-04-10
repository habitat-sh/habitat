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

import {Component, Input, OnInit} from "@angular/core";
import {FormControl, FormGroup, FormBuilder, Validators} from "@angular/forms";
import {List} from "immutable";

@Component({
    selector: "hab-origin-members-tab",
    template: `
    <tab tabTitle="Members">
        <div class="page-body">
            <div class="hab-origin--left">
                <div class="hab-origin-members-tab--section invite-members">
                    <h3>Invite Member</h3>
                    <form
                        #formValues="ngForm"
                        [formGroup]="form"
                        (ngSubmit)="submit(formValues.value.username)">
                        <label>Add existing users by GitHub username</label>
                        <input type="search" name="username"
                            [formControl]="form.controls['username']">
                        <div class="hab-origin-members-tab--submit">
                            <button
                                class="hab-origin-members-tab--save"
                                [disabled]="!control.valid">
                                Send invitation
                            </button>
                            <div
                                *ngIf="errorMessage"
                                class="hab-origin-members-tab--errors">
                                {{errorMessage}}
                            </div>
                        </div>
                    </form>
                </div>
                <hr>
                <div class="hab-origin-members-tab--section">
                    <h3>Pending Invitations</h3>
                    <p *ngIf="invitations.size === 0">No pending invitations.</p>
                    <ul class="pending">
                        <li *ngFor="let invitation of invitations"
                            class="hab-item-list hab-no-select">
                            <h3>{{invitation.account_name}}</h3>
                        </li>
                    </ul>
                </div>
                <hr>
                <div class="hab-origin-members-tab--section">
                    <h3>Current Members</h3>
                    <p *ngIf="members.size === 0">No Members.</p>
                    <ul>
                        <li *ngFor="let member of members"
                            class="hab-item-list hab-no-select">
                            <h3>{{member}}</h3>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="hab-origin--right">
                <p>
                    <em>Origin keys</em> ensure only authorized users (or
                    organizations) are able to push updates to packages
                    in this origin.
                </p>
                <p>
                    Read the docs for more information on
                    <a href="{{docsUrl}}/concepts-keys/">
                        managing and using keys</a>.
                </p>
            </div>
        </div>
    </tab>
    `,
})

export class OriginMembersTabComponent implements OnInit {
    @Input() docsUrl: string;
    @Input() errorMessage: string;
    @Input() invitations: List<Object>;
    @Input() members: List<Object>;
    @Input() onSubmit: Function;

    private form: FormGroup;
    private control: FormControl;

    constructor(formBuilder: FormBuilder) {
        this.form = formBuilder.group({});
    }

    private submit(username: string) {
        this.onSubmit(username);
    }

    public ngOnInit() {
        this.control = new FormControl("", Validators.required);
        this.form.addControl("username", this.control);
    }
}
