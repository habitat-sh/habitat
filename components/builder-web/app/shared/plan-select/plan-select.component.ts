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

import { Component, Input, OnInit, AfterViewInit, Output, EventEmitter, ViewChild } from "@angular/core";
import { FormControl, FormGroup, FormBuilder, Validators } from "@angular/forms";
import { RouterLink } from "@angular/router";
import { DockerExportSettingsComponent } from "../docker-export-settings/docker-export-settings.component";
import { GitHubApiClient } from "../../GitHubApiClient";
import { GitHubFileResponse } from "../../github/api/shared/github-file-response.model";
import { GitHubFile } from "../../github/file/shared/github-file.model";
import { AppStore } from "../../AppStore";
import config from "../../config";

@Component({
  selector: "hab-plan-select",
  template: require("./plan-select.component.html")
})

export class PackagePlanSelectComponent implements OnInit {
  @Output() planSelected: EventEmitter<object> = new EventEmitter();
  @Output() planSelectCanceled: EventEmitter<boolean> = new EventEmitter();

  @Input() ownerAndRepo: string;
  @Input() project: string;

  @ViewChild("docker")
  docker: DockerExportSettingsComponent;

  gitHubClient: GitHubApiClient = new GitHubApiClient(this.store.getState().gitHub.authToken);
  form: FormGroup;
  control: FormControl;
  formIndex: number = 0;
  plans: Array <GitHubFile>;
  errorText: string;

  disablePlanSelectTab: boolean = true;
  repo: string = "";
  owner: string = "";
  plan: string = "";

  constructor(private formBuilder: FormBuilder, private store: AppStore) {
    this.form = formBuilder.group({});
  }

  get token() {
    return this.store.getState().gitHub.authToken;
  }

  get integrations() {
    return this.store.getState().origins.currentIntegrations.docker;
  }

  onTabChange(tab) {
    this.formIndex = tab.index;
  }

  repoSelected(ownerAndRepo: string) {
    [this.owner, this.repo] = ownerAndRepo.split("/");

    this.gitHubClient.findFileInRepo(this.owner, this.repo, "plan.")
      .then(this.handleGitHubFileResponse.bind(this));

    return false;
  };

  handleGitHubFileResponse(result: GitHubFileResponse) {
    if (result.total_count === 0) {
      this.owner = "";
      this.repo = "";
      this.errorText = "That repo doesn't appear to have a plan file. Please select another repo.";
      return false;
    }

    this.errorText = "";
    this.formIndex = 1;

    this.plans = result.items.map((item) => {
      if (item.name.endsWith(".sh")) {
        item.type = "linux";
      } else if (item.name.endsWith(".ps1")) {
        item.type = "windows";
      }

      return item;
    });
  }

  handleSubmit() {
    this.planSelected.emit({
      project: {
        github: {
          organization: this.owner,
          repo: this.repo,
        },
        plan_path: this.form.get("plan").value
      },
      integrations: {
        docker: this.docker.settings
      }
    });
  }

  cancel() {
    this.planSelectCanceled.emit(true);
  }

  public ngOnInit() {
    this.control = new FormControl("", Validators.compose([
      Validators.required
    ]));

    this.form.addControl("plan", this.control);
  }
}
