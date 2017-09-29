import { Component, Inject } from "@angular/core";
import { MdDialog, MdDialogRef, MD_DIALOG_DATA } from "@angular/material";

@Component({
  template: require("./generate-keys-confirm.dialog.html")
})
export class GenerateKeysConfirmDialog {
  constructor(private ref: MdDialogRef<GenerateKeysConfirmDialog>) {}

  ok() {
    this.ref.close(true);
  }

  cancel() {
    this.ref.close(false);
  }
}
