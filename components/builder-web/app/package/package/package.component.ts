import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";

@Component({
    template: require("./package.component.html")
})
export class PackageComponent implements OnDestroy {
    origin: string;
    name: string;

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
}
