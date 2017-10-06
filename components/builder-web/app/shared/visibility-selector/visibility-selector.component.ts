import { Component, EventEmitter, Input, Output } from "@angular/core";

@Component({
  selector: "hab-visibility-selector",
  template: require("./visibility-selector.component.html")
})
export class VisibilitySelectorComponent {

  @Input() setting: string = "public";
  @Output() changed: EventEmitter<string> = new EventEmitter<string>();

  change() {
    this.changed.emit(this.setting);
  }
}
