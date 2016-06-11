// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, Input, OnInit} from "angular2/core";
import {Control, ControlGroup, FormBuilder, Validators} from "angular2/common";
import {List} from "immutable";
import {TabComponent} from "../TabComponent";

@Component({
    selector: "hab-origin-members-tab",
    directives: [TabComponent],
    template: `
    <tab tabTitle="Members">
        <div class="page-body">
            <div class="hab-origin--left">
                <div class="hab-origin-members-tab--section">
                    <h3>Invite a User</h3>
                    <form
                        #formValues="ngForm"
                        [ngFormModel]="form"
                        (ngSubmit)="submit(formValues.value.username)">
                        <label>Enter a user's GitHub username</label>
                        <input type="search" name="username"
                            [ngFormControl]="form.controls['username']">
                        <div class="hab-origin-members-tab--submit">
                            <button
                                class="hab-origin-members-tab--save"
                                [disabled]="!control.valid">
                                Invite User
                            </button>
                            <div
                                *ngIf="errorMessage"
                                class="hab-origin-members-tab--errors">
                                {{errorMessage}}
                            </div>
                        </div>
                    </form>
                </div>
                <div class="hab-origin-members-tab--section">
                    <h3>Pending Invitations</h3>
                    <p *ngIf="invitations.size === 0">No pending invitations.</p>
                    <ul>
                        <li *ngFor="#invitation of invitations"
                            class="hab-item-list hab-no-select">
                            <h3>{{invitation.account_name}}</h3>
                        </li>
                    </ul>
                </div>
                <div class="hab-origin-members-tab--section">
                    <h3>Members</h3>
                    <p *ngIf="members.size === 0">No Members.</p>
                    <ul>
                        <li *ngFor="#member of members"
                            class="hab-item-list hab-no-select">
                            <h3>{{member}}</h3>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="hab-origin--right">
                <p>
                    As an origin <em>owner</em>, you can grant admin access,
                    manage packages, and manage keys.
                </p>
                <p>
                    <em>Members</em> will be able to push updates to
                    packages that are associated with this origin.
                </p>
            </div>
        </div>
    </tab>
    `,
})

export class OriginMembersTabComponent implements OnInit {
    @Input() errorMessage: string;
    @Input() invitations: List<Object>;
    @Input() members: List<Object>;
    @Input() onSubmit: Function;

    private form: ControlGroup;
    private control: Control;

    constructor(formBuilder: FormBuilder) {
        this.form = formBuilder.group({});
    }

    private submit(username: string) {
        this.onSubmit(username);
    }

    public ngOnInit() {
        this.control = new Control("", Validators.required);
        this.form.addControl("username", this.control);
    }
}