import { Component, Input, OnChanges, SimpleChanges } from "@angular/core";
import { packageString, targetToPlatform, releaseToDate } from "../../util";
import { AppStore } from "../../AppStore";
import { fetchLatestInChannel, fetchPackageVersions, submitJob } from "../../actions/index";

@Component({
    selector: "hab-package-sidebar",
    template: require("./package-sidebar.component.html")
})
export class PackageSidebarComponent implements OnChanges {
    @Input() origin: string;
    @Input() name: string;
    @Input() building: boolean = false;
    @Input() buildable: boolean = false;

    constructor(private store: AppStore) {}

    ngOnChanges(changes: SimpleChanges) {
        let fetch = false;

        if (changes["origin"]) {
            this.origin = changes["origin"].currentValue;
            fetch = true;
        }

        if (changes["name"]) {
            this.name = changes["name"].currentValue;
            fetch = true;
        }

        if (fetch) {
            this.fetchLatestStable();
            this.fetchPackageVersions();
        }
    }

    get latestStable() {
        return this.store.getState().packages.latestInChannel.stable;
    }

    build() {
        if (this.buildable) {
            let token = this.store.getState().gitHub.authToken;
            this.store.dispatch(submitJob(this.origin, this.name, token));
        }
    }

    get buildButtonLabel() {
        return this.building ? "Build pending" : "Build latest version";
    }

    get exportCommand() {
        return `hab pkg export docker ${this.origin}/${this.name}`;
    }

    get runCommand() {
        return `hab start ${this.origin}/${this.name}`;
    }

    get platforms() {
        let targets = [];
        let versions = this.store.getState().packages.versions || [];

        versions.forEach((v) => {
            v.platforms.forEach((p) => {
                if (targets.indexOf(p) === -1) {
                    targets.push(p);
                }
            });
        });

        return targets.sort();
    }

    private fetchLatestStable() {
      this.store.dispatch(fetchLatestInChannel(this.origin, this.name, "stable"));
    }

    private fetchPackageVersions() {
        this.store.dispatch(fetchPackageVersions(this.origin, this.name));
    }
}
