import { Component, Input } from "@angular/core";
import { AppStore } from "../AppStore";

@Component({
  selector: "hab-build-status",
  template: `
    <hab-icon
      [symbol]="iconFor(state)"
      class="status {{ state | lowercase }}"
      *ngIf="state"></hab-icon>
  `
})
export class BuildStatusComponent {
  @Input() origin;
  @Input() name;
  @Input() version;
  @Input() release;

  constructor(private store: AppStore) {}

  iconFor(state) {
      return {
          Complete: "check",
          Dispatched: "loading",
          Failed: "alert",
          Pending: "pending",
          Processing: "loading",
          Rejected: "alert"
      }[state];
  }

  get id() {
    return this.first() ? this.first().id : null;
  }

  get state() {
    return this.first() ? this.first().state : null;
  }

  private first() {
    return this.store.getState().builds.visible.find((b) => {
      if (b.origin === this.origin && b.name === this.name) {
        return this.version ? b.version === this.version : true;
      }
    });
  }
}
