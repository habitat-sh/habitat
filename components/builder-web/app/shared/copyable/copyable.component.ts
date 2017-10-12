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

import { Component, Input, ViewChild } from '@angular/core';
import { MdTooltip } from '@angular/material';

@Component({
  selector: 'hab-copyable',
  template: require('./copyable.component.html')
})
export class CopyableComponent {

  @Input() command: string = '';

  public copied: boolean = false;

  @ViewChild(MdTooltip)
  tooltip: MdTooltip;

  copy(text) {
    let el = document.createElement('input');

    Object.assign(el.style, {
      opacity: '0',
      position: 'fixed',
      left: '-200px'
    });

    document.body.appendChild(el);
    el.value = this.command;
    el.select();
    document.execCommand('copy');
    document.body.removeChild(el);

    this.copied = true;
    setTimeout(() => { this.tooltip.show(); }, 10);
    setTimeout(() => { this.copied = false; }, 2500);
  }

  get symbol() {
    return this.copied ? 'check' : 'copy';
  }

  get title() {
    return this.copied ? 'Copied!' : 'Copy to clipboard';
  }
}
