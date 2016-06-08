// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Control, ControlGroup, FormBuilder, Validators} from "angular2/common";
import {AfterViewInit, Component, OnInit} from "angular2/core";
import {Observable} from "rxjs";
import {AppStore} from "../AppStore";
import {AsyncValidator} from "../AsyncValidator";
import {CheckingInputComponent} from "../CheckingInputComponent";
import {createOrigin} from "../actions/index";
import {BuilderApiClient} from "../BuilderApiClient";
import {requireSignIn} from "../util";

@Component({
    directives: [CheckingInputComponent],
    template: `
    <div class="hab-origin-create">
        <div class="page-title">
            <h2>Add Origin</h2>
            <p>An origin represents the organization creating the artifact.</p>
        </div>
        <form class="page-body hab-origin-create--form"
              [ngFormModel]="form"
              (ngSubmit)="createOrigin(form.value)"
              #formValues="ngForm">
            <label for="name">Origin Name</label>
            <small>Must be unique and contain no spaces.</small>
            <small>Must begin with a lowercase letter or number.</small>
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
    </div>`
})

export class OriginCreatePageComponent implements AfterViewInit, OnInit {
    private builderApiClient: BuilderApiClient;
    private form: ControlGroup;
    private isOriginAvailable: Function;
    private maxLength = 255;
    private name: Control;

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