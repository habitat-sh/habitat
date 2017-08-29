import { Component, Input } from "@angular/core";

@Component({
  selector: "hab-channels",
  template: require("./channels.component.html")
})
export class ChannelsComponent {

  @Input() channels: string[];
}
