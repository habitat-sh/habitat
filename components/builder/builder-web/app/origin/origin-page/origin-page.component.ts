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
import { AppStore } from '../../app.store';
import { Origin } from '../../records/Origin';
import { fetchOrigin, fetchMyOrigins, getUniquePackages, fetchIntegrations, fetchProjects } from '../../actions';

@Component({
  template: require('./origin-page.component.html')
})
export class OriginPageComponent implements OnInit, OnDestroy {
  loadPackages: Function;
  perPage: number = 50;
  sub: Subscription;
  originName: string;

  constructor(private route: ActivatedRoute, private store: AppStore) {
    this.sub = this.route.params.subscribe(params => {
      this.originName = params['origin'];
    });
  }

  ngOnInit() {
    this.store.dispatch(fetchOrigin(this.origin.name));
    this.fetchIntegrations();
    this.fetchMyOrigins();
    this.fetchPackages();
    this.fetchProjects();
    this.loadPackages = this.fetchPackages.bind(this);
  }

  ngOnDestroy() {
    this.sub.unsubscribe();
  }

  get origin() {
    const current = this.store.getState().origins.current;

    if (current.name === this.originName) {
      return current;
    }

    return Origin({ name: this.originName });
  }

  get features() {
    return this.store.getState().users.current.flags;
  }

  get token() {
    return this.store.getState().session.token;
  }

  get ui() {
    return this.store.getState().origins.ui.current;
  }

  get totalCount() {
    return this.store.getState().packages.totalCount;
  }

  get myOrigins() {
    return this.store.getState().origins.mine;
  }

  get memberOfOrigin() {
    return !!this.myOrigins.find(org => {
      return org['name'] === this.origin.name;
    });
  }

  private fetchIntegrations() {
    if (this.token) {
      this.store.dispatch(fetchIntegrations(this.origin.name, this.token));
    }
  }

  private fetchMyOrigins() {
    if (this.token) {
      this.store.dispatch(fetchMyOrigins(this.token));
    }
  }

  private fetchProjects() {
    if (this.token) {
      this.store.dispatch(fetchProjects(this.origin.name, this.token));
    }
  }

  private fetchPackages() {
    this.store.dispatch(getUniquePackages(this.origin.name, 0, this.token));
  }
}
