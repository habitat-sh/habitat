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
import { targetToPlatform, releaseToDate } from '../../util';

@Component({
  selector: 'hab-package-detail',
  template: require('./package-detail.component.html')
})
export class PackageDetailComponent {
  @Input() package: object;

  releaseToDate(release) {
    return releaseToDate(release);
  }

  get fullName() {
    let ident = this.package['ident'];
    let name = '';

    if (ident.origin && ident.name) {
      name = `${ident.origin}/${ident.name}`;
    }

    return name;
  }
}
