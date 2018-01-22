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

@Component({
  selector: 'hab-icon',
  template: `<mat-icon [svgIcon]="id" [matTooltip]="tooltip" matTooltipPosition="above"></mat-icon>`
})
export class IconComponent {
  @Input() symbol: string;
  @Input() title: string = '';

  get id() {
    if (this.symbol) {
      return `icon-${this.symbol}`;
    }
  }

  get tooltip() {
    let tip;

    if (this.title.trim() !== '') {
      tip = this.title;
    }

    return tip;
  }
}
