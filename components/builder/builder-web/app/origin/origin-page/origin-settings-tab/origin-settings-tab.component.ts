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

import { Component, OnInit, OnDestroy } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs/Subscription';
import { AppStore } from '../../../app.store';
import { updateOrigin } from '../../../actions/index';

@Component({
  template: require('./origin-settings-tab.component.html')
})

export class OriginSettingsTabComponent implements OnInit, OnDestroy {

  private sub: Subscription;

  constructor(
    private store: AppStore,
    private route: ActivatedRoute,
    private title: Title
  ) { }

  ngOnInit() {
    this.sub = this.route.parent.params.subscribe((params) => {
      this.title.setTitle(`Origins › ${params.origin} › Settings | Habitat`);
    });
  }

  ngOnDestroy() {
    if (this.sub) {
      this.sub.unsubscribe();
    }
  }

  get origin() {
    return this.store.getState().origins.current;
  }

  get visibility() {
    return this.origin.default_package_visibility;
  }

  get token() {
    return this.store.getState().session.token;
  }

  update(setting) {
    this.store.dispatch(updateOrigin({ name: this.origin.name, default_package_visibility: setting }, this.token));
  }
}
