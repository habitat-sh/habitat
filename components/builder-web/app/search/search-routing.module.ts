import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { SearchComponent } from "./search/search.component";
import { SearchResultsComponent } from "./results/results.component";

const routes: Routes = [
  {
    path: "pkgs/search/:query",
    component: SearchComponent,
  },
  {
      path: "pkgs/:origin",
      component: SearchComponent
  },
  {
      path: "pkgs",
      redirectTo: "/pkgs/core"
  }
];

@NgModule({
  imports: [ RouterModule.forChild(routes) ],
  exports: [ RouterModule ]
})
export class SearchRoutingModule {}
