import { Component } from "@angular/core";
import { MdDialog, MdDialogRef } from "@angular/material";

@Component({
  selector: "hab-docker-credentials-dialog",
  template: require("./docker-credentials-form.dialog.html")
})
export class DockerCredentialsFormDialog {
    constructor(
        public dialogRef: MdDialogRef<DockerCredentialsFormDialog>) { }

    onNoClick(): void {
        this.dialogRef.close();
    }
}
