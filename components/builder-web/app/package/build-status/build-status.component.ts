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
import { iconForBuildState } from '../../util';
import { AppStore } from '../../app.store';

@Component({
  selector: 'hab-build-status',
  template: `
    <hab-icon
      [symbol]="iconFor(state)"
      class="status {{ state | lowercase }}"
      *ngIf="state"></hab-icon>
  `
})
export class BuildStatusComponent {
  @Input() origin;
  @Input() name;
  @Input() version;
  @Input() release;

  constructor(private store: AppStore) { }

  iconFor(state) {
    return iconForBuildState(state);
  }

  get id() {
    return this.first() ? this.first().id : null;
  }

  get state() {
    return this.first() ? this.first().state : null;
  }

  private first() {
    return this.store.getState().builds.visible.find((b) => {
      if (b.origin === this.origin && b.name === this.name) {
        return this.version ? b.version === this.version : true;
      }
    });
  }
}
