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
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";
import { AppStore } from "../../../AppStore";
import { BuilderApiClient } from "../../../BuilderApiClient";
import { GenerateKeysConfirmDialog } from "./dialog/generate-keys-confirm/generate-keys-confirm.dialog";
import { KeyAddFormDialog } from "./key-add-form/key-add-form.dialog";
import { fetchOriginPublicKeys, fetchMyOrigins, generateOriginKeys } from "../../../actions/index";
import { OriginService } from "../../origin.service";
import config from "../../../config";

@Component({
    template: require("./origin-keys-tab.component.html")
})

export class OriginKeysTabComponent implements OnInit, OnDestroy {
    origin: string;
    sub: Subscription;

    constructor(
        private route: ActivatedRoute,
        private store: AppStore,
        private keyAddDialog: MdDialog,
        private keyGenerateDialog: MdDialog,
        private originService: OriginService
    ) {}

    ngOnInit() {
        this.sub = this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.store.dispatch(fetchMyOrigins(this.token));
            this.store.dispatch(fetchOriginPublicKeys(this.origin, this.token));
        });
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    get memberOfOrigin() {
        return !!this.origins.find(origin => origin["name"] === this.origin);
    }

    get origins() {
        return this.store.getState().origins.mine;
    }

    get privateKey() {
        return this.store.getState().origins.current.private_key_name;
    }

    get publicKeys() {
        return this.store.getState().origins.currentPublicKeys;
    }

    get token() {
        return this.store.getState().session.token;
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    downloadPrivateKey() {
        new BuilderApiClient(this.store.getState().session.token)
            .getSigningKey(this.origin)
            .then((response: any) => {
                response.blob().then((blob) => {
                    let header = response.headers.get("content-disposition");
                    let filename = header.split("; filename=")[1].trim().replace(/"/g, "");
                    this.download(blob, filename);
                });
            });
    }

    generateKeys() {
        this.keyGenerateDialog.open(GenerateKeysConfirmDialog, {
            width: "480px"
        })
        .afterClosed()
        .subscribe((confirmed) => {
          if (confirmed) {
            this.store.dispatch(generateOriginKeys(this.origin, this.token));
          }
        });
    }

    openKeyAddForm(type: string) {
        this.keyAddDialog.open(KeyAddFormDialog, {
            data: { type, origin: this.origin },
            width: "480px"
        });
    }

    urlFor(key) {
        return `${config["habitat_api_url"]}/v1/depot${key.location}`;
    }

    private download(blob, name) {
        const msSave = navigator.msSaveBlob;

        if (typeof msSave === "function") {
            msSave(blob, name);
        }
        else {
            let href = window.URL.createObjectURL(blob);
            let a = document.createElement("a");
            a.href = href;
            a.download = name;
            a.setAttribute("style", "display: none");
            a.onclick = (e) => { e.stopPropagation(); };
            document.body.appendChild(a);
            a.click();
            setTimeout(() => { document.body.removeChild(a); }, 100);
        }
    }
}
