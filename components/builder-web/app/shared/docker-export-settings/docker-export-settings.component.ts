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

import { Component, Input, OnChanges, OnDestroy, SimpleChanges } from '@angular/core';
import { MatDialog } from '@angular/material';
import { DockerExportSettingsDialog } from './dialog/docker-export-settings.dialog';
import { fetchProjectIntegration } from '../../actions/projects';
import { AppStore } from '../../app.store';

@Component({
  selector: 'hab-docker-export-settings',
  template: require('./docker-export-settings.component.html')
})
export class DockerExportSettingsComponent implements OnChanges, OnDestroy {
  @Input() origin: string;
  @Input() package: string;
  @Input() integrations: any;
  @Input() current: any;
  @Input() enabled: boolean = false;

  private name: string;
  private docker_hub_repo_name: string = '';
  private custom_tag: string;
  private latest_tag: boolean = true;
  private version_tag: boolean = true;
  private version_release_tag: boolean = true;
  private unsubscribe: Function;

  constructor(
    private store: AppStore,
    private dialog: MatDialog
  ) { }

  get configured() {
    return Object.keys(this.integrations).length > 0;
  }

  get settings(): any {
    return {
      valid: this.valid,
      name: this.name,
      enabled: this.enabled,
      settings: {
        docker_hub_repo_name: this.docker_hub_repo_name,
        custom_tag: this.custom_tag,
        latest_tag: this.latest_tag,
        version_tag: this.version_tag,
        version_release_tag: this.version_release_tag
      }
    };
  }

  get repoPlaceholder() {
    return this.store.getState().projects.current.name || `${this.username}/example-repo`;
  }

  get token() {
    return this.store.getState().session.token;
  }

  get username() {
    return this.store.getState().users.current.username;
  }

  get valid() {

    if (this.docker_hub_repo_name && this.docker_hub_repo_name.trim() !== '') {
      return true;
    }

    return false;
  }

  applySettings(name, settings) {
    this.name = name;
    this.docker_hub_repo_name = settings.docker_hub_repo_name;
    this.custom_tag = settings.custom_tag;
    this.latest_tag = settings.latest_tag;
    this.version_tag = settings.version_tag;
    this.version_release_tag = settings.version_release_tag;
  }

  configure(integration) {
    this.name = integration;
    const integrations = this.store.getState().projects.current.settings;
    const settings = integrations.get(integration) || {};

    this.dialog
      .open(DockerExportSettingsDialog, {
        data: {
          repoPlaceholder: this.repoPlaceholder,
          docker_hub_repo_name: settings.docker_hub_repo_name,
          custom_tag: settings.custom_tag,
          latest_tag: settings.latest_tag,
          version_tag: settings.version_tag,
          version_release_tag: settings.version_release_tag
        },
        width: '480px'
      })
      .afterClosed()
      .subscribe((result) => {
        if (result) {
          this.applySettings(integration, result);
        }
      });
  }

  decode(s) {
    return decodeURIComponent(s);
  }

  ngOnChanges(changes: SimpleChanges) {
    const i: any = changes['integrations'];

    if (i && i.currentValue) {
      const integrations = i.currentValue;

      Object.keys(integrations).forEach(key => {
        integrations[key].forEach(name => {
          this.store.dispatch(fetchProjectIntegration(this.origin, this.package, name, this.token));
        });
      });

      this.unsubscribe = this.store.subscribe(state => {
        state.projects.current.settings.forEach((v, k) => {
          this.applySettings(k, v);
          this.unsubscribe();
          return false;
        });
      });
    }
  }

  ngOnDestroy() {
    if (this.unsubscribe) {
      this.unsubscribe();
    }
  }

  onChange(name) {
    const settings = this.settingsFor(name);
    this.applySettings(name, settings || {});

    if (!settings) {
      this.configure(name);
    }
  }

  settingsFor(integration) {
    return this.store.getState().projects.current.settings.get(integration);
  }
}
