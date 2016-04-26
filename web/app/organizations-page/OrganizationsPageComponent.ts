// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {GravatarComponent} from "../GravatarComponent";
import {RouterLink} from "angular2/router";
import {icon, requireSignIn} from "../util";

@Component({
    directives: [GravatarComponent, RouterLink],
    template: `
    <div class="hab-organizations">
      <div class="page-title">
          <h2>Organizations</h2>
          <a *ngIf="orgs.size > 0" class="button create" href="#"
             [routerLink]='["OrganizationCreate"]'>
              Add Organization
          </a>
      </div>
      <div class="page-body">
          <div *ngIf="orgs.size === 0">
              <div class="hero">
                  <h3>You don't currently have any organizations, let's add one now.</h3>
                  <p>
                      <a class="button cta" [routerLink]='["OrganizationCreate"]'>
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
                  <li *ngFor="#org of orgs">
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
