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
import { BrowserModule, DomSanitizer } from "@angular/platform-browser";
import { HttpModule } from "@angular/http";
import { MdIconModule, MdIconRegistry } from "@angular/material";
import { routing } from "./routes";
import { AppStore } from "./AppStore";
import { AppComponent } from "./AppComponent";
import { BuildComponent } from "./build/build.component";
import { BuildPageComponent } from "./build-page/build-page.component";
import { BuildStatusComponent } from "./build-status/build-status.component";
import { CheckingInputComponent } from "./CheckingInputComponent";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { DashboardComponent } from "./dashboard/dashboard.component";
import { DashboardGuard } from "./dashboard/dashboard.guard";
import { ExploreComponent } from "./explore/explore.component";
import { FooterComponent } from "./footer/FooterComponent";
import { GitHubRepoPickerComponent } from "./github-repo-picker/GitHubRepoPickerComponent";
import { GravatarComponent } from "./GravatarComponent";
import { HeaderComponent } from "./header/HeaderComponent";
import { IconComponent } from "./icon/icon.component";
import { BuildListComponent } from "./build-list/build-list.component";
import { KeyAddFormComponent } from "./origin-page/KeyAddFormComponent";
import { KeyListComponent } from "./origin-page/KeyListComponent";
import { NotificationsComponent } from "./notifications/NotificationsComponent";
import { OriginCreatePageComponent } from "./origin-create-page/OriginCreatePageComponent";
import { OriginMembersTabComponent } from "./origin-page/OriginMembersTabComponent";
import { OriginPageComponent } from "./origin-page/OriginPageComponent";
import { OriginsPageComponent } from "./origins-page/OriginsPageComponent";
import { PackageBreadcrumbsComponent } from "./PackageBreadcrumbsComponent";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackageInfoComponent } from "./package-info/package-info.component";
import { PackageListComponent } from "./package-page/PackageListComponent";
import { PackagePageComponent } from "./package-page/PackagePageComponent";
import { PackageVersionsPageComponent } from "./package-versions-page/package-versions-page.component";
import { PackagesListComponent } from "./packages-list/packages-list.component";
import { PackagesPageComponent } from "./packages-page/packages-page.component";
import { ProjectSettingsPageComponent } from "./project-settings-page/ProjectSettingsPageComponent";
import { ProjectCreatePageComponent } from "./project-create-page/ProjectCreatePageComponent";
import { ProjectInfoComponent } from "./project-info/ProjectInfoComponent";
import { ProjectPageComponent } from "./project-page/ProjectPageComponent";
import { ProjectsPageComponent } from "./projects-page/ProjectsPageComponent";
import { SCMReposPageComponent } from "./scm-repos-page/SCMReposPageComponent";
import { SideNavComponent } from "./side-nav/SideNavComponent";
import { SignInPageComponent } from "./sign-in-page/SignInPageComponent";
import { TabComponent } from "./TabComponent";
import { TabsComponent } from "./TabsComponent";
import { UserNavComponent } from "./header/user-nav/UserNavComponent";
import { RepoFilterPipe } from "./pipes/repoFilter.pipe";

@NgModule({
    imports: [
        BrowserModule,
        FormsModule,
        HttpModule,
        MdIconModule,
        ReactiveFormsModule,
        routing
    ],
    declarations: [
        AppComponent,
        BuildComponent,
        BuildPageComponent,
        BuildListComponent,
        BuildStatusComponent,
        CheckingInputComponent,
        DashboardComponent,
        ExploreComponent,
        FooterComponent,
        GitHubRepoPickerComponent,
        GravatarComponent,
        HeaderComponent,
        IconComponent,
        KeyAddFormComponent,
        KeyListComponent,
        NotificationsComponent,
        OriginCreatePageComponent,
        OriginMembersTabComponent,
        OriginPageComponent,
        OriginsPageComponent,
        PackageBreadcrumbsComponent,
        PackageBuildsComponent,
        PackageLatestComponent,
        PackageInfoComponent,
        PackageListComponent,
        PackagePageComponent,
        PackageVersionsPageComponent,
        PackagesListComponent,
        PackagesPageComponent,
        ProjectCreatePageComponent,
        ProjectInfoComponent,
        ProjectPageComponent,
        ProjectsPageComponent,
        SCMReposPageComponent,
        SideNavComponent,
        SignInPageComponent,
        ProjectSettingsPageComponent,
        TabComponent,
        TabsComponent,
        UserNavComponent,
        RepoFilterPipe
    ],
    providers: [
        { provide: LocationStrategy, useClass: HashLocationStrategy },
        AppStore,
        DashboardGuard
    ],
    bootstrap: [ AppComponent ]
})

export class AppModule {
    constructor(private mdIconRegistry: MdIconRegistry, private sanitizer: DomSanitizer) {
        mdIconRegistry.addSvgIconSet(
            sanitizer.bypassSecurityTrustResourceUrl("/assets/images/icons/all.svg")
        );
    }
}
