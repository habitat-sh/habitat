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

import { Component, EventEmitter, Input, OnInit, OnChanges, Output, SimpleChanges, ViewChild } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { MdDialog, MdDialogRef } from '@angular/material';
import { DisconnectConfirmDialog } from './dialog/disconnect-confirm/disconnect-confirm.dialog';
import { DockerExportSettingsComponent } from '../../shared/docker-export-settings/docker-export-settings.component';
import { BuilderApiClient } from '../../BuilderApiClient';
import { GitHubApiClient } from '../../GitHubApiClient';
import { AppStore } from '../../app.store';
import {
  addProject, updateProject, setProjectIntegrationSettings, deleteProject,
  fetchGitHubInstallations, fetchProject, setProjectVisibility
} from '../../actions/index';
import config from '../../config';

@Component({
  selector: 'hab-project-settings',
  template: require('./project-settings.component.html')
})
export class ProjectSettingsComponent implements OnChanges {
  connecting: boolean = false;
  doesFileExist: Function;
  doesRepoExist: Function;
  form: FormGroup;
  selectedInstallation: any;
  selectedRepo: string;
  selectedPath: string;

  @Input() integrations;
  @Input() name: string;
  @Input() origin: string;
  @Input() project: any;

  @Output() saved: EventEmitter<any> = new EventEmitter<any>();
  @Output() toggled: EventEmitter<any> = new EventEmitter<any>();

  @ViewChild('docker')
  docker: DockerExportSettingsComponent;

  private api: BuilderApiClient;
  private defaultPath = 'habitat/plan.sh';
  private _visibility: string;

  constructor(private formBuilder: FormBuilder, private store: AppStore, private disconnectDialog: MdDialog) {
    this.api = new BuilderApiClient(this.token);
    this.selectedPath = this.defaultPath;

    this.doesRepoExist = (repo) => {
      return new Promise((resolve, reject) => {
        const matched = this.installations.filter((i) => {
          return i.get('full_name') === repo.trim();
        });

        if (matched.size > 0) {
          resolve(matched.get(0).get('full_name'));
        }
        else {
          reject();
        }
      });
    };

    this.doesFileExist = (path) => {
      return this.api.findFileInRepo(
        this.selectedInstallation.get('installation_id'),
        this.selectedInstallation.get('org'),
        this.selectedInstallation.get('name'),
        this.planField.value
      );
    };
  }

  ngOnChanges(changes: SimpleChanges) {
    const p = changes['project'];

    if (p && p.currentValue) {
      this.selectedRepo = p.currentValue.vcs_data;
      this.selectedPath = p.currentValue.plan_path;
      this.visibility = p.currentValue.visibility || this.visibility;
    }
  }

  get config() {
    return config;
  }

  get connectButtonLabel() {
    return this.project ? 'Update' : 'Save';
  }

  get dockerEnabled() {
    return this.dockerSettings && this.dockerSettings.docker_hub_repo_name !== '';
  }

  get dockerSettings() {
    return this.store.getState().projects.current.settings;
  }

  get files() {
    return this.store.getState().gitHub.files;
  }

  get hasPrivateKey() {
    const currentOrigin = this.store.getState().origins.current;
    return currentOrigin.name === this.origin && !!currentOrigin.private_key_name;
  }

  get installations() {
    return this.store.getState().gitHub.installations;
  }

  get loading() {
    return this.store.getState().gitHub.ui.installations.loading;
  }

  get orgs() {
    return this.store.getState().gitHub.orgs;
  }

  get planField() {
    return this.form.controls['plan_path'];
  }

  get planTemplate() {
    return {
      'origin': this.origin,
      'plan_path': this.planField.value,
      'installation_id': this.selectedInstallation.get('installation_id'),
      'repo_id': this.selectedInstallation.get('repo_id')
    };
  }

  get projectsEnabled() {
    return !!this.store.getState().featureFlags.current.get('project');
  }

