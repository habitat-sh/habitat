import { Component, Input, OnChanges, OnDestroy, ElementRef } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs";
import * as AnsiUp from "ansi_up";
import * as moment from "moment";
import { fetchBuildLog, streamBuildLog } from "../actions/index";
import { requireSignIn } from "../util";
import { AppStore } from "../AppStore";

@Component({
    selector: "hab-build",
    template: require("./build.component.html")
})
export class BuildComponent implements OnChanges, OnDestroy {
    @Input() build;
    @Input() stream: boolean = false;

    private fetched: boolean = false;
    private logSub: Subscription;

    constructor(
        private store: AppStore,
        private elementRef: ElementRef) {
        requireSignIn(this);
    }

    ngOnChanges(change) {
        let id = change.build.currentValue.id;

        if (id) {
            this.fetch(id);
        }
    }

    ngOnDestroy() {
        if (this.logSub) {
            this.logSub.unsubscribe();
        }

        this.store.dispatch(streamBuildLog(false));
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

    get buildsLink() {
        let link = ["/pkgs", this.build.origin, this.build.name];

        if (this.build.version) {
            link.push(this.build.version);
        }

        return link;
    }

    get elapsed() {
        let started = this.build.build_started_at;
        let finished = this.build.build_finished_at;
        let e;

        if (started && finished) {
            let s = +moment.utc(started);
            let f = +moment.utc(finished);
            e = moment.utc(f - s).format("m [min], s [sec]");
        }

        return e;
    }

    get completed() {
        let finished = this.build.build_finished_at;
        let f;

        if (finished) {
            f = moment.utc(finished).format("dddd, MMMM D, YYYY [at] h:mm:ss A");
        }

        return f;
    }

    get info() {
        return this.store.getState().builds.selected.info;
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    private fetch(id) {
        if (!this.fetched) {
            this.store.dispatch(streamBuildLog(this.stream));
            this.store.dispatch(fetchBuildLog(id, this.token, 0));
            this.fetched = true;
            this.watchForLogs();
        }
    }

    private watchForLogs() {
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
}
