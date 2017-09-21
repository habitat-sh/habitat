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

import { Component, Input } from "@angular/core";

@Component({
    selector: "hab-package-form-progress",
    template: `
    <ol class="hab-package-form-progress small">
    <li class="hab-package-form-progress-step" *ngFor="let step of steps">
      {{step.disabled}}
      <a [ngClass]="{current: step.current}" [routerLink]="step.disabled ? null : step.target">{{step.name}}</a>
    </li>
    </ol>`
})

export class FormProgressComponent {
    @Input() steps;
}
