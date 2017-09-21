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
                        src="../assets/images/habitat-logo-by-chef-white.svg"
                        onerror="this.src='../assets/images/habitat-logo-by-chef-white.png'"
                        alt="Habitat by Chef" />
                </a>
            </div>
            <div class="footer--links">
                <ul class="no-bullet">
                    <li><h4>About Habitat</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}">Home</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/about">Why Habitat</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['docs_url']}}/overview">Get Started</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Build Service</h4></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['/pkgs', 'core']">Find Packages</a></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['/sign-in']">Sign In</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['docs_url']}}/get-habitat">Download CLI</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Resources</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['tutorials_url']}}">Tutorials</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['docs_url']}}">Docs</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['www_url']}}/blog">Blog</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Community</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['slack_url']}}">Habitat Slack</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['events_url']}}">Events</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['roadmap_url']}}">Roadmap</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['source_code_url']}}/projects/1">GitHub Tracker</a></li>
                </ul>
                <ul class="no-bullet">
                    <li><h4>Support</h4></li>
                    <li class="footer--sitemap--link"><a href="{{config['source_code_url']}}/issues">New Issues</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['feature_requests_url']}}">Feature Request</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['forums_url']}}">Forums</a></li>
                    <li class="footer--sitemap--link"><a href="{{config['status_url']}}" target="_blank">Status</a></li>
                </ul>
                <ul class="no-bullet social">
                    <li><h4>Get in Touch</h4></li>
                    <li class="footer--sitemap--link"><a class="github" href="{{config['source_code_url']}}">GitHub</a></li>
                    <li class="footer--sitemap--link"><a class="slack" href="{{config['slack_url']}}">Slack</a></li>
                    <li class="footer--sitemap--link"><a class="youtube" href="{{config['youtube_url']}}">YouTube</a></li>
                </ul>
            </div>
            <div class="footer--legal">
                <ul class="footer--copyright">
                  <li class="footer--sitemap--link">
                    &copy; {{currentYear}} <a href="http://chef.io">Chef Software, Inc</a>. All Rights Reserved. Patent Pending.
                    <span class="footer--version">{{config["version"]}}</span>
                  </li>
                  <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/licensing">Licensing</a></li>
                  <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/terms-and-conditions">Terms &amp; Conditions</a></li>
                  <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/trademark-policy">Trademark Policy</a></li>
                  <li class="footer--sitemap--link"><a href="{{config['www_url']}}/legal/privacy-policy">Privacy Policy</a></li>
                </ul>
            </div>
        </div>
    </footer>`,
})

export class FooterComponent {
    @Input() currentYear: number;

    get config() { return config; }
}
