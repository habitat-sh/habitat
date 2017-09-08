import { Component, Input } from "@angular/core";

@Component({
  selector: "hab-icon",
  template: `<md-icon [svgIcon]="id" [mdTooltip]="tooltip" mdTooltipPosition="above"></md-icon>`
})
export class IconComponent {
  @Input() symbol: string;
  @Input() title: string = "";

  get id() {
    if (this.symbol) {
      return `icon-${this.symbol}`;
    }
  }

  get tooltip() {
    let tip;

    if (this.title.trim() !== "") {
      tip = this.title;
    }

    return tip;
  }
}
