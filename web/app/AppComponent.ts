import {AppStore} from "./AppStore";
import {Component, Inject} from "angular2/core";
import {DashboardComponent} from "./dashboard/DashboardComponent";
import {HomeComponent} from "./home/HomeComponent";
import {PackageComponent} from "./package/PackageComponent";
import {Router, RouteConfig, ROUTER_DIRECTIVES} from "angular2/router";
import {SignInComponent} from "./sign-in/SignInComponent";
import {UserNavComponent} from "./user-nav/UserNavComponent";
import {routeChange} from "./actions";

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
  { path: "/packages/:derivation/:id", name: "Package", component: PackageComponent },
])

export class AppComponent {
  constructor(private router: Router, private store: AppStore) {
    this.router.subscribe(value => this.store.dispatch(routeChange(value)));
    store.subscribe(state => console.log('new state received ', state.toObject())); 
  }

  get now() {
    return this.store.getState().currentYear;
  }
}
