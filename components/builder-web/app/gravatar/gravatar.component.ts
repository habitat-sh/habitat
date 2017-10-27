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
const md5 = require('blueimp-md5');

@Component({
  selector: 'hab-gravatar',
  template: `
    <img class="hab-gravatar"
        width="{{size || DEFAULT_SIZE}}" height="{{size || DEFAULT_SIZE}}"
        src='{{gravatarUrl(defaultStyle, email, size)}}'>`
})

export class GravatarComponent {
  @Input() defaultStyle;
  @Input() email;
  @Input() size;

  DEFAULT_SIZE = 80;

  gravatarUrl(defaultStyle: string = 'retro',
    email: string, size: number = this.DEFAULT_SIZE) {

    defaultStyle = encodeURIComponent(defaultStyle || 'retro');
    return `https://secure.gravatar.com/avatar/
            ${md5(email.toLowerCase().trim())}?
            d=${defaultStyle}&
            s=${size}`.replace(/\s/g, '');
  }
}
