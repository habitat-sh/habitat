import { Component, Input, Output, EventEmitter, OnChanges } from "@angular/core";
import { FormControl } from "@angular/forms";

@Component({
  selector: "hab-docker-export-settings",
  template: require("./docker-export-settings.component.html")
})
export class DockerExportSettingsComponent implements OnChanges  {
  @Input() integrations: any;

  private enabled: boolean = false;
  private name: string;
  private repoName: string = "";
  private customTag: string;
  private latestTag: boolean = true;
  private versionTag: boolean = true;
  private releaseTag: boolean = true;

  get configured() {
    return this.integrations.size > 0;
  }

  get settings(): any {
    return {
      valid: this.valid,
      name: this.name,
      enabled: this.enabled,
      settings: {
        docker_hub_repo_name: this.repoName,
        custom_tag: this.customTag,
        latest_tag: this.latestTag,
        version_tag: this.versionTag,
        version_release_tag: this.releaseTag
      }
    };
  }

  get valid() {

    if (this.repoName.trim() !== "") {
      return true;
    }

    return false;
  }

  ngOnChanges(changes) {
    if (changes.integrations) {
      this.name = changes.integrations.currentValue.get(0);
    }
  }
}
