import { Component, OnInit, OnDestroy, ElementRef } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs";
import * as AnsiUp from "ansi_up";
import { clearBuild, fetchBuild, fetchBuildLog, streamBuildLog } from "../actions/index";
import { requireSignIn } from "../util";
import { AppStore } from "../AppStore";

@Component({
  template: require("./build.component.html")
})
export class BuildComponent implements OnInit, OnDestroy {
    private routeSub: Subscription;
    private logSub: Subscription;

    constructor(
        private store: AppStore,
        private route: ActivatedRoute,
        private elementRef: ElementRef) {
        requireSignIn(this);
    }

    ngOnInit() {
        this.routeSub = this.route.params.subscribe((p) => {
            this.store.dispatch(streamBuildLog(true));
            this.store.dispatch(fetchBuild(p.id, this.token));
            this.store.dispatch(fetchBuildLog(p.id, this.token, 0));
        });

        let pre = this.elementRef.nativeElement.querySelector("pre");
        let content = this.store.getState().builds.selected.log.content;

        this.logSub = content.subscribe((lines) => {
            let fragment = document.createDocumentFragment();

            lines.forEach((line) => {
                let el = document.createElement("div");
                el.innerHTML = AnsiUp.ansi_to_html(line);
                fragment.appendChild(el);
            });

            pre.appendChild(fragment);
        });
    }

    ngOnDestroy() {
        this.store.dispatch(streamBuildLog(false));

        if (this.routeSub) {
            this.routeSub.unsubscribe();
        }

        if (this.logSub) {
            this.logSub.unsubscribe();
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

    get token() {
      return this.store.getState().gitHub.authToken;
    }
}
