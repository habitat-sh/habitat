import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/subscription";
import { AppStore } from "../../AppStore";
import { fetchPackage } from "../../actions/index";

@Component({
    template: require("./package-release.component.html")
})
export class PackageReleaseComponent implements OnDestroy {
    origin: string;
    name: string;
    version: string;
    release: string;

    private sub: Subscription;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        this.sub = this.route.params.subscribe((params) => {
            let parentParams = this.route.parent.params["value"];
            this.origin = parentParams["origin"];
            this.name = parentParams["name"];
            this.version = params["version"];
            this.release = params["release"];
            this.fetchRelease();
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
            name: this.name,
            version: this.version,
            release: this.release
        };
    }

    get package() {
        return this.store.getState().packages.current;
    }

    private fetchRelease() {
        this.store.dispatch(fetchPackage({ ident: this.ident }));
    }
}
