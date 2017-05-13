import { Component, OnInit, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs";
import { clearBuild, fetchBuild, fetchBuildLog, streamBuildLog } from "../actions/index";
import { requireSignIn } from "../util";
import { AppStore } from "../AppStore";

@Component({
  template: require("./build.component.html")
})
export class BuildComponent implements OnInit, OnDestroy {
    private sub: Subscription;

    constructor(
        private store: AppStore,
        private route: ActivatedRoute) {
        requireSignIn(this);
    }

    ngOnInit() {
        this.sub = this.route.params.subscribe((p) => {
            this.store.dispatch(streamBuildLog(true));
            this.store.dispatch(fetchBuild(p.id, this.token));
            this.store.dispatch(fetchBuildLog(p.id, this.token, 0));
        });
    }

    ngOnDestroy() {
        this.store.dispatch(streamBuildLog(false));

        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    iconFor(state) {
        return {
            Complete: "check",
            Dispatched: "sync",
            Failed: "issue-opened",
            Pending: "clock",
            Processing: "sync",
            Rejected: "issue-opened"
        }[state];
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

    get log() {
        let state = this.store.getState();
        let selected = state.builds.selected;
        let content = selected.log.content;

        if (content && content.length) {
            return content.join("\n")
                // TODO: Remove this once the debug output is cleaned up.
                .replace(/^ ?[\d: ]+/gm, "");
        }
    }

    get token() {
      return this.store.getState().gitHub.authToken;
    }
}
