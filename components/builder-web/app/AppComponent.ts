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

import {Subscription} from "rxjs/Subscription";
import {AppStore} from "./AppStore";
import {Component, OnInit, OnDestroy} from "@angular/core";
import {Router, RouterOutlet} from "@angular/router";
import {authenticateWithGitHub, loadSessionState, removeNotification,
    requestGitHubAuthToken, routeChange, setGitHubAuthState,
    setPackagesSearchQuery, signOut, toggleUserNavMenu, loadFeatureFlags} from "./actions/index";

@Component({
    selector: "hab",
    template: `
    <div id="main-nav">
        <hab-notifications [notifications]="state.notifications.all"
                           [removeNotification]="removeNotification">
        </hab-notifications>
        <hab-header [appName]="state.app.name"
                    [isUserNavOpen]="user.isUserNavOpen"
                    [isSignedIn]="user.isSignedIn"
                    [username]="user.username"
                    [avatarUrl]="user.gitHub.get('avatar_url')"
                    [signOut]="signOut"
                    [toggleUserNavMenu]="toggleUserNavMenu"></hab-header>
    </div>
    <div class="hab-container">
        <hab-side-nav [isSignedIn]="user.isSignedIn"
                      [route]="state.router.route"
                      *ngIf="!hideNav">
        </hab-side-nav>
        <section class="hab-main" [ngClass]="{centered: hideNav}">
            <router-outlet></router-outlet>
        </section>
    </div>
    <hab-footer [currentYear]="state.app.currentYear"></hab-footer>`,
})

export class AppComponent implements OnInit, OnDestroy {
    removeNotification: Function;
    signOut: Function;
    toggleUserNavMenu: Function;
    hideNav: boolean;
    private sub: Subscription;

    constructor(private router: Router, private store: AppStore) {
        // Whenever the Angular route has an event, dispatch an event with the new
        // route data.
        this.sub = this.router.events.subscribe(event => {
            let eventName = event.toString();
            // Don't show the side nav on the Sign In screen
            this.hideNav = eventName.indexOf("sign-in") !== -1;
            store.dispatch(routeChange(eventName));
            // Clear the package search when the route changes
            store.dispatch(setPackagesSearchQuery(""));
        });

        // Listen for changes on the state.
        store.subscribe(state => {
            // If the state has a requestedRoute attribute, use the router to navigate
            // to the route that was requested.
            const requestedRoute = state.router.requestedRoute;
            if (requestedRoute) { router.navigate(requestedRoute); }
        });

        this.removeNotification = function(i) {
            this.store.dispatch(removeNotification(i));
            return false;
        }.bind(this);

        this.signOut = function() {
            this.store.dispatch(signOut());
            return false;
        }.bind(this);

        this.toggleUserNavMenu = function() {
            this.store.dispatch(toggleUserNavMenu());
            return false;
        }.bind(this);

    }

    get origin() { return this.state.origins.current; }

    get state() { return this.store.getState(); }

    get user() { return this.state.users.current; }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    ngOnInit() {
        // Populate the GitHub authstate (used to get a token) in SessionStorage
        // either with what's there already, or with a new UUID.
        this.store.dispatch(setGitHubAuthState());

        // Load up the session state when we load the page
        this.store.dispatch(loadSessionState());

        // Request an auth token from GitHub. This doesn't do anything if the
        // "code" and "state" query parameters are not present.
        this.store.dispatch(requestGitHubAuthToken(
            window.location.search,
            this.store.getState().gitHub.authState
        ));

        // When the page loads attempt to authenticate with GitHub. If there
        // is no token stored in session storage, this won't do anything.
        this.store.dispatch(
            authenticateWithGitHub(this.state.gitHub.authToken)
        );

        this.store.dispatch(loadFeatureFlags());
    }
}
