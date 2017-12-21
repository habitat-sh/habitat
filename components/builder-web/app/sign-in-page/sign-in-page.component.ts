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

import { Component, OnDestroy } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { AppStore } from '../app.store';
import { setLayout, signOut } from '../actions/index';
import { createGitHubLoginUrl } from '../util';
import config from '../config';

@Component({
  template: require('./sign-in-page.component.html')
})
export class SignInPageComponent implements OnDestroy {

  constructor(private store: AppStore, private title: Title) {
    store.dispatch(signOut(false));
    this.title.setTitle('Sign In | Habitat');
    this.store.dispatch(setLayout('centered'));
  }

  get wwwUrl() {
    return config['www_url'];
  }

  get gitHubJoinUrl() {
    return `${config['github_web_url']}/join`;
  }

  get gitHubLoginUrl() {
    return createGitHubLoginUrl(this.store.getState().gitHub.authState);
  }

  ngOnDestroy() {
    this.store.dispatch(setLayout('default'));
  }
}
