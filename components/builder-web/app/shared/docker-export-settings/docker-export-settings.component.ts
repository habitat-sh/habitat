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

import { Component, Input, Output, EventEmitter, OnChanges } from '@angular/core';
import { FormControl } from '@angular/forms';
import { AppStore } from '../../app.store';

@Component({
  selector: 'hab-docker-export-settings',
  template: require('./docker-export-settings.component.html')
})
export class DockerExportSettingsComponent implements OnChanges {
  @Input() integrations: any;
  @Input() current: any;
  @Input() enabled: boolean = false;

  private name: string;
  private repoName: string = '';
  private customTag: string;
  private latestTag: boolean = true;
  private versionTag: boolean = true;
  private releaseTag: boolean = true;

  constructor(private store: AppStore) { }

  get configured() {
    return this.integrations.size > 0;
  }

  get settings(): any {
    return {
      valid: this.valid,
      name: this.name,
      enabled: this.enabled,
      settings: {
        docker_hub_repo_name: this.repoName,
        custom_tag: this.customTag,
        latest_tag: this.latestTag,
        version_tag: this.versionTag,
        version_release_tag: this.releaseTag
      }
    };
  }

  get repoPlaceholder() {
    return this.store.getState().projects.current.name || `${this.username}/example-repo`;
  }

  get username() {
    return this.store.getState().users.current.username;
  }

  get valid() {

    if (this.repoName.trim() !== '') {
      return true;
    }

    return false;
  }

  ngOnChanges(changes) {

    if (changes.integrations) {
      this.name = changes.integrations.currentValue.get(0);
    }

    if (changes.current) {
      const value = changes.current.currentValue;

      if (value) {
        this.repoName = value.docker_hub_repo_name;
        this.customTag = value.custom_tag;
        this.latestTag = value.latest_tag;
        this.versionTag = value.version_tag;
        this.releaseTag = value.version_release_tag;
      }
    }
  }
}
