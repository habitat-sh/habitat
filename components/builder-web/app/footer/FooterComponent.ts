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

import {Component, Input} from "@angular/core";
import config from "../config";

@Component({
    selector: "hab-footer",
    template: `
    <footer class="hab-footer">
        <div class="footer--sitemap">
            <div class="footer--logos">
                <a href="{{config['www_url']}}" title="Habitat Home">
                    <img class="footer--logo habitat"
                        src="../assets/images/logo-chef-habitat-lockup.svg"
                        onerror="this.src='../assets/images/habitat-logo-by-chef-white.png'"
                        alt="Habitat by Chef" />
                </a>
            </div>
            <div class="footer--links">
                <ul class="no-bullet">
                    <li><h4>Habitat</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}">Home</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['docs_url']}}">Docs</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['tutorials_url']}}">Tutorials</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['source_code_url']}}">Source</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Habitat Web</h4></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['/sign-in']">Sign In</a></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['/pkgs', 'core']">Search Packages</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>More</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/about">About</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['community_url']}}">Community</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Legal</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/licensing">Licensing</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/terms-and-conditions">Terms &amp; Conditions</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/trademark-policy">Trademark Policy</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/privacy-policy">Privacy Policy</a></li>
                </ul>
            </div>
        </div>
        <div class="footer--legal">
            <p class="footer--copyright">
                &copy; {{currentYear}} <a href="http://chef.io">Chef Software, Inc</a>. All Rights Reserved. Patent Pending.
                <span class="footer--version">{{config["version"]}}</span>
            </p>

        </div>
    </footer>`,
})

export class FooterComponent {
    @Input() currentYear: number;

    get config() { return config; }
}
