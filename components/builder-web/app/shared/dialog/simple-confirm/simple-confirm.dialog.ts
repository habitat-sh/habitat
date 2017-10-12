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

import { Component, Inject } from '@angular/core';
import { MdDialogRef, MD_DIALOG_DATA } from '@angular/material';

@Component({
  template: require('./simple-confirm.dialog.html')
})
export class SimpleConfirmDialog {

  constructor(
    private ref: MdDialogRef<SimpleConfirmDialog>,
    @Inject(MD_DIALOG_DATA) private data: any
  ) { }

  get heading() {
    return this.data.heading || 'Confirm';
  }

  get body() {
    return this.data.body || 'Are you sure?';
  }

  get action() {
    return this.data.action || 'do it';
  }

  ok() {
    this.ref.close(true);
  }

  cancel() {
    this.ref.close(false);
  }
}
