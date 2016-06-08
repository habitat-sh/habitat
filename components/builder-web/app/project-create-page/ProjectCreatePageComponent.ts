// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {ControlGroup, FormBuilder, Validators} from "angular2/common";
import {RouteParams, RouterLink} from "angular2/router";
import {addProject} from "../actions/index";
import {AppStore} from "../AppStore";
import {CheckingInputComponent} from "../CheckingInputComponent";
import {GitHubApiClient} from "../GitHubApiClient";
import {requireSignIn} from "../util";

@Component({
    directives: [CheckingInputComponent, RouterLink],
    template: `
    <div class="hab-project-create">
      <div class="page-title">
          <h2>Add Project</h2>
          <p>
              All projects require a origin (your username or organization
              name) and a path to the plan in the source code repository.
          </p>
      </div>
      <div class="page-body">
          <form [ngFormModel]="form" (ngSubmit)="addProject(form.value)" #formValues="ngForm">
              <div class="scm-repo-fields">
                  <label>GitHub Repository</label>
                  <div *ngIf="repo">
                      <a href="https://github.com/{{ownerAndRepo}}" target="_blank">
                          {{ownerAndRepo}}
                      </a>
                      <a [routerLink]='["SCMRepos"]' href="#">(change)</a>
                  </div>
                  <div *ngIf="!repo">
                      <a [routerLink]='["SCMRepos"]' href="#">
                          (select a GitHub repository)
                      </a>
                  </div>
              </div>
              <div class="project-fields">
                  <div class="origin">
                      <label for="origin">Project Origin</label>
                      <select ngControl="origin"
                              id="origin">
                          <option *ngFor="#origin of myOrigins">
                              {{origin.name}}
                          </option>
                      </select>
                  </div>
                  <div class="name">
                      <label for="name">Project Name</label>
                      <hab-checking-input autofocus=true
                                          displayName="Name"
                                          [form]="form"
                                          id="name"
                                          [isAvailable]="false"
                                          name="name"
                                          placeholder="Required. Max 40 characters."
                                          [value]="repo">
                      </hab-checking-input>
                  </div>
                  <div class="plan">
                      <label for="plan">Path to Plan file</label>
                      <small>The location in the repository of the plan.sh that will build this project.</small>
                      <hab-checking-input availableMessage="exists"
                                          displayName="File"
                                          [form]="form"
                                          id="plan"
                                          [isAvailable]="doesFileExist"
                                          [maxLength]="false"
                                          name="plan"
                                          notAvailableMessage="does not exist in repository"
                                          [pattern]="false"
                                          value="/plan.sh">
                      </hab-checking-input>
                  </div>
                  <div class="submit">
                      <button type="submit"
                              [disabled]="!form.valid">
                          Save Project
                      </button>
                  </div>
              </div>
          </form>
      </div>
    </div>`
})

export class ProjectCreatePageComponent implements OnInit {
    private doesFileExist: Function;
    private form: ControlGroup;
    private isProjectAvailable: Function;

    constructor(private formBuilder: FormBuilder,
        private routeParams: RouteParams, private store: AppStore) {
        this.form = formBuilder.group({
            repo: [this.repo || "", Validators.required],
            origin: [this.store.getState().origins.current.name,
                Validators.required],
            plan: ["/plan.sh", Validators.required],
        });

        this.doesFileExist = function (path) {
            return new GitHubApiClient(
                this.store.getState().gitHub.authToken
            ).doesFileExist(this.repoOwner, this.repo, path);
        }.bind(this);

        // TODO: Implement this
        this.isProjectAvailable = (name) => false;
    }

    get myOrigins() {
        return this.store.getState().origins.mine;
    }

    get ownerAndRepo() {
        if (this.routeParams.params["repo"]) {
            return decodeURIComponent(this.routeParams.params["repo"]);
        } else {
            return undefined;
        }
    }

    get repoOwner() {
        return (this.ownerAndRepo || "").split("/")[0];
    }

    get repo() {
        return (this.ownerAndRepo || "").split("/")[1];
    }

    private addProject(values) {
        this.store.dispatch(addProject(values));
        return false;
    }

    public ngOnInit() {
        requireSignIn(this);

        // Wait a second to set the fields as dirty to do validation on page
        // load. Doing this later in the lifecycle causes a changed after it was
        // checked error.
        setTimeout(() => {
            this.form.controls["plan"].markAsDirty();
            this.form.controls["name"].markAsDirty();
         } , 1000);
    }
}
