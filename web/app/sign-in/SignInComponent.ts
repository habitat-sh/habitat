// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {Router} from "angular2/router";
import {AppStore} from "../AppStore";
import {attemptSignIn, requestRoute} from "../actions"

@Component({
    template: `
    <div class="bldr-sign-in">
      <h2>Sign In</h2>
      <form (ngSubmit)="onSubmit(usernameOrEmail, password)">
        <input placeholder="Username or email" autofocus required #usernameOrEmail>
        <input type="password" placeholder="Password" required #password>
        <button>Sign In</button>
      </form>
    </div>
    `
})

export class SignInComponent {
  constructor(private store: AppStore) {}

  get username() {
    return this.store.getState().username;
  }

  ngOnInit() {
    if (this.store.getState().isSignedIn) {
      this.store.dispatch(
        requestRoute(["Packages", { show: "mine" }])
      );
    }
  }

  onSubmit(username) {
    this.store.dispatch(attemptSignIn(username.value));
    this.store.dispatch(requestRoute(["Packages", { show: "mine" }]));
  }
}
