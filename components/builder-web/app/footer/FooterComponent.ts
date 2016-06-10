// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, Input} from "angular2/core";
import {RouterLink} from "angular2/router";
import config from "../config";

@Component({
    directives: [RouterLink],
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
                </ul>
                <ul class="no-bullet">
                    <li><h4>Habitat Web</h4></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['SignIn']">Sign In</a></li>
                    <li class="footer--sitemap--link"><a [routerLink]="['PackagesForOrigin', { origin: 'core' }]">Search Packages</a></li>
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
            <p class="footer--copyright">&copy; {{currentYear}} <a href="http://chef.io">Chef Software, Inc</a>. All Rights Reserved. Patent Pending.</p>
        </div>
    </footer>`,
})

export class FooterComponent {
    @Input() currentYear: number;

    get config() { return config; }
}
