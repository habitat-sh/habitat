import { Component, EventEmitter, Input, OnInit, OnChanges, Output, SimpleChanges, ViewChild } from "@angular/core";
import { FormBuilder, FormGroup, Validators } from "@angular/forms";
import { MdDialog, MdDialogRef } from "@angular/material";
import { DisconnectConfirmDialog } from "./dialog/disconnect-confirm/disconnect-confirm.dialog";
import { DockerExportSettingsComponent } from "../../shared/docker-export-settings/docker-export-settings.component";
import { BuilderApiClient } from "../../BuilderApiClient";
import { GitHubApiClient } from "../../GitHubApiClient";
import { GitHubRepo } from "../../github/repo/shared/github-repo.model";
import { AppStore } from "../../AppStore";
import { addProject, updateProject, setProjectIntegrationSettings, deleteProject, fetchGitHubFiles,
    fetchGitHubInstallations, fetchGitHubInstallationRepositories, fetchProject, setProjectVisibility } from "../../actions/index";
import config from "../../config";

@Component({
    selector: "hab-project-settings",
    template: require("./project-settings.component.html")
})
export class ProjectSettingsComponent implements OnChanges {
    connecting: boolean = false;
    doesFileExist: Function;
    filter: GitHubRepo = new GitHubRepo();
    form: FormGroup;
    selectedInstallation: any;
    selectedRepo: any;
    selectedPath: string;

    @Input() integrations;
    @Input() name: string;
    @Input() origin: string;
    @Input() project: any;

    @Output() saved: EventEmitter<any> = new EventEmitter<any>();
    @Output() toggled: EventEmitter<any> = new EventEmitter<any>();

    @ViewChild("docker")
    docker: DockerExportSettingsComponent;

    private api: BuilderApiClient;
    private defaultPath = "habitat/plan.sh";
    private _visibility: string;

    constructor(private formBuilder: FormBuilder, private store: AppStore, private disconnectDialog: MdDialog) {
        this.api = new BuilderApiClient(this.token);
        this.form = formBuilder.group({});
        this.selectedPath = this.defaultPath;

        this.doesFileExist = (path) => {
            return this.api.findFileInRepo(
                this.selectedInstallation.get("id"),
                this.selectedRepo.getIn(["owner", "login"]),
                this.selectedRepo.get("name"),
                this.planField.value
            );
        };
    }

    ngOnChanges(changes: SimpleChanges) {
        const p = changes["project"];

        if (p && p.currentValue) {
            this.visibility = p.currentValue.visibility || this.visibility;
        }
    }

    get config() {
        return config;
    }

    get planField() {
        return this.form.controls["plan_path"];
    }

    get connectButtonLabel() {
        return this.project ? "Update" : "Save";
    }

    get files() {
        return this.store.getState().gitHub.files;
    }

    get installations() {
        return this.store.getState().gitHub.installations;
    }

    get orgs() {
        return this.store.getState().gitHub.orgs;
    }

    get planTemplate() {
        return {
            "origin": this.origin,
            "plan_path": this.selectedPath,
            "installation_id": this.selectedInstallation.get("id"),
            "repo_id": this.selectedRepo.get("id")
        };
    }

    get projectsEnabled() {
        return !!this.store.getState().featureFlags.current.get("project");
    }

    get repos() {
        return this.store.getState().gitHub.installationRepositories;
    }

    get repoUrl() {
        if (this.selectedRepo) {
            return `https://github.com/${this.selectedRepo.getIn(["owner", "login"])}/${this.selectedRepo.get("name")}`;
        }
    }

    get token() {
        return this.store.getState().session.token;
    }

    get username() {
        return this.store.getState().users.current.gitHub.get("login");
    }

    get valid() {
        const dockerValid = (this.docker && this.docker.settings.enabled) ? this.docker.settings.valid : true;
        const planPathValid = this.form.valid;
        return this.selectedRepo && dockerValid && planPathValid;
    }

    get visibility() {
        return this._visibility || this.store.getState().origins.current.default_package_visibility || "public";
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
        this.selectedInstallation = null;
        this.selectedRepo = null;
        this.selectedPath = this.defaultPath;
        this.filter = new GitHubRepo();
    }

    editConnection() {
        this.clearConnection();
        this.connect();
        this.selectedPath = this.project.plan_path;
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

    selectInstallation(installation) {
        this.selectedInstallation = installation;
        this.store.dispatch(fetchGitHubInstallationRepositories(installation.get("id")));
    }

    selectRepo(repo) {
        this.selectedRepo = repo;
        this.selectedPath = this.defaultPath;
        window.scrollTo(0, 0);

        setTimeout(() => {
            this.planField.markAsDirty();
        }, 1000);
    }

    settingChanged(setting) {
        this.visibility = setting;
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
