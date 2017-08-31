import { AfterViewChecked, Component, ElementRef, Input } from "@angular/core";

@Component({
  selector: "hab-icon",
  template: `
    <md-icon [svgIcon]="id" [mdTooltip]="tooltip" mdTooltipPosition="above"></md-icon>
  `
})
export class IconComponent implements AfterViewChecked {
  @Input() symbol: string;
  @Input() title: string = "";

  private el: ElementRef;

  constructor(el: ElementRef) {
    this.el = el;
  }

  ngAfterViewChecked() {
    if (this.svg) {
      this.svg.setAttribute("viewBox", "0 0 24 24");
    }
  }

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

  get svg() {
    return this.el.nativeElement.querySelector("svg");
  }
}
