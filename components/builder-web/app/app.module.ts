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

import {LocationStrategy, HashLocationStrategy} from "@angular/common";
import {NgModule, provide} from "@angular/core";
import {BrowserModule} from "@angular/platform-browser";
import {routing} from "./routes";
import {AppComponent} from "./AppComponent";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {ExplorePageComponent} from "./explore-page/ExplorePageComponent";
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
import {SCMReposPageComponent} from "./scm-repos-page/SCMReposPageComponent";
import {SignInPageComponent} from "./sign-in-page/SignInPageComponent";
import {ProjectSettingsPageComponent} from "./project-settings-page/ProjectSettingsPageComponent";

@NgModule({
    imports: [
        BrowserModule,
        FormsModule,
        ReactiveFormsModule,
        routing
    ],
    declarations: [
        AppComponent,
        ExplorePageComponent,
        OriginCreatePageComponent,
        OriginPageComponent,
        OriginsPageComponent,
        OrganizationCreatePageComponent,
        OrganizationsPageComponent,
        PackagePageComponent,
        PackagesPageComponent,
        ProjectCreatePageComponent,
        ProjectPageComponent,
        ProjectsPageComponent,
        SCMReposPageComponent,
        SignInPageComponent,
        ProjectSettingsPageComponent
    ],
    providers: [
        provide(LocationStrategy, {useClass: HashLocationStrategy})
    ],
    bootstrap: [ AppComponent ]
})

export class AppModule { }
