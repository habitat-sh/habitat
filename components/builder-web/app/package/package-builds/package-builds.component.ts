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
import { Subscription } from 'rxjs/subscription';
import { AppStore } from '../../app.store';

@Component({
  template: require('./package-builds.component.html')
})
export class PackageBuildsComponent implements OnDestroy {
  origin: string;
  name: string;

  private sub: Subscription;

  constructor(
    private route: ActivatedRoute,
    private store: AppStore,
    private router: Router) {

    this.sub = this.route.parent.params.subscribe((params) => {
      this.origin = params['origin'];
      this.name = params['name'];
    });
  }

  ngOnDestroy() {
    if (this.sub) {
      this.sub.unsubscribe();
    }
  }

  get builds() {
    return this.store.getState().builds.visible;
  }

  onSelect(build) {
    this.router.navigate(['pkgs', this.origin, this.name, 'builds', build.id]);
  }
}
