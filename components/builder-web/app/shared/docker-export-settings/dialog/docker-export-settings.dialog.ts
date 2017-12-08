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
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material';

@Component({
  template: require('./docker-export-settings.dialog.html')
})
export class DockerExportSettingsDialog {
  private repoPlaceholder: string;
  private docker_hub_repo_name: string;
  private custom_tag: string;
  private latest_tag: boolean;
  private version_tag: boolean;
  private version_release_tag: boolean;

  constructor(
    private ref: MatDialogRef<DockerExportSettingsDialog>,
    @Inject(MAT_DIALOG_DATA) private data: any
  ) {
    this.repoPlaceholder = data.repoPlaceholder;
    this.docker_hub_repo_name = data.docker_hub_repo_name;
    this.custom_tag = data.custom_tag;
    this.latest_tag = !!data.latest_tag;
    this.version_tag = !!data.version_tag;
    this.version_release_tag = !!data.version_release_tag;
  }

  get settings() {
    return {
      docker_hub_repo_name: this.docker_hub_repo_name,
      custom_tag: this.custom_tag,
      latest_tag: this.latest_tag,
      version_tag: this.version_tag,
      version_release_tag: this.version_release_tag
    };
  }

  ok() {
    this.ref.close(this.settings);
  }

  cancel() {
    this.ref.close(null);
  }
}
