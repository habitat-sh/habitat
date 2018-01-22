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

import { Component, Inject, OnInit } from '@angular/core';
import { FormControl, FormGroup, FormBuilder, Validators } from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material';
import { uploadOriginPrivateKey, uploadOriginPublicKey } from '../../../../actions/index';
import { parseKey } from '../../../../util';
import { AppStore } from '../../../../app.store';
import config from '../../../../config';

@Component({
  selector: 'hab-key-add-form',
  template: require('./key-add-form.dialog.html')
})
export class KeyAddFormDialog implements OnInit {
  originName: string;
  type: string;
  form: FormGroup;
  control: FormControl;

  constructor(private formBuilder: FormBuilder, private store: AppStore,
    public dialogRef: MatDialogRef<KeyAddFormDialog>,
    @Inject(MAT_DIALOG_DATA) public data: any) {
    this.originName = data.origin;
    this.type = data.type;
    this.form = formBuilder.group({});
  }

  ngOnInit() {
    this.control = new FormControl(
      '',
      Validators.compose([
        Validators.required,
        this.keyFormatValidator,
        this.keyTypeValidator.bind(this),
        this.originMatchValidator.bind(this),
      ])
    );

    this.form.addControl('key', this.control);
  }

  submit(key) {
    if (this.type === 'public') {
      this.store.dispatch(uploadOriginPublicKey(key, this.token));
    } else {
      this.store.dispatch(uploadOriginPrivateKey(key, this.token));
    }
    this.dialogRef.close();
  }

  keyFormatValidator(control) {
    if (parseKey(control.value).valid) {
      return null;
    } else {
      return { invalidFormat: true };
    }
  }

  close() {
    this.dialogRef.close();
  }

  get token() {
    return this.store.getState().session.token;
  }

  get ui() {
    return this.store.getState().origins.ui.current;
  }

  get icon() {
    if (this.type === 'public') {
      return 'visibility';
    } else {
      return 'visibility-off';
    }
  }

  get docsUrl() {
    return config['docs_url'];
  }

  get keyFileHeaderPrefix() {
    if (this.type === 'public') {
      return 'SIG-PUB-1';
    } else {
      return 'SIG-SEC-1';
    }
  }

  private keyTypeValidator(control) {
    if (parseKey(control.value).type === this.keyFileHeaderPrefix) {
      return null;
    } else {
      return { invalidType: true };
    }
  }

  private originMatchValidator(control) {
    if (parseKey(control.value).origin === this.originName) {
      return null;
    } else {
      return { invalidOrigin: true };
    }
  }
}
