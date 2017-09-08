import { Component, OnDestroy, OnInit } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Subscription } from "rxjs/subscription";
import { AppStore } from "../../AppStore";
import { packageString, targetToPlatform, releaseToDate } from "../../util";
import { fetchPackageVersions, fetchLatestPackage, filterPackagesBy, submitJob } from "../../actions/index";

@Component({
    template: require("./package-versions.component.html")
})
export class PackageVersionsComponent implements OnDestroy {
    origin: string;
    name: string;
    selected: object = null;

    private sub: Subscription;

    constructor(
        private route: ActivatedRoute,
        private store: AppStore,
        private router: Router) {

        this.sub = this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.name = params["name"];
            this.fetchVersions();
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    get ident() {
        return {
            origin: this.origin,
            name: this.name
        };
    }

    toggle(item) {
        if (this.selected === item) {
            this.selected = null;
        }
        else {
            this.selected = item;

            this.fetchPackages({
                origin: item.origin,
                name: item.name,
                version: item.version
            });
        }
    }

    get platforms() {
        let targets = [];

        this.versions.forEach((v) => {
            v.platforms.forEach((p) => {
                if (targets.indexOf(p) === -1) {
                    targets.push(p);
                }
            });
        });

        return targets.sort();
    }

    fetchPackages(params) {
        this.store.dispatch(filterPackagesBy(params, null, false));
    }

    packageString(pkg) {
        return packageString(pkg);
    }

    releaseToDate(release) {
        return releaseToDate(release);
    }

    osIconFor(pkg) {
        return pkg.target || "linux";
    }

    toggleFor(version) {
        return this.selected === version ? "chevron-up" : "chevron-down";
    }

    navigateTo(pkg) {
        let params = ["pkgs", pkg.origin, pkg.name, pkg.version, pkg.release];
        this.router.navigate(params);
    }

    get versions() {
        return this.store.getState().packages.versions || [];
    }

    packagesFor(version) {
        let packages = this.store.getState().packages.visible;

        if (packages && packages.size > 0 && packages.get(0).version === version.version) {
            return packages;
        }

        return [];
    }

    private fetchVersions() {
        this.store.dispatch(fetchPackageVersions(this.origin, this.name));
    }
}
