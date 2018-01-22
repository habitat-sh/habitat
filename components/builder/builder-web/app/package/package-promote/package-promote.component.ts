import { Component, Input } from '@angular/core';
import { MatDialog } from '@angular/material';
import { AppStore } from '../../app.store';
import { promotePackage } from '../../actions/packages';
import { SimpleConfirmDialog } from '../../shared/dialog/simple-confirm/simple-confirm.dialog';

@Component({
  selector: 'hab-package-promote',
  template: require('./package-promote.component.html')
})
export class PackagePromoteComponent {
  @Input() origin: string;
  @Input() name: string;
  @Input() version: string;
  @Input() release: string;
  @Input() channel: string;

  promoting: boolean = false;

  constructor(
    private confirmDialog: MatDialog,
    private store: AppStore
  ) { }

  prompt(evt) {
    evt.stopPropagation();

    this.confirmDialog
      .open(SimpleConfirmDialog, {
        width: '480px',
        data: {
          heading: 'Confirm promote',
          body: `Are you sure you want to promote this artifact? Doing so will add the artifact to the ${this.channel} channel.`,
          action: 'promote it'
        }
      })
      .afterClosed()
      .subscribe((confirmed) => {
        if (confirmed) {
          this.promoting = true;

          setTimeout(() => {
            this.store.dispatch(
              promotePackage(this.origin, this.name, this.version, this.release, this.channel, this.store.getState().session.token)
            );
          }, 1000);
        }
      });
  }
}
