// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

import { Component, OnDestroy } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { Subscription } from 'rxjs/Subscription';
import { AppStore } from '../../app.store';
import { packageString, releaseToDate } from '../../util';
import { fetchPackageVersions, filterPackagesBy } from '../../actions/index';

@Component({
  template: require('./package-versions.component.html')
})
export class PackageVersionsComponent implements OnDestroy {
  origin: string;
  name: string;
  selected: object = null;

  private sub: Subscription;

  constructor(
    private route: ActivatedRoute,
    private store: AppStore,
    private router: Router) {

    this.sub = this.route.parent.params.subscribe((params) => {
      this.origin = params['origin'];
      this.name = params['name'];
      this.fetchVersions();
    });
  }

  ngOnDestroy() {
    if (this.sub) {
      this.sub.unsubscribe();
    }
  }

  get ident() {
    return {
      origin: this.origin,
      name: this.name
    };
  }

  toggle(item) {
    if (this.selected === item) {
      this.selected = null;
    }
    else {
      this.selected = item;

      this.fetchPackages({
        origin: item.origin,
        name: item.name,
        version: item.version
      });
    }
  }

  get platforms() {
    let targets = [];

    this.versions.forEach((v) => {
      v.platforms.forEach((p) => {
        if (targets.indexOf(p) === -1) {
          targets.push(p);
        }
      });
    });

    return targets.sort();
  }

  fetchPackages(params) {
    this.store.dispatch(filterPackagesBy(params, null, false));
  }

  promotable(pkg) {
    return this.memberOfOrigin && pkg.channels.indexOf('stable') === -1;
  }

  get memberOfOrigin() {
    return !!this.store.getState().origins.mine.find(origin => origin['name'] === this.origin);
  }

  packageString(pkg) {
    return packageString(pkg);
  }

  releaseToDate(release) {
    return releaseToDate(release);
  }

  osIconFor(pkg) {
    return pkg.target || 'linux';
  }

  toggleFor(version) {
    return this.selected === version ? 'chevron-up' : 'chevron-down';
  }

  navigateTo(pkg) {
    let params = ['pkgs', pkg.origin, pkg.name, pkg.version, pkg.release];
    this.router.navigate(params);
  }

  get versions() {
    return this.store.getState().packages.versions || [];
  }

  packagesFor(version) {
    let packages = this.store.getState().packages.visible;

    if (packages && packages.size > 0 && packages.get(0).version === version.version) {
      return packages;
    }

    return [];
  }

  private fetchVersions() {
    this.store.dispatch(fetchPackageVersions(this.origin, this.name));
  }
}
