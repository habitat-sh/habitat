import { Injectable } from "@angular/core";
import { CanActivate, Router } from "@angular/router";
import { AppStore } from "../../AppStore";
import config from "../../config";

@Injectable()
export class UserLoggedInGuard implements CanActivate {

  constructor(private store: AppStore, private router: Router) { }

  canActivate() {
    const hasToken = !!this.store.getState().session.token;
    const hasCode = window.location.search.slice(1).split("&").filter((param) => {
      return !!param.match(/^code=.+/);
    }).length >= 1;

    if (hasCode || hasToken) {
      return true;
    }

    window.location.href = config["www_url"];
    return false;
  }
}
