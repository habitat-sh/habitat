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

import {AppStore} from "../AppStore";
import {Component} from "@angular/core";
import {icon, requireSignIn} from "../util";

@Component({
    template: `
    <div class="hab-organizations">
      <div class="page-title">
          <h2>Organizations</h2>
          <a *ngIf="orgs.size > 0" class="button create" href="#"
             [routerLink]="['/orgs', 'create']">
              Add Organization
          </a>
      </div>
      <div class="page-body">
          <div *ngIf="orgs.size === 0">
              <div class="hero">
                  <h3>You don't currently have any organizations, let's add one now.</h3>
                  <p>
                      <a class="button cta" [routerLink]="['/orgs', 'create']">
                          Add Organization
                      </a>
                  </p>
              </div>
              <div class="info">
                  <hr>
                  <ul>
                      <li>
                          Invite users to manage a project
                      </li>
                      <li>
                          List public projects under your organization profile
                      </li>
                      <li>
                          Set admin permissions for members to edit organization
                          settings
                      </li>
                  </ul>
              </div>
          </div>
          <div *ngIf="orgs.size > 0">
              <ul>
                  <li *ngFor="let org of orgs">
                      <a href="#" class="hab-item-list">
                          <div class="item-title">
                              <gravatar size=32 email="{{org.email}}"></gravatar>
                              <h3>
                                  {{org.name}}
                              </h3>
                          </div>
                          <div class="item-info">
                              <span class="count">
                                  <img src='{{icon("organization")}}'>
                                  {{org.members.size}}
                              </span>
                          </div>
                      </a>
                  </li>
              </ul>
          </div>
      </div>
    </div>`
})

export class OrganizationsPageComponent {
    constructor(private store: AppStore) {
        requireSignIn(this);
    }

    get orgs() { return this.store.getState().orgs.all; }

    private icon(x) { return icon(x); }
}
