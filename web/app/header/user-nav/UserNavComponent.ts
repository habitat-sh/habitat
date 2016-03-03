// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../../AppStore";
import {Component, Input} from "angular2/core";
import {Router, RouterLink} from "angular2/router";
import {requestRoute, signOut, toggleUserNavMenu} from "../../actions/index";

@Component({
    directives: [RouterLink],
    selector: "user-nav",
    template: `
    <nav>
        <ul>
            <li><a class="button" *ngIf="isSignUpLinkVisible" [routerLink]="['Home']">Sign Up</a></li>
            <li><a *ngIf="!isSignedIn" [routerLink]="['SignIn']">Sign In</a></li>
            <li *ngIf="isSignedIn">
                <a class="username" href="#" (click)="toggleMenu()">{{username}}
                    <span *ngIf="!isOpen">▼</span>
                    <span *ngIf="isOpen">▲</span>
                </a>
                <ul *ngIf="isOpen">
                    <li><a href="#" (click)="signOut()">Sign Out</a></li>
                </ul>
            </li>
        </ul>
    </nav>`,
})

export class UserNavComponent {
    constructor(private store: AppStore) { }

    get state() {
        return this.store.getState().user;
    }

    get isOpen() {
        return this.state.isUserNavOpen;
    }

    get isSignedIn() {
        return this.state.isSignedIn;
    }

    get username() {
        return this.state.username;
    }

    get isOnSignUpPage() {
        return window.location.pathname === "/";
    }

    get isSignUpLinkVisible() {
        return !this.isSignedIn && !this.isOnSignUpPage;
    }

    signOut() {
        this.store.dispatch(toggleUserNavMenu());
        this.store.dispatch(signOut());
        this.store.dispatch(requestRoute(["Home"]));
        return false;
    }

    toggleMenu() {
        this.store.dispatch(toggleUserNavMenu());
        return false;
    }
}
