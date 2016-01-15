import appState from "./app-state";
import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
  directives: [RouterLink],
  selector: "sign-up-form",
  template: `
    <div *ngIf="!isSubmitted">
      <h2>Get Started Now</h2>
      <h3>Create your bldr account</h3>
      <form (ngSubmit)="onSubmit()">
        <input placeholder="Username" required>
        <input type="email" placeholder="Email Address" required>
        <input type="password" placeholder="Password" required>
        <button>Sign Up</button>
      </form>
    </div>
    <div *ngIf="isSubmitted">
      <h3>You're almost ready to go!</h3>
      <p>Check your email to activate your account.</p>
      <p><small>
        (ok now pretend you clicked the link in that email and it took you to the
        <a [routerLink]="['Dashboard']">dashboard</a>.)
      </small></p>
    <div>
  `,
})

export class SignUpFormComponent {
  onSubmit() {
    appState.set("sign-up-form-submitted", true);
  }

  get isSubmitted() {
    return appState.get("sign-up-form-submitted");
  }
}
