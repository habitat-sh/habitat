import { Component, EventEmitter, Input, Output } from "@angular/core";
import { List } from "immutable";
import * as moment from "moment";
import { iconForBuildState } from "../../util";

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
        return iconForBuildState(state);
    }
}
