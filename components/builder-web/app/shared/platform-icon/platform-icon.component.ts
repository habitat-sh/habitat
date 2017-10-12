import { Component, Input } from '@angular/core';
import { targetToPlatform } from '../../util';

@Component({
  selector: 'hab-platform-icon',
  template: `<hab-icon [symbol]="os" class="icon-os" [title]="os | titlecase"></hab-icon>`
})
export class PlatformIconComponent {

  @Input() platform;

  get os() {
    return targetToPlatform(this.platform);
  }
}
