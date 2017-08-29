import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";
import { AppStore } from "../../AppStore";
import { fetchLatestPackage } from "../../actions/index";

@Component({
    template: require("./package-latest.component.html")
})
export class PackageLatestComponent implements OnDestroy {
    origin: string;
    name: string;

    private sub: Subscription;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.name = params["name"];
            this.fetchLatest();
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

    get latest() {
        return this.store.getState().packages.latest;
    }

    private fetchLatest () {
        this.store.dispatch(fetchLatestPackage(this.origin, this.name));
    }
}
