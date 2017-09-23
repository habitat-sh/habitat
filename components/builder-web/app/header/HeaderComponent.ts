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
import { Router, RouterLink } from "@angular/router";
import config from "../config";

@Component({
  selector: "hab-header",
  template: `
      <div class="main-nav--container clearfix">
        <div class="main-nav--logo">
          <a [routerLink]="['/']"><h1>{{appName}}</h1></a>
        </div>
        <nav class="main-nav--links-wrap">
          <div class="main-nav--toggle">
            <svg width="44px" height="44px" viewBox="0 0 44 44" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
              <g id="Habitat-Web-Home" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                <g id="Habitat-Web-Home-Mobile" transform="translate(-260.000000, -10.000000)" fill="#5C6670">
                  <g id="Nav-Mobile" transform="translate(15.000000, 10.000000)">
                    <g id="Nav" transform="translate(245.000000, 0.000000)">
                      <path d="M29,28 C29,27.44775 28.6471446,27 28.2119365,27 L14.7880635,27 C14.3528554,27 14,27.44775 14,28 C14,28.55225 14.3528554,29 14.7880635,29 L28.2119365,29 C28.6471446,29 29,28.55225 29,28 M26,24 C26,23.44775 25.6511152,23 25.2208045,23 L14.7791955,23 C14.3488848,23 14,23.44775 14,24 C14,24.55225 14.3488848,25 14.7791955,25 L25.2208045,25 C25.6511152,25 26,24.55225 26,24 M24,20 C24,19.44775 23.6373833,19 23.1901359,19 L14.8098641,19 C14.3626167,19 14,19.44775 14,20 C14,20.55225 14.3626167,21 14.8098641,21 L23.1901359,21 C23.6373833,21 24,20.55225 24,20 M29.2,17 L14.8,17 C14.3582,17 14,16.55225 14,16 C14,15.44775 14.3582,15 14.8,15 L29.2,15 C29.6418,15 30,15.44775 30,16 C30,16.55225 29.6418,17 29.2,17" id="align-left"></path>
                    </g>
                  </g>
                </g>
              </g>
            </svg>
          </div>
          <hab-user-nav [isOpen]="isUserNavOpen"
                    [isSignedIn]="isSignedIn"
                    [username]="username"
                    [avatarUrl]="avatarUrl"
                    [signOut]="signOut"
                    [toggleUserNavMenu]="toggleUserNavMenu"></hab-user-nav>
          <ul class="main-nav--links">
            <li class="main-nav--link">
              <a class="about"
                [routerLink]="['/explore']"
                [class.is-current-page]="area === 'explore'">Explore</a>
            </li>
            <li class="main-nav--link">
              <a class="tutorials" href="{{config['tutorials_url']}}">Tutorials</a>
            </li>
            <li class="main-nav--link">
              <a class="docs" href="{{config['docs_url']}}">Docs</a>
            </li>
            <li class="main-nav--link">
              <a class="community" href="{{config['community_url']}}">Community</a>
            </li>
            <li class="main-nav--link">
              <a class="blog" href="{{config['www_url']}}/blog">Blog</a>
            </li>
            <li class="main-nav--link">
              <a class="depot"
                [routerLink]="['/pkgs']"
                [class.is-current-page]="area === 'depot'">Builder</a>
            </li>
            <li class="main-nav--link cta-link" *ngIf="!isSignedIn">
              <a [routerLink]="['/sign-in']">Sign In</a>
            </li>
          </ul>
        </nav>
      </div>`
})

export class HeaderComponent {
  @Input() appName;
  @Input() isUserNavOpen;
  @Input() isSignedIn;
  @Input() username;
  @Input() avatarUrl;
  @Input() signOut;
  @Input() toggleUserNavMenu;

  constructor(private router: Router) { }

  get config() {
    return config;
  }

  get area() {
    if (this.router.url === "/explore") {
      return "explore";
    }
    else {
      return "depot";
    }
  }
}
