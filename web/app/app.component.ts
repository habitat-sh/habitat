import {Component} from "angular2/core";
import {DashboardComponent} from "./dashboard/dashboard.component";
import {HomeComponent} from "./home/home.component";
import {RouteConfig, ROUTER_DIRECTIVES} from "angular2/router";
import {SignInComponent} from "./sign-in/sign-in.component";
import {UserNavComponent} from "./user-nav/user-nav.component";

@Component({
  directives: [ROUTER_DIRECTIVES, UserNavComponent],
  selector: "bldr",
  template: `
    <div class="bldr-container">
      <header class="bldr-header">
        <h1>bldr</h1>
        <nav class="bldr-header-user">
          <user-nav></user-nav>
        </nav>
      </header>
      <section class="bldr-main">
        <router-outlet></router-outlet>
      </section>
      <footer class="bldr-footer">
        <p>&copy; {{now}} Chef Software, Inc. All Rights Reserved.</p>
      </footer>
    </div>
  `,
})

@RouteConfig([
  { path: "/", name: "Home", component: HomeComponent },
  { path: "/dashboard", name: "Dashboard", component: DashboardComponent },
  { path: "/sign-in", name: "Sign In", component: SignInComponent },
])

export class AppComponent {
  now = new Date().getFullYear();
}
