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

import { Routes, RouterModule } from "@angular/router";
import { BuildPageComponent } from "./build-page/build-page.component";
import { DashboardComponent } from "./dashboard/dashboard.component";
import { DashboardGuard } from "./dashboard/dashboard.guard";
import { ExploreComponent } from "./explore/explore.component";
import { OriginCreatePageComponent } from "./origin-create-page/OriginCreatePageComponent";
import { OriginPageComponent } from "./origin-page/OriginPageComponent";
import { OriginsPageComponent } from "./origins-page/OriginsPageComponent";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackagePageComponent } from "./package-page/PackagePageComponent";
import { PackageVersionsPageComponent } from "./package-versions-page/package-versions-page.component";
import { PackagesPageComponent } from "./packages-page/packages-page.component";
import { ProjectCreatePageComponent } from "./project-create-page/ProjectCreatePageComponent";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SCMReposPageComponent } from "./scm-repos-page/SCMReposPageComponent";
import { SignInPageComponent } from "./sign-in-page/SignInPageComponent";
import { ProjectSettingsPageComponent } from "./project-settings-page/ProjectSettingsPageComponent";

export const routes: Routes = [
    {
        path: "",
        component: DashboardComponent,
        canActivate: [ DashboardGuard ]
    },
    {
        path: "explore",
        component: ExploreComponent
    },
    {
        path: "builds/:id",
        component: BuildPageComponent
    },
    {
        path: "origins",
        component: OriginsPageComponent,
    },
    {
        path: "origins/create",
        component: OriginCreatePageComponent,
    },
    {
        path: "origins/:origin",
        component: OriginPageComponent,
    },
    {
        path: "pkgs/search/:query",
        component: PackagesPageComponent,
    },
    {
        path: "pkgs/:origin/:name/latest",
        component: PackageLatestComponent
    },
    {
        path: "pkgs/:origin/:name/builds",
        component: PackageBuildsComponent
    },
    {
        path: "pkgs/:origin/:name/:version/:release",
        component: PackagePageComponent
    },
    {
        path: "pkgs/:origin/:name/:version",
        component: PackagesPageComponent,
    },
    {
        path: "pkgs/:origin/:name",
        component: PackagesPageComponent
    },
    {
        path: "pkgs/:origin",
        component: PackagesPageComponent
    },
    {
        path: "pkgs",
        redirectTo: "/pkgs/core"
    },
    {
        path: "sign-in",
        component: SignInPageComponent
    },
    {
        path: "projects",
        component: ProjectsPageComponent
    },
    {
        path: "projects/create",
        component: ProjectCreatePageComponent
    },
    {
        path: "projects/:origin/:name",
        component: ProjectPageComponent
    },
    {
        path: "projects/:origin/:name/settings",
        component: ProjectSettingsPageComponent
    },
    {
        path: "scm-repos",
        component: SCMReposPageComponent,
    },
    {
        path: "*",
        redirectTo: "/pkgs/core"
    }
];

export const routing = RouterModule.forRoot(routes);
