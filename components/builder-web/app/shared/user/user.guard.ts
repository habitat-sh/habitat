import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { AppStore } from "../../AppStore";
import config from "../../config";

@Injectable()
export class UserLoggedInGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const hasToken = !!this.store.getState().gitHub.authToken;
    const isCodeInQueryString = new URLSearchParams(
      window.location.search.slice(1)
    ).has("code");

    if (isCodeInQueryString || hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}

@Injectable()
export class UserLoggedOutGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const hasToken = !!this.store.getState().gitHub.authToken;
    const isCodeInQueryString = new URLSearchParams(
      window.location.search.slice(1)
    ).has("code");

    if (!isCodeInQueryString || !hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}
