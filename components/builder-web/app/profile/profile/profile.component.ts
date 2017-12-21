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

import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { AppStore } from '../../app.store';
import { fetchProfile, saveProfile } from '../../actions/index';

@Component({
  template: require('./profile.component.html')
})
export class ProfileComponent implements OnInit {

  constructor(private store: AppStore, private title: Title) {
    this.title.setTitle(`My Profile | Habitat`);
  }

  ngOnInit() {
    this.store.dispatch(fetchProfile(this.token));
  }

  save(form) {
    this.store.dispatch(saveProfile({ email: form.email }, this.token));
  }

  get profile() {
    return this.store.getState().users.current.profile;
  }

  get token() {
    return this.store.getState().session.token;
  }
}
