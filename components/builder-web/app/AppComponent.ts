// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import {AppStore} from "./AppStore";
import {Component, OnInit} from "angular2/core";
import {ExplorePageComponent} from "./explore-page/ExplorePageComponent";
import {FooterComponent} from "./footer/FooterComponent";
import {HeaderComponent} from "./header/HeaderComponent";
import {NotificationsComponent} from "./notifications/NotificationsComponent";
import {OriginCreatePageComponent} from "./origin-create-page/OriginCreatePageComponent";
import {OriginPageComponent} from "./origin-page/OriginPageComponent";
import {OriginsPageComponent} from "./origins-page/OriginsPageComponent";
import {OrganizationCreatePageComponent} from "./organization-create-page/OrganizationCreatePageComponent";
import {OrganizationsPageComponent} from "./organizations-page/OrganizationsPageComponent";
import {PackagePageComponent} from "./package-page/PackagePageComponent";
import {PackagesPageComponent} from "./packages-page/PackagesPageComponent";
import {ProjectCreatePageComponent} from "./project-create-page/ProjectCreatePageComponent";
import {ProjectPageComponent} from "./project-page/ProjectPageComponent";
import {ProjectsPageComponent} from "./projects-page/ProjectsPageComponent";
import {RouteConfig, Router, RouterOutlet} from "angular2/router";
import {SCMReposPageComponent} from "./scm-repos-page/SCMReposPageComponent";
import {SideNavComponent} from "./side-nav/SideNavComponent";
import {SignInPageComponent} from "./sign-in-page/SignInPageComponent";
import {authenticateWithGitHub, loadSessionState, removeNotification,
    requestGitHubAuthToken, routeChange, setGitHubAuthState,
    setPackagesSearchQuery, signOut, toggleUserNavMenu} from "./actions/index";

@Component({
    directives: [FooterComponent, HeaderComponent, NotificationsComponent,
        RouterOutlet, SideNavComponent],
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

@RouteConfig([
    {
        path: "/",
        redirectTo: ["PackagesForOrigin", { origin: "core" }],
    },
    {
        path: "/explore",
        name: "Explore",
        component: ExplorePageComponent
    },
    {
        path: "/origins",
        name: "Origins",
        component: OriginsPageComponent,
    },
    {
        path: "/origins/create",
        name: "OriginCreate",
        component: OriginCreatePageComponent,
    },
    {
        path: "/origins/:origin",
        name: "Origin",
        component: OriginPageComponent,
    },
    {
        path: "/orgs",
        name: "Organizations",
        component: OrganizationsPageComponent,
    },
    {
        path: "/orgs/create",
        name: "OrganizationCreate",
        component: OrganizationCreatePageComponent,
    },
    {
        path: "/pkgs",
        name: "Packages",
        component: PackagesPageComponent
    },
    {
        path: "/pkgs/*/:name",
        name: "PackagesForName",
        component: PackagesPageComponent
    },
    {
        path: "/pkgs/:origin",
        name: "PackagesForOrigin",
        component: PackagesPageComponent
    },
    {
        path: "/pkgs/:origin/:name",
        name: "PackagesForOriginAndName",
        component: PackagesPageComponent,
    },
    {
        path: "/pkgs/:origin/:name/:version",
        name: "PackagesForOriginAndNameAndVersion",
        component: PackagesPageComponent,
    },
    {
        path: "/pkgs/search/:query",
        name: "PackagesSearch",
        component: PackagesPageComponent,
    },
    {
        path: "/pkgs/:origin/:name/:version/:release",
        name: "Package",
        component: PackagePageComponent
    },
    {
        path: "/projects",
        name: "Projects",
        component: ProjectsPageComponent
    },
    {
        path: "/projects/create",
        name: "ProjectCreate",
        component: ProjectCreatePageComponent
    },
    {
        path: "/projects/:origin/:name",
        name: "Project",
        component: ProjectPageComponent
    },
    {
        path: "/scm-repos",
        name: "SCMRepos",
        component: SCMReposPageComponent,
    },
    {
        path: "/sign-in",
        name: "SignIn",
        component: SignInPageComponent
    },
])

export class AppComponent implements OnInit {
    removeNotification: Function;
    signOut: Function;
    toggleUserNavMenu: Function;
    hideNav: boolean;

    constructor(private router: Router, private store: AppStore) {
        // Whenever the Angular route has an event, dispatch an event with the new
        // route data.
        router.subscribe(value => {
            // Don't show the side nav on the Sign In screen
            this.hideNav = value.indexOf("sign-in") !== -1;
            store.dispatch(routeChange(value));
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
    }
}
