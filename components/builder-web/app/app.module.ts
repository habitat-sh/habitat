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

import { NgModule } from "@angular/core";
import { LocationStrategy, HashLocationStrategy } from "@angular/common";
import { MdButtonModule, MD_PLACEHOLDER_GLOBAL_OPTIONS } from "@angular/material";
import { BrowserModule, DomSanitizer } from "@angular/platform-browser";
import { HttpModule } from "@angular/http";
import { routing } from "./routes";
import { AppStore } from "./AppStore";
import { AppComponent } from "./AppComponent";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { DashboardComponent } from "./dashboard/dashboard.component";
import { DashboardGuard } from "./dashboard/dashboard.guard";
import { ExploreComponent } from "./explore/explore.component";
import { FooterComponent } from "./footer/FooterComponent";
import { GitHubRepoPickerComponent } from "./github-repo-picker/GitHubRepoPickerComponent";
import { GravatarComponent } from "./GravatarComponent";
import { HeaderComponent } from "./header/HeaderComponent";
import { NotificationsComponent } from "./notifications/NotificationsComponent";
import { ProjectSettingsPageComponent } from "./project-settings-page/ProjectSettingsPageComponent";
import { ProjectCreatePageComponent } from "./project-create-page/ProjectCreatePageComponent";
import { ProjectInfoComponent } from "./project-info/ProjectInfoComponent";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SCMReposPageComponent } from "./scm-repos-page/SCMReposPageComponent";
import { SideNavComponent } from "./side-nav/SideNavComponent";
import { SignInPageComponent } from "./sign-in-page/sign-in-page.component";
import { UserNavComponent } from "./header/user-nav/UserNavComponent";
import { RepoFilterPipe } from "./pipes/repoFilter.pipe";

import { OriginModule } from "./origin/origin.module";
import { PackageModule } from "./package/package.module";
import { SearchModule } from "./search/search.module";
import { SharedModule } from "./shared/shared.module";

@NgModule({
    imports: [
        BrowserModule,
        FormsModule,
        HttpModule,
        MdButtonModule,
        OriginModule,
        PackageModule,
        ReactiveFormsModule,
        SearchModule,
        SharedModule,
        routing
    ],
    declarations: [
        AppComponent,
        DashboardComponent,
        ExploreComponent,
        FooterComponent,
        GitHubRepoPickerComponent,
        GravatarComponent,
        HeaderComponent,
        NotificationsComponent,
        ProjectCreatePageComponent,
        ProjectInfoComponent,
        ProjectPageComponent,
        ProjectsPageComponent,
        SCMReposPageComponent,
        SideNavComponent,
        SignInPageComponent,
        ProjectSettingsPageComponent,
        UserNavComponent,
        RepoFilterPipe
    ],
    providers: [
        { provide: LocationStrategy, useClass: HashLocationStrategy, },
        { provide: MD_PLACEHOLDER_GLOBAL_OPTIONS, useValue: {float: "always"}},
        AppStore,
        DashboardGuard
    ],
    bootstrap: [ AppComponent ]
})

export class AppModule {

}
