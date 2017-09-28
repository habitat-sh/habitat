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
import { updateOrigin } from "../../../actions/index";

@Component({
    template: require("./origin-settings-tab.component.html")
})

export class OriginSettingsTabComponent {

  constructor(private store: AppStore) {}

  get origin() {
    return this.store.getState().origins.current;
  }

  get privacy() {
    return this.origin.default_package_visibility;
  }

  get token() {
    return this.store.getState().gitHub.authToken;
  }

  update(event) {
    this.store.dispatch(updateOrigin({ name: this.origin.name, default_package_visibility: event.value }, this.token));
  }
}
