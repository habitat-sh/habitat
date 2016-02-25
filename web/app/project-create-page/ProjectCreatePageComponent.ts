// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {addProject} from "../actions/index";
import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {ControlGroup, FormBuilder, Validators} from "angular2/common";

@Component({
    template: `
    <div class="bldr-project-create">
        <h2>Add Project</h2>
        <p>
            All projects require a origin (your username or organization
            name) and a path to the plan in the source code repository.
        </p>
        <form [ngFormModel]="form" (ngSubmit)="addProject(form.value)" #formValues="ngForm">
            <div class="scm-repo-fields">
                <label>GitHub Repository</label>
                smith / example
                <a href="#">(change)</a>
            </div>
            <div class="project-fields">
                <div class="deriv">
                    <label for="origin">Project Origin</label>
                    <input ngControl="origin" disabled id="origin" name="origin">
                </div>
                <div class="name">
                    <label for="name">Project Name</label>
                    <input ngControl="name" id="name" name="name" placeholder="Required. Max 40 characters." required>
                </div>
                <div class="plan">
                    <label for="plan">Path to Plan file</label>
                    <p>The location in the repository of the plan.sh that will build this project.</p>
                    <input ngControl="plan" id="plan" name="plan" required>
                </div>
                <div class="submit">
                    <button type="submit">Save Project</button>
                </div>
            </div>
        </form>
    </div>`
})

export class ProjectCreatePageComponent {
    private form: ControlGroup;

    constructor(private formBuilder: FormBuilder, private store: AppStore) {
        this.form = formBuilder.group({
            origin: ["smith", Validators.required],
            name: ["", Validators.required],
            plan: ["/plan.sh", Validators.required],
        });

    }

    private addProject(values) {
        this.store.dispatch(addProject(values));
    }
}
