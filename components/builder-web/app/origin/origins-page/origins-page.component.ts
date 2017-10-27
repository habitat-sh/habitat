// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { MatDialog } from '@angular/material';
import { SimpleConfirmDialog } from '../../shared/dialog/simple-confirm/simple-confirm.dialog';
import { acceptOriginInvitation, fetchMyOriginInvitations, fetchMyOrigins, ignoreOriginInvitation } from '../../actions/index';
import { AppStore } from '../../app.store';
import config from '../../config';

@Component({
  template: require('./origins-page.component.html')
})
export class OriginsPageComponent implements OnInit {

  constructor(
    private store: AppStore,
    private router: Router,
    private confirmDialog: MatDialog
  ) { }

  ngOnInit() {
    if (this.token) {
      this.store.dispatch(fetchMyOrigins(this.token));
      this.store.dispatch(fetchMyOriginInvitations(this.token));
    }
  }

  get config() {
    return config;
  }

  get origins() {
    const mine = this.store.getState().origins.mine;
    const invites = this.store.getState().origins.myInvitations;
    return mine.concat(invites).sortBy(item => item.name || item.origin_name);
  }

  get token() {
    return this.store.getState().session.token;
  }

  get ui() {
    return this.store.getState().origins.ui.mine;
  }

  accept(item) {
    this.store.dispatch(acceptOriginInvitation(
      item.origin_invitation_id, item.origin_name, this.token
    ));
  }

  ignore(item) {
    const data = {
      heading: 'Confirm ignore',
      body: `Are you sure you want to ignore this invitation? Doing so will prevent
                access to this origin and its private packages.`,
      action: 'ignore it'
    };

    this.confirm(data, () => {
      this.store.dispatch(ignoreOriginInvitation(
        item.origin_invitation_id, item.origin_name, this.token
      ));
    });
  }

  name(item) {
    return item.name || item.origin_name;
  }

  navigateTo(item) {
    if (!this.isInvitation(item)) {
      this.router.navigate(['/origins', item.name]);
    }
  }

  packageCount(item) {
    const count = item.packageCount;
    return count >= 0 ? count : '-';
  }

  isInvitation(item) {
    return !!item.origin_invitation_id;
  }

  private confirm(data, then) {
    this.confirmDialog
      .open(SimpleConfirmDialog, { width: '480px', data: data })
      .afterClosed()
      .subscribe((confirmed) => {
        if (confirmed) {
          then();
        }
      });
  }
}
