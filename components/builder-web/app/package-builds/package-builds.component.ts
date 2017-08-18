import { Component, OnInit, OnDestroy } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { Subscription } from "rxjs";
import { AppStore } from "../AppStore";
import { clearBuilds, fetchBuilds } from "../actions/index";

@Component({
    template: require("./package-builds.component.html")
})
export class PackageBuildsComponent implements OnInit, OnDestroy {
    public name: string;
    public origin: string;

    private sub: Subscription;

    constructor(
        private store: AppStore,
        private route: ActivatedRoute,
        private router: Router) {
    }

    ngOnInit() {
        this.sub = this.route.params.subscribe((p) => {
            this.name = p.name;
            this.origin = p.origin;
            this.store.dispatch(fetchBuilds(p.origin, p.name, this.token));
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }

        this.store.dispatch(clearBuilds());
    }

    onSelect(build) {
        this.router.navigate(["builds", build.id]);
    }

    get ident() {
        return {
            origin: this.origin,
            name: this.name
        };
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get builds() {
        return this.store.getState().builds;
    }
}
