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
import { List } from 'immutable';
import { packageString } from '../../util';

@Component({
  selector: 'hab-search-results',
  template: require('./results.component.html')
})
export class SearchResultsComponent {
  @Input() errorMessage: string;
  @Input() noPackages: boolean;
  @Input() packages: List<Object>;

  routeFor(pkg) {
    return ['/pkgs', pkg.origin, pkg.name, 'latest'];
  }

  packageString(pkg) {
    return packageString(pkg);
  }
}
