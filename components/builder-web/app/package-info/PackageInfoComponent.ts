// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import {Component, Input} from "@angular/core";
import {PackageListComponent} from "../package-page/PackageListComponent";

@Component({
    selector: "hab-package-info",
    directives: [PackageListComponent],
    template: `
    <div class="has-sidebar">
      <div class="page-body--main">
        <div class="hab-package-info">
          <dl>
            <dt>Version</dt>
            <dd>{{package.ident.version}}</dd>
            <dt>Release</dt>
            <dd>{{package.ident.release}}</dd>
            <dt>Checksum</dt>
            <dd>{{package.checksum}}</dd>
            <dt *ngIf="package.exposes.length > 0">Exposed Ports</dt>
            <dd *ngIf="package.exposes.length > 0">
              <span *ngFor="let port of package.exposes">{{port}} </span>
            </dd>
          </dl>
        </div>
        <div class="hab-package-manifest">
          <h3>Manifest</h3>
          <div class="manifest" [innerHTML]="package.manifest"></div>
        </div>
        <div class="hab-package-config" *ngIf="package.config">
          <h3>Configuration</h3>
          <pre>{{package.config}}</pre>
        </div>
      </div>
    </div>
    <div class="page-body--sidebar">
      <div class="hab-package-deps-build">
        <h3>Dependencies</h3>
        <package-list [currentPackage]="package"
                [packages]="package.deps"></package-list>
      </div>
      <div class="hab-package-deps-runtime">
        <h3>Transitive Dependencies</h3>
        <package-list [currentPackage]="package"
                                [packages]="package.tdeps"></package-list>
      </div>
    </div>
    `
})

export class PackageInfoComponent {
    @Input() package: Object;
}
