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
import { setOriginPrivacySettings } from "../../../actions/index";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DockerCredentialsFormDialog } from "../docker-credentials-form/docker-credentials-form.dialog";

@Component({
    template: require("./origin-settings-tab.component.html")
})

export class OriginSettingsTabComponent {
  constructor(private store: AppStore, private dialog: MdDialog) {}

  get originPrivacy() {
    return this.store.getState().origins.current.default_package_visibility;
  }

  updatePrivacy(event) {
    this.store.dispatch(setOriginPrivacySettings(event.value));
  }

  openDialog(): void {
    let dialogRef = this.dialog.open(DockerCredentialsFormDialog, {
      width: "480px",
    });

    dialogRef.afterClosed().subscribe(result => {
      console.log(`The dialog was closed with: ${result}`);
    });
  }
}
