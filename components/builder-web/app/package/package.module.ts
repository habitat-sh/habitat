import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { MdTabsModule } from "@angular/material";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackageDetailComponent } from "./package-detail/package-detail.component";
import { PackageSettingsComponent } from "./package-settings/package-settings.component";
import { PackageReadmeComponent } from "./package-readme/package-readme.component";
import { PackageReleaseComponent } from "./package-release/package-release.component";
import { PackageSidebarComponent } from "./package-sidebar/package-sidebar.component";
import { PackageVersionsComponent } from "./package-versions/package-versions.component";
import { PackageComponent } from "./package/package.component";
import { SharedModule } from "../shared/shared.module";

let routes = RouterModule.forChild([
  {
      path: "pkgs/:origin/:name",
      component: PackageComponent,
      children: [
        {
          path: "",
          pathMatch: "full",
          redirectTo: "versions"
        },
        {
          path: "versions",
          component: PackageVersionsComponent
        },
        {
          path: "latest",
          component: PackageLatestComponent
        },
        {
          path: "builds",
          component: PackageBuildsComponent
        },
        {
          path: "readme",
          component: PackageReadmeComponent
        },
        {
          path: "settings",
          component: PackageSettingsComponent
        },
        {
          path: ":version",
          component: PackageVersionsComponent
        },
        {
          path: ":version/:release",
          component: PackageReleaseComponent
        }
      ]
  }
]);

@NgModule({
  imports: [
    CommonModule,
    routes,
    RouterModule,
    MdTabsModule,
    SharedModule
  ],
  declarations: [
    PackageComponent,
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
export class PackageModule {

}
