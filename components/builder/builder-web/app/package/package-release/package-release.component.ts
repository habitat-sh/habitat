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
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs/Subscription';
import { AppStore } from '../../app.store';
import { fetchPackage } from '../../actions/index';

@Component({
  template: require('./package-release.component.html')
})
export class PackageReleaseComponent implements OnDestroy {
  origin: string;
  name: string;
  version: string;
  release: string;

  private sub: Subscription;

  constructor(
    private route: ActivatedRoute,
    private store: AppStore,
    private title: Title
  ) {
    this.sub = this.route.params.subscribe((params) => {
      let parentParams = this.route.parent.params['value'];
      this.origin = parentParams['origin'];
      this.name = parentParams['name'];
      this.version = params['version'];
      this.release = params['release'];
      this.title.setTitle(`Packages › ${this.origin}/${this.name}/${this.version}/${this.release} | Habitat`);
      this.fetchRelease();
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
      name: this.name,
      version: this.version,
      release: this.release
    };
  }

  get package() {
    return this.store.getState().packages.current;
  }

  private fetchRelease() {
    this.store.dispatch(fetchPackage({ ident: this.ident }));
  }
}
