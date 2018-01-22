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

import { Component, OnInit, OnDestroy } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs/Subscription';
import { List } from 'immutable';
import { PackageBuildsComponent } from '../package-builds/package-builds.component';
import { PackageLatestComponent } from '../package-latest/package-latest.component';
import { PackageReleaseComponent } from '../package-release/package-release.component';
import { PackageVersionsComponent } from '../package-versions/package-versions.component';
import { AppStore } from '../../app.store';
import { fetchBuilds, fetchIntegrations, fetchOrigin, fetchProject } from '../../actions/index';

@Component({
  template: require('./package.component.html')
})
export class PackageComponent implements OnInit, OnDestroy {
  origin: string;
  name: string;
  showSidebar: boolean = false;
  showActiveBuild: boolean = false;

  private sub: Subscription;
  private poll: number;

  constructor(private route: ActivatedRoute, private store: AppStore) {
    this.sub = this.route.params.subscribe(params => {
      this.origin = params['origin'];
      this.name = params['name'];
      this.store.dispatch(fetchOrigin(this.origin));
    });
  }

  ngOnInit() {

    // When a build is active, check on it periodically so we can
    // indicate when it completes.
    this.poll = window.setInterval(() => {
      if (this.building) {
        this.fetchBuilds();
      }
    }, 10000);
  }

  ngOnDestroy() {
    window.clearInterval(this.poll);

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

  get isOriginMember() {
    return !!this.store.getState().origins.mine.find((o) => {
      return o.name === this.origin;
    });
  }

  get buildable(): boolean {
    let hasProject = this.store.getState().projects.ui.current.exists;

    if (this.isOriginMember && hasProject) {
      return true;
    }

    return false;
  }

  get activeBuilds(): List<any> {
    const activeStates = ['Dispatched', 'Pending', 'Processing'];

    return this.store.getState().builds.visible.filter((b) => {
      return activeStates.indexOf(b.state.toString()) !== -1;
    });
  }

  get activeBuild() {
    let active = this.activeBuilds.last();
    return active;
  }

  get building(): boolean {
    return this.activeBuilds.size > 0;
  }

  get token() {
    return this.store.getState().session.token;
  }

  onRouteActivate(routedComponent) {
    this.showSidebar = false;
    this.showActiveBuild = false;

    [
      PackageBuildsComponent,
      PackageLatestComponent,
      PackageReleaseComponent,
      PackageVersionsComponent
    ].forEach((c) => {
      if (routedComponent instanceof c) {
        this.showSidebar = true;
        this.showActiveBuild = true;
      }
    });

    this.fetchProject();
    this.fetchBuilds();
  }

  private fetchProject() {
    if (this.token && this.origin && this.name && this.isOriginMember) {
      this.store.dispatch(fetchProject(this.origin, this.name, this.token, false));
      this.store.dispatch(fetchIntegrations(this.origin, this.token));
    }
  }

  private fetchBuilds() {
    if (this.token) {
      this.store.dispatch(fetchBuilds(this.origin, this.name, this.token));
    }
  }
}
