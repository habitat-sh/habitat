import { Component, Inject } from "@angular/core";
import { MdDialogRef } from "@angular/material";

@Component({
  template: require("./disconnect-confirm.dialog.html")
})
export class DisconnectConfirmDialog {
  constructor(private ref: MdDialogRef<DisconnectConfirmDialog>) {}

  ok() {
      this.ref.close(true);
  }

  cancel() {
      this.ref.close(false);
  }
}
