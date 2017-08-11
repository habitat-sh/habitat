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
import {parseKey} from "../util";

@Component({
    selector: "hab-key-add-form",
    template: `
    <form class="hab-key-add-form" [formGroup]="form" #formValues="ngForm"
        (ngSubmit)="submit(formValues.value.key)">
        <a class="hab-key-add-form--close" href="#" (click)="onCloseClick()">
            Close
        </a>
        <label class="hab-key-add-form--label" for="key">Key</label>
        <small>
            Paste your key here. Check the documentation for a guide on
            <a href="{{docsUrl}}/share-packages-overview/">
                generating keys</a>.
        </small>
        <textarea
            autofocus
            name="key"
            [formControl]="form.controls['key']"
            placeholder="Begins with '{{keyFileHeaderPrefix}}'"
            rows=6></textarea>
        <div class="hab-key-add-form--submit">
            <button class="cta hab-key-add-form--save" [disabled]="!form.valid">
                Upload Key
            </button>
            <div *ngIf="control.dirty && control.errors" class="hab-key-add-form--errors">
                <span *ngIf="control.errors.required">
                    A value is required.
                </span>
                <span *ngIf="control.errors.invalidFormat">
                    This is not a valid key format.
                </span>
                <span *ngIf="control.errors.invalidType">
                    Key must begin with '{{keyFileHeaderPrefix}}'.
                </span>
                <span *ngIf="control.errors.invalidOrigin">
                    Key origin must match '{{originName}}'.
                </span>
            </div>
            <div *ngIf="errorMessage" class="hab-key-add-form--errors">
                Failed to save key: {{errorMessage}}.
            </div>
        </div>
    </form>`,
})

export class KeyAddFormComponent implements OnInit {
    @Input() docsUrl: string;
    @Input() errorMessage: string;
    @Input() keyFileHeaderPrefix: string;
    @Input() onCloseClick: Function;
    @Input() originName: string;
    @Input() uploadKey: Function;

    form: FormGroup;
    control: FormControl;

    constructor(private formBuilder: FormBuilder) {
        this.form = formBuilder.group({});
    }

    ngOnInit() {
        this.control = new FormControl(
            "",
            Validators.compose([
                Validators.required,
                this.keyFormatValidator,
                this.keyTypeValidator.bind(this),
                this.originMatchValidator.bind(this),
            ])
        );

        this.form.addControl("key", this.control);
    }

    submit(key) {
        this.uploadKey(key);
        return false;
    }

    keyFormatValidator(control) {
        if (parseKey(control.value).valid) {
            return null;
        } else {
            return { invalidFormat: true };
        }
    }

    private keyTypeValidator(control) {
        if (parseKey(control.value).type === this.keyFileHeaderPrefix) {
            return null;
        } else {
            return { invalidType: true };
        }
    }

    private originMatchValidator(control) {
        if (parseKey(control.value).origin === this.originName) {
            return null;
        } else {
            return { invalidOrigin: true };
        }
    }
}
