// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, Input, OnInit} from "angular2/core";
import {Control, ControlGroup, FormBuilder, Validators} from "angular2/common";
import {parseKey} from "../util";

@Component({
    selector: "hab-key-add-form",
    template: `
    <form class="hab-key-add-form" [ngFormModel]="form" #formValues="ngForm"
        (ngSubmit)="submit(formValues.value.key)">
        <a class="hab-key-add-form--close" href="#" (click)="onCloseClick()">
            Close
        </a>
        <label class="hab-key-add-form--label" for="key">Key</label>
        <small>
            Paste your key here. Check the documentation for a guide on
            <a href="{{docsUrl}}/concepts-keys/">
                generating keys</a>.
        </small>
        <textarea
            autofocus
            name="key"
            [ngFormControl]="form.controls['key']"
            placeholder="Begins with '{{keyFileHeaderPrefix}}'"
            rows=6></textarea>
        <div class="hab-key-add-form--submit">
            <button class="hab-key-add-form--save" [disabled]="!form.valid">
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

    private form: ControlGroup;
    private control: Control;

    constructor(private formBuilder: FormBuilder) {
        this.form = formBuilder.group({});
    }

    private keyFormatValidator(control) {
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

    private submit(key) {
        this.uploadKey(key);
        return false;
    }

    public ngOnInit() {
        this.control = new Control(
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
}