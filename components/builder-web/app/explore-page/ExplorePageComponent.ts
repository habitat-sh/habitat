// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {fetchExplore} from "../actions/index";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-explore">
      <div class="page-title">
          <h2>Explore</h2>
      </div>
      <div class="page-body">
          <ul class="hab-packages-plan-list">
              <li class="hab-packages-package" *ngFor="#item of store.getState().packages.explore">
                  <a [routerLink]="['PackagesForName', { name: item.name }]">
                      <span class="title">{{item.name}}</span>
                      <div class="info">
                          <span class="stars">{{item.starCount}} ★</span>
                      </div>
                  </a>
              </li>
          </ul>
      </div>
    </div>`,
})
export class ExplorePageComponent implements OnInit {
    constructor(private store: AppStore) { }
    ngOnInit() {
        this.store.dispatch(fetchExplore());
    }
}
