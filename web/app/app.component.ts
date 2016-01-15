import {Component} from "angular2/core";
import {DashboardComponent} from "./dashboard.component";
import {HomeComponent} from "./home.component";
import {RouteConfig, ROUTER_DIRECTIVES} from "angular2/router";

@Component({
  directives: [ROUTER_DIRECTIVES],
  selector: "bldr",
  template: `
    <h1>bldr</h1>
    <router-outlet></router-outlet>
  `,
})

@RouteConfig([
  { path: "/", name: "Home", component: HomeComponent },
  { path: "/dashboard", name: "Dashboard", component: DashboardComponent },
])

export class AppComponent { }
