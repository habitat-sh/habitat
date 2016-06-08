// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Control, ControlGroup, Validators} from "angular2/common";
import {ChangeDetectorRef, Component, OnInit} from "angular2/core";
import {AsyncValidator} from "./AsyncValidator";

@Component({
    inputs: ["autofocus", "availableMessage", "displayName", "form", "id",
        "isAvailable", "maxLength", "name", "notAvailableMessage", "pattern",
        "placeholder", "value"],
    selector: "hab-checking-input",
    template: `
    <div class="hab-checking-input">
        <div class="hab-checking-input--input-wrapper">
            <i class="hab-checking-input--input-icon"
                [class.loading]="control.pending"
                [class.invalid]="control.dirty && !control.pending && !control.valid"
                [class.valid]="!control.pending && control.valid">
            </i>
            <input class="hab-checking-input--input"
                [class.loading]="control.pending"
                autocomplete="off"
                autofocus="{{autofocus}}"
                id="{{id}}"
                [ngFormControl]="form.controls[name]"
                placeholder="{{placeholder}}">
        </div>
        <small class="hab-checking-input--input-msg-wrap">
            &nbsp;
            <span *ngIf="control.dirty && !control.pending && !control.valid"
                    class="hab-checking-input--input-msg invalid">
                <span *ngIf="control.errors.invalidFormat">
                    {{displayName}} must match correct format
                </span>
                <span *ngIf="control.errors.required">
                    {{displayName}} is required
                </span>
                <span *ngIf="control.errors.taken">
                    {{displayName}} {{notAvailableMessage}}
                </span>
                <span *ngIf="control.errors.maxlength">
                    Cannot be longer than {{maxLength}} characters
                </span>
            </span>
            <span *ngIf="!control.pending && control.valid"
                    class="hab-checking-input--input-msg valid">
                {{displayName}} {{availableMessage}}
            </span>
        </small>
    </div>`
})

export class CheckingInputComponent implements OnInit {
    private availableMessage: string;
    private control: Control;
    private defaultMaxLength = 255;
    private defaultPattern = "^[a-z0-9][a-z0-9_-]*$";
    private form: ControlGroup;
    private isAvailable: Function;
    private maxLength;
    private name: string;
    private notAvailableMessage: string;
    private pattern;
    private value: string;

    constructor(private cdr: ChangeDetectorRef) { }

    private patternValidator(control) {
        const value = control.value;

        if (!this.pattern || !value || value.match(this.pattern)) {
            return null;
        } else {
            return { invalidFormat: true };
        }
    }

    private takenValidator(control) {
        return new Promise(resolve => {
            // If we're empty or invalid, don't attempt to validate.
            if ((control.errors && control.errors.required) ||
                (control.errors && control.errors.invalidFormat)) {
                resolve(null);
            }

            if (this.isAvailable) {
                this.isAvailable(control.value).
                    then(() => resolve(null)).
                    catch(() => resolve({ taken: true }));
            } else {
                resolve(null);
            }
        });
    }

    public ngOnInit() {
        let validators = [
            Validators.required,
            this.patternValidator.bind(this),
        ];

        // If explicitly passed false, don't validate for max length. If one
        // wasn't passed, use the default.
        if (this.maxLength !== false) {
            this.maxLength = this.maxLength || this.defaultMaxLength;
            validators.push(Validators.maxLength(this.maxLength));
        }

        // If explicitly passed false, don't use a pattern. If one wasn't
        // passed, use the default.
        if (this.pattern !== false) {
            this.pattern = this.pattern || this.defaultPattern;
        }

        this.notAvailableMessage = this.notAvailableMessage ||
            "is already in use";
        this.availableMessage = this.availableMessage || "is available";

        this.control = new Control(
            this.value,
            Validators.compose(validators),
            AsyncValidator.debounce(control => this.takenValidator(control))
        );

        this.form.addControl(this.name, this.control);
        this.cdr.detectChanges();
    }
}