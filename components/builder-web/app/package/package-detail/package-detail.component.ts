import { Component, Input } from "@angular/core";
import { releaseToDate } from "../../util";

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

    osIconFor(pkg) {
      let icon;

      if (pkg.target) {
        if (pkg.target.match("windows")) {
          icon = "windows";
        }
        else if (pkg.target.match("linux")) {
          icon = "linux";
        }
      }

      return icon;
  }
}
