import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { AppStore } from "../AppStore";
import config from "../config";

@Injectable()
export class DashboardGuard implements CanActivate {

  constructor(
    private store: AppStore,
    private router: Router) {}

  canActivate() {
    let state = this.store.getState();
    let signingIn = state.users.current.isSigningIn;
    let hasToken = !!state.gitHub.authToken;

    if (signingIn || hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}
