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
import { ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot } from '@angular/router';
import { AppStore } from '../../app.store';
import { Browser } from '../../browser';
import { signOut } from '../../actions/index';
import config from '../../config';

@Injectable()
export class SignedInGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate(route: ActivatedRouteSnapshot, routerState: RouterStateSnapshot): Promise<boolean> {
    const state = this.store.getState();
    const signedin = !!state.session.token;
    const signingIn = state.users.current.isSigningIn;
    const signInFailed = state.users.current.failedSignIn;

    return new Promise((resolve, reject) => {

      if (signedin) {
        resolve(true);
      }
      else if (signInFailed) {
        reject(() => this.redirectToSignIn());
      }
      else if (signingIn) {
        this.handleSigningIn(resolve, reject);
      }
      else {
        reject(() => {
          if (routerState.url === '/origins') {
            this.sendHome();
          }
          else {
            this.redirectToSignIn(routerState.url);
          }
        });
      }
    })
      .catch(next => next())
      .then(() => true);
  }

  private handleSigningIn(resolve, reject) {
    const unsub = this.store.subscribe(state => {

      if (state.gitHub.authToken && state.session.token) {
        const name = 'redirectPath';
        const path = Browser.getCookie(name);
        Browser.removeCookie(name);

        if (path) {
          this.router.navigate([path]);
        }

        resolve(true);
        unsub();
      }
      else if (state.users.current.failedSignIn) {
        reject(() => this.redirectToSignIn());
        unsub();
      }
    });
  }

  private sendHome() {
    Browser.redirect(config.www_url);
  }

  private redirectToSignIn(url?: string) {
    this.store.dispatch(signOut(true, url));
  }
}
