import {Component} from "angular2/core";

@Component({
    inputs: ["isSpinning", "onClick"],
    selector: "hab-spinner",
    template: `
    <span (click)="onClick()" [class.spinning]="isSpinning"
        class="hab-spinner"></span>
    `
})

export class SpinnerComponent { }