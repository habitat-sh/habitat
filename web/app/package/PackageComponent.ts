import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {Router, RouterLink} from "angular2/router";

@Component({
  directives: [RouterLink],
  template: `
    <div *ngIf="!package" class="bldr-package">
      <h2>Not Found</h2>
      <p>{{currentPackage}} does not exist.</p>
      <p>Here's how you would make it: &hellip;</p>
    </div>
    <div *ngIf="package" class="bldr-package">
      <h2>
        <a [routerLink]="['Dashboard']">{{package.derivation}}</a>
        /
        {{package.name}}
      </h2>
      <div class="bldr-package-info">
        <dl>
          <dt>Maintainer</dt>
          <dd>{{package.maintainer}}</dd>
          <dt>License</dt>
          <dd>{{package.license}}</dd>
          <dt>Source URL</dt>
          <dd><a href="{{package.source}}">{{package.source}}</a></dd>
        </dl>
      </div>
      <div class="bldr-package-version-info">
        <h3>Current Release</h3>
        <dl>
          <dt>Version</dt>
          <dd>{{package.version}}</dd>
          <dt>Release</dt>
          <dd>{{package.release}}</dd>
          <dt>SHA</dt>
          <dd>{{package.sha}}</dd>
        </dl>
      </div>
      <div class="bldr-package-deps">
        <h3>Dependencies</h3>
        <div class="bldr-package-deps-build">
          <h4>Build Dependencies</h4>
          <ul>
            <li *ngIf="package.buildDependencies.length === 0">None</li>
            <li *ngFor="#dep of package.buildDependencies">
              <a [routerLink]="['Package', { id: dep.name, derivation: dep.derivation }]">
                {{dep.identifier}}
              </a>
            </li>
          </ul>
        </div>
        <div class="bldr-package-deps-runtime">
          <h4>Runtime Dependencies</h4>
          <ul>
            <li *ngIf="package.dependencies.length === 0">None</li>
            <li *ngFor="#dep of package.dependencies">
              <a [routerLink]="['Package', { id: dep.name, derivation: dep.derivation }]">
                {{dep.identifier}}
              </a>
            </li>
          </ul>
        </div>
      </div>
    </div>
  `,
})

export class PackageComponent {
  constructor (private router: Router, private store: AppStore) {}

  get currentPackage() {
    let parts = window.location.pathname.split("/")
    return `${parts[2]}/${parts[3]}`;
  }

  get package () {
    return this.store.getState().packages.filter((pkg, index) => {
      return pkg.identifier === this.currentPackage;
    })[0];
  }
}
