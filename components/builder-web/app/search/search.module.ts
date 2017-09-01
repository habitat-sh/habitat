import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { RouterModule } from "@angular/router";
import { SearchComponent } from "./search/search.component";
import { SearchResultsComponent } from "./results/results.component";
import { SearchRoutingModule } from "./search-routing.module";
import { SharedModule } from "../shared/shared.module";

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    SearchRoutingModule,
    SharedModule
  ],
  declarations: [
    SearchComponent,
    SearchResultsComponent
  ]
})
export class SearchModule {}
