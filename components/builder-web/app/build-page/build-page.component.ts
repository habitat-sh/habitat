import { Component, OnInit, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs";
import { fetchBuild } from "../actions/index";
import { requireSignIn } from "../util";
import { AppStore } from "../AppStore";

@Component({
  template: require("./build-page.component.html")
})
export class BuildPageComponent implements OnInit, OnDestroy {
    private routeSub: Subscription;

    constructor(
        private store: AppStore,
        private route: ActivatedRoute) {
        requireSignIn(this);
    }

    ngOnInit() {
        this.routeSub = this.route.params.subscribe((p) => {
            this.store.dispatch(fetchBuild(p.id, this.token));
        });
    }

    ngOnDestroy() {
        if (this.routeSub) {
            this.routeSub.unsubscribe();
        }
    }

    get build() {
      return this.store.getState().builds.selected.info;
    }

    get ident() {
        return {
            origin: this.info.origin,
            name: this.info.name,
            version: this.info.version,
            release: this.info.release
        };
    }

    get info() {
        return this.store.getState().builds.selected.info;
    }

    get token() {
      return this.store.getState().gitHub.authToken;
    }
}
