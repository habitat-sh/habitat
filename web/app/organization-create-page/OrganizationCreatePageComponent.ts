// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {addOrg, finishAddingOrg} from "../actions/index";
import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {ControlGroup, FormBuilder, Validators} from "angular2/common";

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
        <h2>
            Add Organization
            <span *ngIf="saved">Members</span>
        </h2>
        <p>
            A namespace, name, and email are required to create an
            organization. All organization projects are public.
        </p>
        <hr>
        <div class="step1" *ngIf="!saved">
            <form [ngFormModel]="form"
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
                <button>Finish</button>
            </form>
            ${sidebar}
        </div>
    </div>`
})

export class OrganizationCreatePageComponent {
    private form: ControlGroup;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        this.form = formBuilder.group({
            namespace: ["", Validators.required],
            name: ["", Validators.required],
            email: [this.store.getState().email, Validators.required],
            website: ["", Validators.nullValidator],
        });
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
            this.store.getState().orgs.beingCreated
        ));
        return false;
    }
}
