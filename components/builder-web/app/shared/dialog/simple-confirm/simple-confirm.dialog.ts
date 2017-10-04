import { Component, Inject } from "@angular/core";
import { MdDialogRef, MD_DIALOG_DATA } from "@angular/material";

@Component({
  template: require("./simple-confirm.dialog.html")
})
export class SimpleConfirmDialog {

  constructor(
    private ref: MdDialogRef<SimpleConfirmDialog>,
    @Inject(MD_DIALOG_DATA) private data: any
  ) {}

  get heading() {
    return this.data.heading || "Confirm";
  }

  get body() {
    return this.data.body || "Are you sure?";
  }

  get action() {
    return this.data.action || "do it";
  }

  ok() {
    this.ref.close(true);
  }

  cancel() {
    this.ref.close(false);
  }
}
