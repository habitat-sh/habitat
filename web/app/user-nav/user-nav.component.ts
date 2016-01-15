import appState from "../app-state";
import {Component, Input} from "angular2/core";
import {Router, RouterLink} from "angular2/router";

@Component({
  directives: [RouterLink],
  selector: "user-nav",
  template: `
    <a *ngIf="!isSignedIn" [routerLink]="['Sign In']">Sign In</a>
    <div *ngIf="isSignedIn">
      <a class="username" href="#" (click)="toggleMenu()">{{username}}
        <span *ngIf="!isOpen">▼</span>
        <span *ngIf="isOpen">▲</span>
      </a>
      <ul *ngIf="isOpen">
        <li><a href="#" (click)="signOut()">Sign Out</a></li>
      </ul>
    <div>
  `
})

export class UserNavComponent {
  constructor(private router: Router) {}

  get isOpen() {
    return appState.get("user-nav-open");
  }

  get isSignedIn() {
    return appState.get("signed-in");
  }

  get username() {
    return appState.get("username");
  }

  signOut() {
    this.toggleMenu();
    appState.set("signed-in", false);
    this.router.navigate(["Home"]);
    return false;
  }

  toggleMenu() {
    appState.set("user-nav-open", !appState.get("user-nav-open"));
    return false;
  }
}
