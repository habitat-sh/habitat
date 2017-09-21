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
import { DashboardComponent } from "./dashboard/dashboard.component";
import { DashboardGuard } from "./dashboard/dashboard.guard";
import { ExploreComponent } from "./explore/explore.component";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SignInPageComponent } from "./sign-in-page/sign-in-page.component";
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
        path: "sign-in",
        component: SignInPageComponent
    },
    {
        path: "projects",
        component: ProjectsPageComponent
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
        path: "*",
        redirectTo: "/pkgs/core"
    }
];

export const routing = RouterModule.forRoot(routes);
