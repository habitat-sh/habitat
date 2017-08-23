import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/subscription";

@Component({
    template: require("./package-readme.component.html")
})
export class PackageReadmeComponent implements OnDestroy {
    origin: string;
    name: string;

    private sub: Subscription;

    constructor(private route: ActivatedRoute) {
        this.sub = this.route.params.subscribe((params) => {
            let parentParams = this.route.parent.params["value"];
            this.origin = parentParams["origin"];
            this.name = parentParams["name"];
        });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }
}
