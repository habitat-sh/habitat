import { Component, Input } from "@angular/core";
import { iconForBuildState } from "../../util";
import { AppStore } from "../../AppStore";

@Component({
  selector: "hab-build-notice",
  template: require("./build-notice.component.html")
})
export class BuildNoticeComponent {

  @Input() build: any;

  constructor(private store: AppStore) {}

  get status() {
    return this.build.state.toLowerCase();
  }

  get symbol() {
    return iconForBuildState(this.status);
  }
}
