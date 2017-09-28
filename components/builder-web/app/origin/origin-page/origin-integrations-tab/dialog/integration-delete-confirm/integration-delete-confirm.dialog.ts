import { Component, Inject } from "@angular/core";
import { MdDialog, MdDialogRef, MD_DIALOG_DATA } from "@angular/material";

@Component({
  template: require("./integration-delete-confirm.dialog.html")
})
export class IntegrationDeleteConfirmDialog {
  constructor(private ref: MdDialogRef<IntegrationDeleteConfirmDialog>) {}

  ok() {
    this.ref.close(true);
  }

  cancel() {
    this.ref.close(false);
  }
}
