import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {Router, RouterLink} from "angular2/router";

@Component({
  directives: [RouterLink],
  template: `
  <div class="bldr-dashboard">
    <h2>Packages</h2>
    <ul class="bldr-dashboard-plan-list">
      <li *ngIf="packages.length === 0">
        You have no packages. Here's how to create one: &hellip;
      </li>
      <li *ngFor="#package of packages">
        <a [routerLink]="['Package', { id: package.name, derivation: package.derivation }]">
          {{index}}
          {{username}}/{{package.name}}
        </a>
      </li>
    </ul>
  </div>
  `
})

export class DashboardComponent {
  constructor(private router: Router, private store: AppStore) {}

  get packages() {
    return this.store.getState().packages;
  }

  get username() {
    return this.store.getState().username;
  }

  ngOnInit() {
    if (!this.store.getState().isSignedIn) {
      this.router.navigate(["Home"])
    }
  }
}
