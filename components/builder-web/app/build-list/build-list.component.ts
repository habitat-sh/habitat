import { Component, EventEmitter, Input, Output } from "@angular/core";
import { List } from "immutable";
import * as moment from "moment";

@Component({
  selector: "hab-build-list",
  template: require("./build-list.component.html")
})
export class BuildListComponent {
    @Input() builds = List();
    @Output() select = new EventEmitter();

    onClick(build) {
        this.select.emit(build);
    }

    dateFor(timestamp) {
        return moment(timestamp, "YYYY-MM-DDTHH:mm:ss").format("YYYY-MM-DD");
    }

    iconFor(state) {
        return {
            Complete: "check",
            Dispatched: "loading",
            Failed: "alert",
            Pending: "clock",
            Processing: "loading",
            Rejected: "alert"
        }[state];
    }

    get activeBuild() {
        for (let i = 0; i < this.builds.size; i++ ) {
            let build = this.builds.get(i);

            if (build["state"] === "Dispatched") {
                return build;
            }
        }

        return null;
    }
}
