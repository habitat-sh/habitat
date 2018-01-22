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

import { Component, Inject, OnDestroy } from '@angular/core';
import { MAT_DIALOG_DATA, MatDialogRef } from '@angular/material';
import { AppStore } from '../../../app.store';
import { clearIntegrationCredsValidation, validateIntegrationCredentials } from '../../../actions/index';

export interface Credentials {
  name: string;
  username: string;
  password: string;
  registry_url: string;
}

export class Credentials implements Credentials {
  name: string;
  username: string = '';
  password: string = '';
  registry_url: string;
}

@Component({
  selector: 'hab-integration-credentials-dialog',
  template: require('./integration-credentials-form.dialog.html')
})
export class IntegrationCredentialsFormDialog implements OnDestroy {
  model: Credentials = new Credentials;

  constructor(
    public dialogRef: MatDialogRef<IntegrationCredentialsFormDialog>,
    @Inject(MAT_DIALOG_DATA) public data: any,
    private store: AppStore
  ) {
    this.model.name = data.name;
    this.model.username = data.username;
    this.model.registry_url = data.registry_url;
  }

  ngOnDestroy() {
    this.store.dispatch(clearIntegrationCredsValidation());
  }

  get token() {
    return this.store.getState().session.token;
  }

  get creds() {
    return this.store.getState().origins.currentIntegrations.ui.creds;
  }

  get message() {
    return this.creds.message;
  }

  get status() {
    let creds = this.creds;

    if (creds.validating) {
      return {
        icon: 'loading',
        className: 'waiting'
      };
    }
    else if (creds.validated) {
      if (creds.valid) {
        return {
          icon: 'check',
          className: 'success'
        };
      }
      else {
        return {
          icon: 'warning',
          className: 'error'
        };
      }
    }
  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  onSubmit() {
    if (this.data.type === 'docker') {
      this.store.dispatch(validateIntegrationCredentials(this.model.username, this.model.password, this.token, this.data.type));
      let unsubscribe;

      unsubscribe = this.store.subscribe(state => {
        const creds = state.origins.currentIntegrations.ui.creds;

        if (!creds.validating && creds.validated) {
          unsubscribe();

          if (creds.valid) {
            setTimeout(() => this.dialogRef.close(this.model), 750);
          }
        }
      });
    } else {
      // We can currently only validate DockerHub creds (╯︵╰,)
      this.dialogRef.close(this.model);
    }
  }

  close() {
    this.dialogRef.close();
  }
}
