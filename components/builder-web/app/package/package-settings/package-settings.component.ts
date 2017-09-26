import { Component, Inject, OnInit, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DisconnectConfirmDialog } from "../dialog/disconnect-confirm/disconnect-confirm.dialog";
import { Subscription } from "rxjs/subscription";
import { GitHubApiClient } from "../../GitHubApiClient";
import { GitHubRepo } from "../../github/repo/shared/github-repo.model";
import { requireSignIn } from "../../util";
import { AppStore } from "../../AppStore";
import { addNotification, addProject, updateProject, deleteProject, fetchGitHubFiles, fetchGitHubOrgs,
    fetchGitHubRepos, fetchProject, clearGitHubRepos } from "../../actions/index";
import config from "../../config";

@Component({
    template: require("./package-settings.component.html")
})
export class PackageSettingsComponent implements OnInit, OnDestroy {
    connecting: boolean = false;
    filter: GitHubRepo = new GitHubRepo();
    name: string;
    origin: string;
    selectedOrg: string;
    selectedRepo: string;
    selectedPath: string;

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
        this.clearConnection();

        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    get config() {
        return config;
    }

    get connectButtonLabel() {
        return this.connectedPlan ? "Update" : "Save";
    }

    get connectedPlan() {
        const project = this.store.getState().projects.current;

        const isMember = !!this.store.getState().origins.mine.find((o) => {
            return o.name === this.origin;
        });

        if (isMember && project.ui.exists) {
            return project;
        }
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

    connect() {
        this.store.dispatch(fetchGitHubOrgs());
        this.connecting = true;
    }

    disconnect() {
        const ref = this.disconnectDialog.open(DisconnectConfirmDialog, {
            width: "460px"
        });

        ref.afterClosed().subscribe((confirmed) => {
            if (confirmed) {
                this.store.dispatch(deleteProject(this.connectedPlan.name, this.token));
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
        this.selectedPath = this.connectedPlan.plan_path;
    }

    saveConnection() {
        new GitHubApiClient(this.store.getState().gitHub.authToken)
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

                if (planVars["pkg_name"] === this.name && planVars["pkg_origin"] === this.origin) {
                    if (this.connectedPlan) {
                        this.store.dispatch(updateProject(this.connectedPlan.name, this.planTemplate, this.token, (result) => {
                            this.saved();
                        }));
                    }
                    else {
                        this.store.dispatch(addProject(this.planTemplate, this.token, (result) => {
                            this.saved();
                        }));
                    }
                }
                else {
                    this.store.dispatch(addNotification({
                        type: "danger",
                        title: "Invalid Selection",
                        body: `The origin and name in this plan file (${planVars["pkg_origin"]}/${planVars["pkg_name"]})
                            must match those of this package (${this.origin}/${this.name}).`
                    }));
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

    saved() {
        this.clearConnection();
        this.store.dispatch(fetchProject(`${this.origin}/${this.name}`, this.token, false));
        window.scroll(0, 0);
    }

    selectOrg(org) {
        this.selectedOrg = org;
        this.store.dispatch(fetchGitHubRepos(org, 1, undefined));
    }

    selectRepo(repo) {
        this.selectedRepo = repo;
        this.store.dispatch(fetchGitHubFiles(this.selectedOrg, this.selectedRepo, "plan."));
    }

    selectUser(user) {
        this.selectedOrg = user;
        this.store.dispatch(fetchGitHubRepos(user, 1, user));
    }
}
