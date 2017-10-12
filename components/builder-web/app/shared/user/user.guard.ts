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

import { Injectable } from '@angular/core';
import { CanActivate, Router } from '@angular/router';
import { AppStore } from '../../app.store';
import config from '../../config';

@Injectable()
export class UserLoggedInGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const hasToken = !!this.store.getState().session.token;
    const hasCode = window.location.search.slice(1).split('&').filter((param) => {
      return !!param.match(/^code=.+/);
    }).length >= 1;

    if (hasCode || hasToken) {
      return true;
    }

    window.location.href = config['www_url'];
    return false;
  }
}
