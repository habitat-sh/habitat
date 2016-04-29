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
import {isOriginAvailable} from "../builderApi";
import {requireSignIn} from "../util";

@Component({
    directives: [CheckingInputComponent],
    template: `
    <div class="hab-origin-create">
        <div class="page-title">
            <h2>Add Origin</h2>
            <p>An origin represents the organization creating the artifact.</p>
            <p *ngIf="isFirstOrigin">
                This will be your default origin although you may be a member of
                many organizations each maintaining its own set of project
                origins.
            </p>
        </div>
        <form class="page-body hab-origin-create--form"
              [ngFormModel]="form"
              (ngSubmit)="createOrigin(form.value)"
              #formValues="ngForm">
            <label for="name">Origin Name</label>
            <small>Must be unique and contain no spaces.</small>
            <small>
                Allowed characters include
                <em>a&thinsp;&ndash;&thinsp;z</em>,
                <em>A&thinsp;&ndash;&thinsp;Z</em>,
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
            <div class="hab-origin-create--checkbox">
                <input [disabled]="isFirstOrigin"
                       id="setAsDefault"
                       ngControl="default"
                       type="checkbox">
                <label class="hab-origin-create--checkbox--label"
                       for="setAsDefault">
                    Set as default origin
                </label>
            </div>
            <p><small>
                This will be the origin that is used by default by the
                web application. You can change it later.
            </small></p>
            <p *ngIf="isFirstOrigin"><small>
                Since this is the first origin you're creating, it will be
                set to default automatically.
            </small></p>
            <button [disabled]="!form.valid || creating">
                <span *ngIf="creating">Saving&hellip;</span>
                <span *ngIf="!creating">Save & Continue</span>
            </button>
        </form>
    </div>`
})

export class OriginCreatePageComponent implements AfterViewInit, OnInit {
    private form: ControlGroup;
    private maxLength = 255;
    private name: Control;
    private pattern = "^[a-z0-9\-_]+$";

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        this.form = formBuilder.group({
            default: new Control(this.isFirstOrigin),
        });
    }

    get creating() { return this.store.getState().origins.ui.current.creating; }

    get isFirstOrigin() {
        return this.store.getState().origins.mine.size === 0;
    }

    private isOriginAvailable(origin) { return isOriginAvailable(origin); }

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