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

import { Component } from '@angular/core';
import { MatDialog, MatDialogRef } from '@angular/material';

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

export class DockerCredentialsFormDialog {
  model: Credentials = new Credentials;

  constructor(
    public dialogRef: MatDialogRef<DockerCredentialsFormDialog>) { }

  onNoClick(): void {
    this.dialogRef.close();
  }

  onSubmit() {
    this.dialogRef.close(this.model);
  }

  close() {
    this.dialogRef.close();
  }
}
