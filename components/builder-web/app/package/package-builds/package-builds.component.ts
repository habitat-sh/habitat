import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Subscription } from "rxjs/subscription";
import { AppStore } from "../../AppStore";
import { fetchBuilds } from "../../actions/index";

@Component({
    template: require("./package-builds.component.html")
})
export class PackageBuildsComponent implements OnDestroy {
    origin: string;
    name: string;

    private sub: Subscription;

    constructor(
        private route: ActivatedRoute,
        private store: AppStore,
        private router: Router) {

        this.sub = this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.name = params["name"];
            this.fetchBuilds();
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    get builds() {
        return this.store.getState().builds.visible;
    }

    onSelect(build) {
        this.router.navigate(["pkgs", this.origin, this.name, "builds", build.id]);
    }

    private fetchBuilds() {
        this.store.dispatch(fetchBuilds(this.origin, this.name, ""));
    }
}
