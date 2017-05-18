import { Component, OnInit } from "@angular/core";
import * as cookies from "js-cookie";
import { AppStore } from "../AppStore";
import { Router } from "@angular/router";
import { fetchMyOrigins, fetchDashboardRecent } from "../actions";
import { releaseToDate } from "../util";
import config from "../config";

@Component({
    template: require("./dashboard.component.html")
})
export class DashboardComponent implements OnInit {

    // API suport for top deps, favoriting and the blog aren't
    // quite ready yet, so we're hiding these sections for now.
    _hiddenSections = ["upper", "blog"];

    constructor(private store: AppStore, private router: Router) {}

    ngOnInit() {
        let selected = false;

        this.store.subscribe((s) => {
            let origin = s.origins.mine.first();

            if (!selected && origin) {
                selected = true;
                this.selectOrigin(origin.name);
            }
        });
    }

    get config() {
        return config;
    }

    get loading() {
        return this.store.getState().origins.ui.mine.loading;
    }

    get myOrigins() {
        return this.store.getState().origins.mine;
    }

    get myPackages() {
        return this.store.getState().packages.dashboard.recent;
    }

    get noOrigins() {
        return !this.store.getState().origins.ui.mine.loading &&
            this.store.getState().origins.mine.size === 0;
    }

    get selectedOrigin() {
        return this.store.getState().packages.dashboard.origin;
    }

    releaseToDate(release) {
        return releaseToDate(release);
    }

    showSection(section) {
        return !this._hiddenSections.includes(section);
    }

    navigateToPackage(pkg) {
        this.router.navigate(["pkgs", pkg.origin, pkg.name]);
    }

    selectOrigin(name: string) {
        this.store.dispatch(fetchDashboardRecent(name));
    }
}