  get repoField() {
    return this.form.controls['repo_path'];
  }

  get repos() {
    return this.store.getState().gitHub.installationRepositories;
  }

  get repoUrl() {
    if (this.selectedInstallation) {
      return `https://github.com/${this.selectedInstallation.get('full_name')}`;
    }
  }

  get token() {
    return this.store.getState().session.token;
  }

  get username() {
    return this.store.getState().users.current.gitHub.get('login');
  }

  get validRepo() {
    return this.repoField ? this.repoField.valid : false;
  }

  get validProject() {
    const planPathValid = this.planField ? this.planField.valid : false;
    const dockerValid = (this.docker && this.docker.settings.enabled) ? this.docker.settings.valid : true;
    return this.selectedInstallation && dockerValid && planPathValid;
  }

  get visibility() {
    return this._visibility || this.store.getState().origins.current.default_package_visibility || 'public';
  }

  set visibility(v: string) {
    this._visibility = v;
  }

  connect() {
    this.deselect();
    this.store.dispatch(fetchGitHubInstallations());
    this.connecting = true;
    this.toggled.emit(this.connecting);
  }

  disconnect() {
    const ref = this.disconnectDialog.open(DisconnectConfirmDialog, {
      width: '460px'
    });

    ref.afterClosed().subscribe((confirmed) => {
      if (confirmed) {
        this.store.dispatch(deleteProject(this.project.name, this.token));
      }
    });
  }

  iconFor(path) {
    return this.isWindows(path) ? 'windows' : 'linux';
  }

  isWindows(path) {
    return !!path.match(/\.ps1$/);
  }

  clearConnection() {
    this.connecting = false;
    this.deselect();
    this.toggled.emit(this.connecting);
    window.scroll(0, 0);
  }

  deselect() {
    this.form = this.formBuilder.group({});
    this.selectedRepo = null;
    this.selectedInstallation = null;
    this.selectedPath = this.defaultPath;
  }

  editConnection() {
    this.clearConnection();
    this.connect();
    this.selectedRepo = this.parseGitHubUrl(this.project.vcs_data);
    this.selectedPath = this.project.plan_path;
    setTimeout(() => {
      if (this.repoField) {
        this.repoField.markAsDirty();
      }
    }, 1000);
  }

  saveConnection() {
    if (this.project) {
      this.store.dispatch(updateProject(this.project.name, this.planTemplate, this.token, (result) => {
        this.handleSaved(result.success, this.project.origin_name, this.project.package_name);
      }));
    }
    else {
      this.store.dispatch(addProject(this.planTemplate, this.token, (result) => {
        this.handleSaved(result.success, result.response.origin_name, result.response.package_name);
      }));
    }
  }

  selectRepo() {
    this.selectedInstallation = this.installations.find(i => i.get('full_name') === this.repoField.value.trim());
    setTimeout(() => {
      if (this.planField) {
        this.planField.markAsDirty();
      }
    }, 1000);
  }

  settingChanged(setting) {
    this.visibility = setting;
  }

  private parseGitHubUrl(url) {
    return (url.match(/github.com\/(.+)\.git$/) || [''])[1] || '';
  }

  private handleSaved(successful, origin, name) {
    if (successful) {
      this.saveVisibility(origin, name);
      this.saveIntegration(origin, name);
      this.store.dispatch(fetchProject(origin, name, this.token, false));
      this.saved.emit({ origin: origin, name: name });
      this.clearConnection();
    }
  }

  private saveIntegration(origin, name) {
    const settings = this.docker.settings;

    if (settings.enabled) {
      this.store.dispatch(
        setProjectIntegrationSettings(
          origin, name, settings.name, settings.settings, this.token
        )
      );
    }
  }

  private saveVisibility(origin, name) {
    this.store.dispatch(setProjectVisibility(origin, name, this.visibility, this.token));
  }
}
