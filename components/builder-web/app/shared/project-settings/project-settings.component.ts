import { Component, EventEmitter, Input, Output, ViewChild } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DisconnectConfirmDialog } from "./dialog/disconnect-confirm/disconnect-confirm.dialog";
import { Subscription } from "rxjs/subscription";
import { DockerExportSettingsComponent } from "../../shared/docker-export-settings/docker-export-settings.component";
import { GitHubApiClient } from "../../GitHubApiClient";
import { GitHubRepo } from "../../github/repo/shared/github-repo.model";
import { requireSignIn } from "../../util";
import { AppStore } from "../../AppStore";
import { addNotification, addProject, updateProject, setProjectIntegrationSettings, deleteProject, fetchGitHubFiles, fetchGitHubOrgs,
         fetchGitHubRepos, fetchProject, clearGitHubRepos } from "../../actions/index";
import config from "../../config";

@Component({
    selector: "hab-project-settings",
    template: require("./project-settings.component.html")
})
export class ProjectSettingsComponent {
    connecting: boolean = false;
    filter: GitHubRepo = new GitHubRepo();
    selectedOrg: string;
    selectedRepo: string;
    selectedPath: string;

    @Input() integrations;
    @Input() name: string;
    @Input() origin: string;
    @Input() project: any;

    @Output() saved: EventEmitter<any> = new EventEmitter<any>();
    @Output() toggled: EventEmitter<any> = new EventEmitter<any>();

    @ViewChild("docker")
    docker: DockerExportSettingsComponent;

    private sub: Subscription;

    constructor(private store: AppStore, private disconnectDialog: MdDialog) {}

    get config() {
        return config;
    }

    get connectButtonLabel() {
        return this.project ? "Update" : "Save";
    }

    get files() {
        return this.store.getState().gitHub.files;
    }

    get orgs() {
        return this.store.getState().gitHub.orgs;
    }

    get planTemplate() {
        return {
            "origin": this.origin,
            "plan_path": this.selectedPath,
            "github": {
                "organization": this.selectedOrg,
                "repo": this.selectedRepo
            }
        };
    }

    get repos() {
        return this.store.getState().gitHub.repos;
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get user() {
        return this.store.getState().users.current.gitHub;
    }

    get valid() {
        const dockerValid = (this.docker && this.docker.settings.enabled) ? this.docker.settings.valid : true;
        return !!this.selectedPath && dockerValid;
    }

    connect() {
        this.store.dispatch(fetchGitHubOrgs());
        this.connecting = true;
        this.toggled.emit(this.connecting);
    }

    disconnect() {
        const ref = this.disconnectDialog.open(DisconnectConfirmDialog, {
            width: "460px"
        });

        ref.afterClosed().subscribe((confirmed) => {
            if (confirmed) {
                this.store.dispatch(deleteProject(this.project.name, this.token));
            }
        });
    }

    iconFor(path) {
        return this.isWindows(path) ? "windows" : "linux";
    }

    isWindows(path) {
        return !!path.match(/\.ps1$/);
    }

    clearConnection() {
        this.connecting = false;
        this.deselect();
        this.toggled.emit(this.connecting);
    }

    deselect() {
        this.selectedOrg = null;
        this.selectedRepo = null;
        this.selectedPath = null;
        this.filter = new GitHubRepo();
        this.store.dispatch(clearGitHubRepos());
    }

    editConnection() {
        this.clearConnection();
        this.connect();
        this.selectedPath = this.project.plan_path;
    }

    saveConnection() {
        new GitHubApiClient(this.token)
            .getFileContent(this.selectedOrg, this.selectedRepo, this.selectedPath)
            .then((response) => {

                // Plan variables may be prefixed with a $
                const dedollar = (key) => key.replace(/^\$/, "");

                // Values may contain quotes
                const dequote = (val) => val.replace(/["']/g, "");

                const content = atob(response["content"]);
                const lines = content.split("\n");
                const ident = lines.filter((line) => ["pkg_name", "pkg_origin"].includes(dedollar(line).split("=")[0]));
                let planVars = {};

                ident.forEach((i) => {
                    const s = i.split("=");
                    planVars[dedollar(s[0]).trim()] = dequote(s[1]).trim();
                });

                const planOrigin = planVars["pkg_origin"];
                const planName = planVars["pkg_name"];

                if (this.name) {
                    if (planName === this.name && planOrigin === this.origin) {
                        if (this.project) {
                            this.store.dispatch(updateProject(this.project.name, this.planTemplate, this.token, (result) => {
                                this.handleSaved(result.success, planOrigin, planName);
                            }));
                        }
                        else {
                            this.store.dispatch(addProject(this.planTemplate, this.token, (result) => {
                                this.handleSaved(result.success, planOrigin, planName);
                            }));
                        }
                    }
                    else {
                        this.store.dispatch(addNotification({
                            type: "danger",
                            title: "Invalid Selection",
                            body: `The origin and name in this plan file (${planOrigin}/${planName})
                                must match those of this package (${this.origin}/${this.name}).`
                        }));
                    }
                }
                else {
                    if (planOrigin === this.origin) {
                        this.store.dispatch(addProject(this.planTemplate, this.token, (result) => {
                          this.handleSaved(result.success, planOrigin, planName);
                        }));
                    }
                    else {
                        this.store.dispatch(addNotification({
                            type: "danger",
                            title: "Invalid Selection",
                            body: `The origin in this plan file (${planOrigin}) must match
                                the current origin (${this.origin}).`
                        }));
                    }
                }
            })
            .catch((error) => {
                this.store.dispatch(addNotification({
                    type: "danger",
                    title: "Error reading plan file",
                    body: `The message from GitHub was ${error.message}.`
                }));
            });
    }

    selectOrg(org) {
        this.selectedOrg = org;
        this.store.dispatch(fetchGitHubRepos(org, 1, undefined));
    }

    selectRepo(repo) {
        this.selectedRepo = repo;
        this.store.dispatch(fetchGitHubFiles(this.selectedOrg, this.selectedRepo, "plan."));
        window.scrollTo(0, 0);
    }

    selectUser(user) {
        this.selectedOrg = user;
        this.store.dispatch(fetchGitHubRepos(user, 1, user));
    }

    private handleSaved(successful, origin, name) {
        if (successful) {
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
}
