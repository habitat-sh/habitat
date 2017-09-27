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

import { Component, OnInit } from "@angular/core";
import { AppStore } from "../../../AppStore";
import { deleteDockerIntegration, setDockerIntegration, setOriginPrivacySettings } from "../../../actions";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DockerCredentialsFormDialog } from "../docker-credentials-form/docker-credentials-form.dialog";
import { IntegrationDeleteConfirmDialog } from "./dialog/integration-delete-confirm/integration-delete-confirm.dialog";

@Component({
  template: require("./origin-integrations-tab.component.html")
})
export class OriginIntegrationsTabComponent {

  constructor(
    private store: AppStore,
    private credsDialog: MdDialog,
    private confirmDialog: MdDialog
  ) { }

  get integrations() {
    return this.store.getState().origins.currentIntegrations;
  }

  get origin() {
    return this.store.getState().origins.current;
  }

  get originPrivacy() {
    return this.store.getState().origins.current.default_package_visibility;
  }

  get token() {
    return this.store.getState().gitHub.authToken;
  }

  addDocker(): void {
    this.credsDialog
      .open(DockerCredentialsFormDialog, { width: "480px" })
      .afterClosed()
      .subscribe((result) => {
        if (result) {
          this.store.dispatch(setDockerIntegration(this.origin.name, result, this.token));
        }
      });
  }

  deleteDocker(name) {
    this.confirmDialog
      .open(IntegrationDeleteConfirmDialog, { width: "480px" })
      .afterClosed()
      .subscribe(confirmed => {
        if (confirmed) {
          this.store.dispatch(deleteDockerIntegration(this.origin.name, this.token, name));
        }
      });
  }

  updatePrivacy(event) {
    this.store.dispatch(setOriginPrivacySettings(event.value));
  }
}
