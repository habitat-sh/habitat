import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { OriginPageComponent } from "../origin-page/origin-page.component";
import { OriginKeysTabComponent } from "./origin-keys-tab/origin-keys-tab.component";
import { OriginMembersTabComponent } from "./origin-members-tab/origin-members-tab.component";
import { OriginPackagesTabComponent } from "./origin-packages-tab/origin-packages-tab.component";
import { OriginSettingsTabComponent } from "./origin-settings-tab/origin-settings-tab.component";

const routes: Routes = [
  {
    path: "origins/:origin",
    component: OriginPageComponent,
    children: [
      {
        path: "",
        redirectTo: "packages",
        pathMatch: "full"
      },
      {
        path: "packages",
        component: OriginPackagesTabComponent
      },
      {
        path: "keys",
        component: OriginKeysTabComponent
      },
      {
        path: "members",
        component: OriginMembersTabComponent
      },
      {
        path: "settings",
        component: OriginSettingsTabComponent
      },
      {
        path: "**",
        redirectTo: "packages"
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class OriginPageRoutingModule {}
