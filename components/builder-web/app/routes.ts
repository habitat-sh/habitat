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
import { ExplorePageComponent } from "./explore-page/ExplorePageComponent";
import { OriginCreatePageComponent } from "./origin-create-page/OriginCreatePageComponent";
import { OriginPageComponent } from "./origin-page/OriginPageComponent";
import { OriginsPageComponent } from "./origins-page/OriginsPageComponent";
import { OrganizationCreatePageComponent } from "./organization-create-page/OrganizationCreatePageComponent";
import { OrganizationsPageComponent } from "./organizations-page/OrganizationsPageComponent";
import { PackagePageComponent } from "./package-page/PackagePageComponent";
import { PackagesPageComponent } from "./packages-page/PackagesPageComponent";
import { ProjectCreatePageComponent } from "./project-create-page/ProjectCreatePageComponent";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SCMReposPageComponent } from "./scm-repos-page/SCMReposPageComponent";
import { SignInPageComponent } from "./sign-in-page/SignInPageComponent";
import { ProjectSettingsPageComponent } from "./project-settings-page/ProjectSettingsPageComponent";

export const routes: Routes = [
    {
        path: "",
        redirectTo: "/pkgs/core",
        pathMatch: "full"
    },
    {
        path: "explore",
        component: ExplorePageComponent
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
        path: "orgs",
        component: OrganizationsPageComponent,
    },
    {
        path: "orgs/create",
        component: OrganizationCreatePageComponent,
    },
    {
        path: "pkgs",
        component: PackagesPageComponent
    },
    {
        path: "pkgs/*/:name",
        component: PackagesPageComponent
    },
    {
        path: "pkgs/:origin",
        component: PackagesPageComponent
    },
    {
        path: "pkgs/:origin/:name",
        component: PackagesPageComponent,
    },
    {
        path: "pkgs/:origin/:name/:version",
        component: PackagesPageComponent,
    },
    {
        path: "pkgs/search/:query",
        component: PackagesPageComponent,
    },
    {
        path: "pkgs/:origin/:name/:version/:release",
        component: PackagePageComponent
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
        path: "sign-in",
        component: SignInPageComponent
    }
];

export const routing = RouterModule.forRoot(routes);
