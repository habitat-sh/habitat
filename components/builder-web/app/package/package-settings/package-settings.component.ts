import { Component, Inject, OnInit, OnDestroy, ViewChild } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MdDialog, MdDialogRef } from "@angular/material";
import { Subscription } from "rxjs/subscription";
import { GitHubApiClient } from "../../GitHubApiClient";
import { GitHubRepo } from "../../github/repo/shared/github-repo.model";
import { requireSignIn } from "../../util";
import { AppStore } from "../../AppStore";
import { addNotification, addProject, updateProject, setProjectIntegrationSettings, deleteProject, fetchProject } from "../../actions/index";
import config from "../../config";

@Component({
    selector: "hab-package-settings",
    template: require("./package-settings.component.html")
})
export class PackageSettingsComponent implements OnInit, OnDestroy {
    name: string;
    origin: string;

    private sub: Subscription;

    constructor(private route: ActivatedRoute, private store: AppStore, private disconnectDialog: MdDialog) {
        this.sub = this.route.parent.params.subscribe((params) => {
            this.origin = params["origin"];
            this.name = params["name"];
        });
    }

    ngOnInit() {
        requireSignIn(this);
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    get project() {
        const project = this.store.getState().projects.current;
        const exists = this.store.getState().projects.ui.current.exists;

        const isMember = !!this.store.getState().origins.mine.find((o) => {
            return o.name === this.origin;
        });

        if (isMember && exists) {
            return project;
        }
    }

    get integrations() {
        return this.store.getState().origins.currentIntegrations.docker;
    }

    get loading() {
        return this.store.getState().projects.ui.current.loading;
    }

    saved(project) {
        window.scroll(0, 0);
    }
}
