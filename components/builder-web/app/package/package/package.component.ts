import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";
import { PackageBuildsComponent } from "../package-builds/package-builds.component";
import { PackageLatestComponent } from "../package-latest/package-latest.component";
import { PackageReleaseComponent } from "../package-release/package-release.component";
import { PackageVersionsComponent } from "../package-versions/package-versions.component";

@Component({
    template: require("./package.component.html")
})
export class PackageComponent implements OnDestroy {
    origin: string;
    name: string;
    sidebar: boolean = false;

    private sub: Subscription;

    constructor(private route: ActivatedRoute) {
        this.sub = this.route.params.subscribe(params => {
            this.origin = params["origin"];
            this.name = params["name"];
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    enabled(feature) {
        return [
            "latest",
            "versions",
            "builds"
        ].indexOf(feature) >= 0;
    }

    get ident() {
        return {
            origin: this.origin,
            name: this.name
        };
    }

    onRouteActivate(routedComponent) {
        this.sidebar = false;

        [
            PackageBuildsComponent,
            PackageLatestComponent,
            PackageReleaseComponent,
            PackageVersionsComponent
        ].forEach((c) => {
            if (routedComponent instanceof c) {
                this.sidebar = true;
            }
        });
    }
}
