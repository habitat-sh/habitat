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

import { Component, Input, OnInit, OnDestroy } from "@angular/core";
import { MdDialog } from "@angular/material";
import { RouterLink, ActivatedRoute } from "@angular/router";
import { List } from "immutable";
import { Subscription } from "rxjs/Subscription";
import { KeyAddFormDialog } from "./key-add-form/key-add-form.dialog";
import { AppStore } from "../../../AppStore";
import { Origin } from "../../../records/Origin";
import config from "../../../config";
import { fetchOriginPublicKeys } from "../../../actions/index";
import { OriginService } from "../../origin.service";

@Component({
    selector: "hab-origin-keys-tab",
    template: require("./origin-keys-tab.component.html")
})

export class OriginKeysTabComponent implements OnInit, OnDestroy {

    origin;
    sub: Subscription;

    constructor(
        private route: ActivatedRoute,
        private store: AppStore,
        private dialog: MdDialog,
        private originService: OriginService
    ) {}

    ngOnInit() {
        this.sub = this.route.parent.params.subscribe(params => {
            this.origin = this.originService.origin(params["origin"], this.store.getState().origins.current);
        });
        this.store.dispatch(fetchOriginPublicKeys(
            this.origin.name, this.gitHubAuthToken
        ));
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    get myOrigins() {
        return this.store.getState().origins.mine;
    }

    get publicKeys() {
        return this.store.getState().origins.currentPublicKeys;
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    openKeyAddForm(type: string) {
        let dialogRef = this.dialog.open(KeyAddFormDialog, {
            data: { type, origin: this.origin.name },
            width: "480px"
        });
    }

    get privateKeyNames() {
        if (this.origin.private_key_name) {
            return List([{
                name: this.origin.private_key_name,
                location: `/origins/${this.origin.name}/secret_keys/latest`
            }]);
        } else {
            return List([]);
        }
    }

    get iAmPartOfThisOrigin() {
        return !!this.myOrigins.find(org => {
            return org["name"] === this.origin.name;
        });
    }

    get docsUrl() {
        return config["docs_url"];
    }

}
