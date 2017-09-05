import { Component, Input } from "@angular/core";
import { List } from "immutable";
import { packageString, releaseToDate } from "../../util";

@Component({
    selector: "hab-search-results",
    template: require("./results.component.html")
})
export class SearchResultsComponent {
    @Input() errorMessage: string;
    @Input() noPackages: boolean;
    @Input() packages: List<Object>;

    routeFor(pkg) {
        return ["/pkgs", pkg.origin, pkg.name, "latest"];
    }

    packageString(pkg) {
        return packageString(pkg);
    }
}
