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

import { Component, Input, OnInit } from "@angular/core";
import { FormControl, FormGroup, FormBuilder, Validators } from "@angular/forms";
import { parseKey } from "../../../../util";
import { AppStore } from "../../../../AppStore";
import config from "../../../../config";
import {
    uploadOriginPrivateKey,
    uploadOriginPublicKey,
    setCurrentOriginAddingPrivateKey,
    setCurrentOriginAddingPublicKey
} from "../../../../actions/index";

@Component({
    selector: "hab-key-add-form",
    template: require("./key-add-form.component.html")
})

export class KeyAddFormComponent implements OnInit {
    @Input() originName: string;
    @Input() type: string;

    form: FormGroup;
    control: FormControl;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
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
        if (this.type === "public") {
            this.uploadPublicKey(key);
        } else {
            this.uploadPrivateKey(key);
        }
        return false;
    }

    keyFormatValidator(control) {
        if (parseKey(control.value).valid) {
            return null;
        } else {
            return { invalidFormat: true };
        }
    }

    onCloseClick() {
        if (this.type === "public") {
            this.onPublicKeyCloseClick();
        } else {
            this.onPrivateKeyCloseClick();
        }
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    get docsUrl() {
        return config["docs_url"];
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    get errorMessage() {
        if (this.type === "public") {
            this.ui.publicKeyErrorMessage;
        } else {
            return this.ui.privateKeyErrorMessage;
        }
    }

    get keyFileHeaderPrefix() {
        if (this.type === "public") {
            return "SIG-PUB-1";
        } else {
            return "SIG-SEC-1";
        }
    }

    uploadPrivateKey(key) {
        this.store.dispatch(uploadOriginPrivateKey(key, this.gitHubAuthToken));
    }

    uploadPublicKey(key) {
        this.store.dispatch(uploadOriginPublicKey(key, this.gitHubAuthToken));
    }

    onPrivateKeyCloseClick() {
        this.store.dispatch(setCurrentOriginAddingPrivateKey(false));
        return false;
    }

    onPublicKeyCloseClick() {
        this.store.dispatch(setCurrentOriginAddingPublicKey(false));
        return false;
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
