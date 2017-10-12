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
import { AppStore } from '../app.store';
import { setGitHubAuthState, signOut, setLayout } from '../actions/index';
import config from '../config';
import { createGitHubLoginUrl } from '../util';

@Component({
  template: require('./sign-in-page.component.html')
})
export class SignInPageComponent implements OnInit, OnDestroy {
  constructor(private store: AppStore) { }

  get wwwUrl() {
    return config['www_url'];
  }

  get gitHubJoinUrl() {
    return `${config['github_web_url']}/join`;
  }

  get gitHubLoginUrl() {
    return createGitHubLoginUrl(this.store.getState().gitHub.authState);
  }

  ngOnInit() {
    this.store.dispatch(signOut());
    this.store.dispatch(setGitHubAuthState());
    this.store.dispatch(setLayout('centered'));
  }

  ngOnDestroy() {
    this.store.dispatch(setLayout('default'));
  }
}
