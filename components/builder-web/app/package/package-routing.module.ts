import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { PackageComponent } from "./package/package.component";
import { PackageBuildComponent } from "./package-build/package-build.component";
import { PackageBuildsComponent } from "./package-builds/package-builds.component";
import { PackageLatestComponent } from "./package-latest/package-latest.component";
import { PackageSettingsComponent } from "./package-settings/package-settings.component";
import { PackageReadmeComponent } from "./package-readme/package-readme.component";
import { PackageReleaseComponent } from "./package-release/package-release.component";
import { PackageVersionsComponent } from "./package-versions/package-versions.component";

const routes: Routes = [
  {
      path: "pkgs/:origin/:name",
      component: PackageComponent,
      children: [
        {
          path: "",
          component: PackageVersionsComponent,
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
          path: "builds/:id",
          component: PackageBuildComponent
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
];

@NgModule({
  imports: [ RouterModule.forChild(routes) ],
  exports: [ RouterModule ]
})
export class PackageRoutingModule {}
