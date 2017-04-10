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

import {FormControl, FormGroup, FormBuilder, Validators} from "@angular/forms";
import {AfterViewInit, Component, OnInit} from "@angular/core";
import {AppStore} from "../AppStore";
import {AsyncValidator} from "../AsyncValidator";
import {createOrigin} from "../actions/index";
import {BuilderApiClient} from "../BuilderApiClient";
import {requireSignIn} from "../util";

@Component({
    template: `
    <div class="hab-origin-create">
        <div class="page-title">
            <h2>Add Origin</h2>
        </div>
        <div class="page-body has-sidebar">
            <div class="page-body--main">
                <form class="hab-origin-create--form"
                      [formGroup]="form"
                      (ngSubmit)="createOrigin(form.value)"
                      #formValues="ngForm">
                    <label for="name">Origin Name</label>
                    <small>Must be unique, contain no spaces, and begin with a lowercase letter or number.</small>
                    <small>
                        Allowed characters include
                        <em>a&thinsp;&ndash;&thinsp;z</em>,
                        <em>0&thinsp;&ndash;&thinsp;9</em>,
                        <em>_</em>, and <em>-</em>.
                        No more than {{maxLength}} characters.
                    </small>
                    <hab-checking-input displayName="Name"
                                        [form]="form"
                                        id="origin-name"
                                        [isAvailable]="isOriginAvailable"
                                        name="name"
                                        [value]="isFirstOrigin ? username : ''">
                    </hab-checking-input>
                    <button [disabled]="!form.valid || creating">
                        <span *ngIf="creating">Saving&hellip;</span>
                        <span *ngIf="!creating">Save & Continue</span>
                    </button>
                </form>
            </div>
            <div class="page-body--sidebar">
                <p>An <em>origin</em> represents the organization creating the package.
                Every package is associated to an origin.</p>
                <p>You will be able to invite members and upload keys after creating
                your origin.</p>
            </div>
        </div>
    </div>`
})

export class OriginCreatePageComponent implements AfterViewInit, OnInit {
    private builderApiClient: BuilderApiClient;
    private form: FormGroup;
    private isOriginAvailable: Function;
    private maxLength = 255;
    private name: FormControl;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        this.form = formBuilder.group({});
        this.builderApiClient = new BuilderApiClient(
            this.store.getState().gitHub.authToken
        );
        this.isOriginAvailable = origin => {
            return this.builderApiClient.isOriginAvailable(origin);
        };
    }

    get creating() { return this.store.getState().origins.ui.current.creating; }

    get isFirstOrigin() {
        return this.store.getState().origins.mine.size === 0;
    }

    get username() { return this.store.getState().users.current.username; }

    ngAfterViewInit() {
        // Attempt to validate when the page loads.
        if (this.isFirstOrigin) {
            setTimeout(() => this.form.controls["name"].markAsDirty(), 1000);
        }
    }

    ngOnInit() {
        requireSignIn(this);
    }

    private createOrigin(origin) {
        this.store.dispatch(createOrigin(
            origin,
            this.store.getState().gitHub.authToken,
            this.isFirstOrigin
        ));
        return false;
    }
}
