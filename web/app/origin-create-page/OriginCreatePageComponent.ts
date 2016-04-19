// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Control, ControlGroup, FormBuilder, Validators} from "angular2/common";
import {Component, OnInit} from "angular2/core";
import {Observable} from "rxjs";
import {AppStore} from "../AppStore";
import {AsyncValidator} from "../AsyncValidator";
import {createOrigin} from "../actions/index";
import {isOriginAvailable} from "../builderApi";
import {requireSignIn} from "../util";

@Component({
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
            <div class="hab-origin-create--input-wrapper">
                <i class="hab-origin-create--input-icon"
                   [class.loading]="name.pending"
                   [class.invalid]="name.dirty && !name.pending && !name.valid"
                   [class.valid]="!name.pending && name.valid">
                </i>
                <input class="hab-origin-create--input"
                    [class.loading]="name.pending"
                    [class.invalid]="name.dirty && !name.pending && !name.valid"
                    [class.valid]="!name.pending && name.valid"
                    autocomplete="off"
                    id="name"
                    ngControl="name"
                    pattern="{{pattern}}"
                    required>
            </div>
            <small class="hab-origin-create--input-msg-wrap">
                &nbsp;
                <span *ngIf="name.dirty && !name.pending && !name.valid"
                      class="hab-origin-create--input-msg invalid">
                    <span *ngIf="name.errors.invalidFormat">
                        Name must match correct format
                    </span>
                    <span *ngIf="name.errors.required">
                        Name is required
                    </span>
                    <span *ngIf="name.errors.taken">
                        Name is already in use
                    </span>
                    <span *ngIf="name.errors.maxlength">
                        Cannot be longer than {{maxLength}} characters
                    </span>
                </span>
                <span *ngIf="!name.pending && name.valid"
                      class="hab-origin-create--input-msg valid">
                    Name is available
                </span>
            </small>
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

export class OriginCreatePageComponent implements OnInit {
    private form: ControlGroup;
    private maxLength = 255;
    private name: Control;
    private pattern = "^[a-z0-9\-_]+$";

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        requireSignIn(this);

        this.name = new Control(
            this.isFirstOrigin ? this.username : "",
            Validators.compose([
                Validators.required,
                Validators.maxLength(this.maxLength),
                this.patternValidator.bind(this),
            ]),
            AsyncValidator.debounce(control => this.takenValidator(control))
        );

        this.form = formBuilder.group({
            name: this.name,
            default: new Control(this.isFirstOrigin),
        });
    }

    get creating() { return this.store.getState().origins.ui.current.creating; }

    get isFirstOrigin() {
        return this.store.getState().origins.mine.size === 0;
    }

    get username() { return this.store.getState().users.current.username; }


    ngOnInit() {
        // Attempt to validate when the page loads.
        if (this.isFirstOrigin) {
            this.name.markAsDirty();
        }
    }

    private createOrigin(origin) {
        this.store.dispatch(createOrigin(origin, this.isFirstOrigin));
        return false;
    }

    private patternValidator(control) {
        const name = control.value;

        if (!name || name.match(this.pattern)) {
            return null;
        } else {
            return { invalidFormat: true };
        }
    }

    private takenValidator(control) {
        const name = control.value;

        return new Promise(resolve => {
            // If we're empty or invalid, don't attempt to validate.
            if ((control.errors && control.errors.required) ||
                (control.errors && control.errors.invalidFormat)) {
                resolve(null);
            }

            isOriginAvailable(name).
                then(() => resolve(null)).
                catch(() => resolve({ taken: true }));
        });
    }
}