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

import { Component } from "@angular/core";
import { AppStore } from "../../../AppStore";
import { setOriginPrivacySettings, addDockerHubCredentials } from "../../../actions";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DockerCredentialsFormDialog } from "../docker-credentials-form/docker-credentials-form.dialog";
@Component({
  selector: "hab-origin-settings-tab",
  template: require("./origin-integrations-tab.component.html")
})

export class OriginIntegrationsTabComponent {
  constructor(private store: AppStore, private dialog: MdDialog) { }

  get originPrivacy() {
    return this.store.getState().origins.current.privacy;
  }

  updatePrivacy(event) {
    this.store.dispatch(setOriginPrivacySettings(event.value));
  }

  get integrations() { return this.store.getState().origin.currentIntegrations; }

  get origin() {
    return this.store.getState().origins.current;
  }

  get githubToken() {
    return this.store.getState().gitHub.authToken;
  }

  openDialog(): void {
    let dialogRef = this.dialog.open(DockerCredentialsFormDialog, {
      width: "480px",
      height: "342px"
    });

    dialogRef.afterClosed().subscribe(result => {
      if (result) {
        this.store.dispatch(addDockerHubCredentials(
          this.origin.name,
          result,
          this.githubToken
        ));
      }
    });
  }
}
