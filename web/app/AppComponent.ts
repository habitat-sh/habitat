// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "./AppStore";
import {Component} from "angular2/core";
import {ExplorePageComponent} from "./explore-page/ExplorePageComponent";
import {HeaderComponent} from "./header/HeaderComponent";
import {HomePageComponent} from "./home-page/HomePageComponent";
import {LinkedAccountsPageComponent} from "./linked-accounts-page/LinkedAccountsPageComponent";
import {NotificationsComponent} from "./notifications/NotificationsComponent";
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
import {removeNotification, routeChange} from "./actions/index";

@Component({
    directives: [HeaderComponent, NotificationsComponent, RouterOutlet, SideNavComponent],
    selector: "hab",
    template: `
    <div class="hab-topbar">
        <hab-notifications [notifications]="state.notifications.all"
                            [removeNotification]="removeNotification">
        </hab-notifications>
        <hab-header [appName]="state.app.name"></hab-header>
    </div>
    <div class="hab-container">
        <hab-side-nav [route]="state.router.route"></hab-side-nav>
        <section class="hab-main">
            <router-outlet></router-outlet>
        </section>
        <footer class="hab-footer">
            <p>&copy; {{state.app.currentYear}} Chef Software, Inc. All Rights Reserved.</p>
        </footer>
    </div>`,
})

@RouteConfig([
    {
        path: "/",
        name: "Home",
        component: HomePageComponent
    },
    {
        path: "/explore",
        name: "Explore",
        component: ExplorePageComponent
    },
    {
        path: "/linked-accounts",
        name: "LinkedAccounts",
        component: LinkedAccountsPageComponent,
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

export class AppComponent {
    removeNotification: Function;

    constructor(private router: Router, private store: AppStore) {
        // Whenever the Angular route has an event, dispatch an event with the new
        // route data.
        router.subscribe(value => store.dispatch(routeChange(value)));

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
    }

    get state() { return this.store.getState(); }
}
