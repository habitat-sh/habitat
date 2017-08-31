import { Component, Input } from "@angular/core";
import { targetToPlatform, releaseToDate } from "../../util";

@Component({
    selector: "hab-package-detail",
    template: require("./package-detail.component.html")
})
export class PackageDetailComponent {
    @Input() package: object;

    releaseToDate(release) {
        return releaseToDate(release);
    }

    get fullName() {
      let ident = this.package["ident"];
      let name = "";

      if (ident.origin && ident.name) {
        name = `${ident.origin}/${ident.name}`;
      }

      return name;
    }
}
