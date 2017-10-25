import { Component } from '@angular/core';
import { AppStore } from '../app.store';

@Component({
  selector: 'hab-banner',
  template: require('./banner.component.html')
})
export class BannerComponent {
  dismissed: boolean = false;

  constructor(private store: AppStore) {}

  get hidden() {
    return this.profile.id && !this.profile.email && !this.dismissed;
  }

  get profile() {
    return this.store.getState().users.current.profile;
  }

  dismiss() {
    this.dismissed = true;
  }
}
