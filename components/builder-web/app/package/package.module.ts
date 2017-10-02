import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { RouterModule } from "@angular/router";
import { MdTabsModule, MdButtonModule, MdRadioModule } from "@angular/material";
import { BuildDetailComponent } from "./build-detail/build-detail.component";
import { BuildListComponent } from "./build-list/build-list.component";
import { BuildNoticeComponent } from "./build-notice/build-notice.component";
import { BuildStatusComponent } from "./build-status/build-status.component";
import { PackageBuildComponent } from "./package-build/package-build.component";
import { PackageComponent } from "./package/package.component";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackageDetailComponent } from "./package-detail/package-detail.component";
import { PackageSettingsComponent } from "./package-settings/package-settings.component";
import { PackageReadmeComponent } from "./package-readme/package-readme.component";
import { PackageReleaseComponent } from "./package-release/package-release.component";
import { PackageSidebarComponent } from "./package-sidebar/package-sidebar.component";
import { PackageVersionsComponent } from "./package-versions/package-versions.component";
import { SharedModule } from "../shared/shared.module";
import { PackageRoutingModule } from "./package-routing.module";

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    PackageRoutingModule,
    ReactiveFormsModule,
    RouterModule,
    MdTabsModule,
    MdButtonModule,
    MdRadioModule,
    SharedModule,
    FormsModule,
    ReactiveFormsModule
  ],
  declarations: [
    BuildDetailComponent,
    BuildListComponent,
    BuildNoticeComponent,
    BuildStatusComponent,
    PackageComponent,
    PackageBuildComponent,
    PackageBuildsComponent,
    PackageLatestComponent,
    PackageDetailComponent,
    PackageReadmeComponent,
    PackageReleaseComponent,
    PackageSidebarComponent,
    PackageSettingsComponent,
    PackageVersionsComponent
  ]
})
export class PackageModule {}
