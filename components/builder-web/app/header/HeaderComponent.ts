// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import config from "../config";
import {UserNavComponent} from "./user-nav/UserNavComponent";

@Component({
    directives: [RouterLink, UserNavComponent],
    inputs: ["appName", "isUserNavOpen", "isSignedIn", "username", "avatarUrl",
        "signOut", "toggleUserNavMenu"],
    selector: "hab-header",
    template: `
      <div class="main-nav--container clearfix">
        <div class="main-nav--logo">
          <a [routerLink]="['PackagesForOrigin', { origin: 'core' }]"><h1>{{appName}}</h1></a>
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
          <div class="main-nav--cta">
            <a class="button" [routerLink]="['SignIn']">Sign In</a>
          </div>
          <ul class="main-nav--links">
              <li class="main-nav--link"><a class="search-packages" [routerLink]="['PackagesForOrigin', { origin: 'core' }]">Search Packages</a></li>
              <li class="main-nav--link"><a class="docs" href="{{config['docs_url']}}">Docs</a></li>
              <li class="main-nav--link"><a class="tutorials" href="{{config['tutorials_url']}}">Tutorials</a></li>
              <li class="main-nav--link"><a class="community" href="{{config['community_url']}}">Community</a></li>
              <user-nav [isOpen]="isUserNavOpen"
                      [isSignedIn]="isSignedIn"
                      [username]="username"
                      [avatarUrl]="avatarUrl"
                      [signOut]="signOut"
                      [toggleUserNavMenu]="toggleUserNavMenu"></user-nav>
          </ul>
        </nav>
      </div>`,
})

export class HeaderComponent {
    get config() { return config; }
}
