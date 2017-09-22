import { Component } from "@angular/core";
import { MdDialog, MdDialogRef } from "@angular/material";

@Component({
  selector: "hab-generate-keys-dialog",
  template: require("./generate-keys.dialog.html")
})

export class GenerateKeysDialog {

  constructor(
    public dialogRef: MdDialogRef<GenerateKeysDialog>) { }

  onNoClick(): void {
    this.dialogRef.close();
  }
}
