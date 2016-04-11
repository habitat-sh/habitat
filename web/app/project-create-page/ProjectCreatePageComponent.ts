// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {ControlGroup, FormBuilder, Validators} from "angular2/common";
import {RouteParams, RouterLink} from "angular2/router";
import {addProject} from "../actions/index";
import {AppStore} from "../AppStore";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
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
                      {{repo}}
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
                      <input ngControl="origin" disabled id="origin" name="origin">
                  </div>
                  <div class="name">
                      <label for="name">Project Name</label>
                      <input ngControl="name" id="name" name="name" placeholder="Required. Max 40 characters." required>
                  </div>
                  <div class="plan">
                      <label for="plan">Path to Plan file</label>
                      <small>The location in the repository of the plan.sh that will build this project.</small>
                      <input ngControl="plan" id="plan" name="plan" required>
                  </div>
                  <div class="submit">
                      <button type="submit">Save Project</button>
                  </div>
              </div>
          </form>
      </div>
    </div>`
})

export class ProjectCreatePageComponent {
    private form: ControlGroup;

    constructor(private formBuilder: FormBuilder,
        private routeParams: RouteParams, private store: AppStore) {
        requireSignIn(this);

        this.form = formBuilder.group({
            repo: [this.repo || "", Validators.nullValidator],
            origin: [this.store.getState().user.username, Validators.required],
            name: ["", Validators.required],
            plan: ["/plan.sh", Validators.required],
        });
    }

    get repo() {
        return decodeURIComponent(this.routeParams.params["repo"]);
    }

    private addProject(values) {
        this.store.dispatch(addProject(values));
    }
}
