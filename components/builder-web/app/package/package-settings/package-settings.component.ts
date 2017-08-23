import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/subscription";

@Component({
    template: require("./package-settings.component.html")
})
export class PackageSettingsComponent implements OnDestroy {
    origin: string;
    name: string;

    private sub: Subscription;

    constructor(private route: ActivatedRoute) {
        this.sub = this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.name = params["name"];
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }
}
