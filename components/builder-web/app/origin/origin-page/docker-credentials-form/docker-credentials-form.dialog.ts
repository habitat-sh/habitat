import { Component } from "@angular/core";
import { MdDialog, MdDialogRef } from "@angular/material";

export interface Credentials {
    username: string;
    password: string;
}

export class Credentials implements Credentials {
    username: string = "";
    password: string = "";
}

@Component({
    selector: "hab-docker-credentials-dialog",
    template: require("./docker-credentials-form.dialog.html")
})

export class DockerCredentialsFormDialog {
    model: Credentials = new Credentials;

    constructor(
        public dialogRef: MdDialogRef<DockerCredentialsFormDialog>) { }

    onNoClick(): void {
        this.dialogRef.close();
    }

    onSubmit() {
        this.dialogRef.close(this.model);
    }
}
