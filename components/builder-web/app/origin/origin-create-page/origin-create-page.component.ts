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

import { AfterViewInit, Component, OnInit } from "@angular/core";
import { FormControl, FormGroup, FormBuilder, Validators } from "@angular/forms";
import { AppStore } from "../../AppStore";
import { AsyncValidator } from "../../AsyncValidator";
import { BuilderApiClient } from "../../BuilderApiClient";
import { createOrigin } from "../../actions/index";
import { requireSignIn } from "../../util";

@Component({
    template: require("./origin-create-page.component.html")
})

export class OriginCreatePageComponent implements AfterViewInit, OnInit {
    form: FormGroup;
    isOriginAvailable: Function;
    maxLength = 255;

    private api: BuilderApiClient;
    private name: FormControl;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        this.api = new BuilderApiClient(this.token);

        this.form = formBuilder.group({
            generateKeys: true
        });

        this.isOriginAvailable = origin => {
            return this.api.isOriginAvailable(origin);
        };
    }

    ngOnInit() {
        requireSignIn(this);
    }

    ngAfterViewInit() {
        // Attempt to validate when the page loads.
        if (this.isFirstOrigin) {
            setTimeout(() => this.form.controls["name"].markAsDirty(), 1000);
        }
    }

    get creating() {
        return this.store.getState().origins.ui.current.creating;
    }

    get isFirstOrigin() {
        return this.store.getState().origins.mine.size === 0;
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get username() {
        return this.store.getState().users.current.username;
    }

    createOrigin(origin) {
        this.store.dispatch(createOrigin(origin, this.token, this.form.get("generateKeys").value, this.isFirstOrigin));
    }
}
