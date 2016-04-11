// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
