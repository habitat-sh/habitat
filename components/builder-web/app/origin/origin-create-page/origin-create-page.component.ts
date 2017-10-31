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

import { AfterViewInit, Component } from '@angular/core';
import { FormGroup, FormBuilder } from '@angular/forms';
import { Router } from '@angular/router';
import { AppStore } from '../../app.store';
import { BuilderApiClient } from '../../client/builder-api';
import { createOrigin } from '../../actions/index';

@Component({
  template: require('./origin-create-page.component.html')
})
export class OriginCreatePageComponent implements AfterViewInit {
  form: FormGroup;
  isOriginAvailable: Function;
  maxLength = 255;
  visibility: string = 'public';

  private api: BuilderApiClient;

  constructor(private formBuilder: FormBuilder, private store: AppStore, private router: Router) {
    this.api = new BuilderApiClient(this.token);
    this.form = formBuilder.group({});

    this.isOriginAvailable = origin => {
      return this.api.isOriginAvailable(origin);
    };
  }

  ngAfterViewInit() {
    // Attempt to validate when the page loads.
    if (this.isFirstOrigin) {
      setTimeout(() => this.form.controls['name'].markAsDirty(), 1000);
    }
  }

  get creating() {
    return this.store.getState().origins.ui.current.creating;
  }

  get isFirstOrigin() {
    return this.store.getState().origins.mine.size === 0;
  }

  get token() {
    return this.store.getState().session.token;
  }

  get username() {
    return this.store.getState().users.current.username;
  }

  createOrigin(origin) {
    origin.default_package_visibility = this.visibility;

    this.store.dispatch(createOrigin(origin, this.token, this.isFirstOrigin, (origin) => {
      this.router.navigate(['/origins', origin.name]);
    }));
  }

  settingChanged(setting) {
    this.visibility = setting;
  }
}
