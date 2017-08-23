import { Component, Input, OnChanges, SimpleChanges } from "@angular/core";
import { packageString, releaseToDate } from "../../util";
import { AppStore } from "../../AppStore";
import { fetchLatestPackage, submitJob } from "../../actions/index";

@Component({
    selector: "hab-package-sidebar",
    template: require("./package-sidebar.component.html")
})
export class PackageSidebarComponent implements OnChanges {
    @Input() origin: string;
    @Input() name: string;

    constructor(private store: AppStore) {}

    ngOnChanges(changes: SimpleChanges) {
        let fetch = false;

        if (changes["origin"]) {
            this.origin = changes["origin"].currentValue;
            fetch = true;
        }

        if (changes["name"]) {
            this.name = changes["name"].currentValue;
            fetch = true;
        }

        if (fetch) {
            this.fetchLatest();
        }
    }

    get latest() {
        return this.store.getState().packages.latest;
    }

    build() {
        if (this.buildable) {
            let token = this.store.getState().gitHub.authToken;
            this.store.dispatch(submitJob(this.origin, this.name, token));
        }
    }

    get exportCommand() {
        return `hab pkg export docker ${this.origin}/${this.name}`;
    }

    get runCommand() {
        return `hab start ${this.origin}/${this.name}`;
    }

    get buildable() {
      let isMember = !!this.store.getState().origins.mine.find((o) => {

        // Still limiting builds to the core origin
        return o.name === this.origin && this.origin === "core";
      });

      if (isMember) {
          return true;
      }

      return false;
    }

    private fetchLatest() {
      this.store.dispatch(fetchLatestPackage(this.origin, this.name));
    }
}
