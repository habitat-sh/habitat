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

import { Component, OnDestroy } from '@angular/core';
import { MatDialogRef } from '@angular/material';
import { AppStore } from '../../../app.store';
import { clearIntegrationCredsValidation, validateDockerCredentials } from '../../../actions/index';

export interface Credentials {
  username: string;
  password: string;
}

export class Credentials implements Credentials {
  username: string = '';
  password: string = '';
}

@Component({
  selector: 'hab-docker-credentials-dialog',
  template: require('./docker-credentials-form.dialog.html')
})
export class DockerCredentialsFormDialog implements OnDestroy {
  model: Credentials = new Credentials;

  constructor(
    public dialogRef: MatDialogRef<DockerCredentialsFormDialog>,
    private store: AppStore
  ) { }

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
        className: 'validating'
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
    this.store.dispatch(validateDockerCredentials(this.model.username, this.model.password, this.token));
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
  }

  close() {
    this.dialogRef.close();
  }
}
