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

import { Component, Input } from '@angular/core';
import { AppStore } from '../../app.store';
import { releaseToDate } from '../../util';

@Component({
  selector: 'hab-package-detail',
  template: require('./package-detail.component.html')
})
export class PackageDetailComponent {
  @Input() package: any;

  constructor(private store: AppStore) {}

  get channels() {
    return this.store.getState().packages.currentChannels;
  }

  get fullName() {
    const ident = this.package['ident'];
    let props = [];

    ['origin', 'name', 'version', 'release'].forEach(prop => {
      if (ident[prop]) {
        props.push(ident[prop]);
      }
    });

    return props.join('/');
  }

  get memberOfOrigin() {
    return !!this.store.getState().origins.mine.find(
      origin => origin['name'] === this.package.ident.origin
    );
  }

  promotable(pkg) {
    return this.memberOfOrigin &&
      this.channels.length > 0 &&
      this.channels.indexOf('stable') === -1;
  }

  releaseToDate(release) {
    return releaseToDate(release);
  }
}
