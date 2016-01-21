import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {attemptSignUp} from "../actions";

@Component({
  directives: [RouterLink],
  selector: "sign-up-form",
  template: `
    <div class="bldr-sign-up-form">
      <div *ngIf="!isSubmitted">
        <h2>Get Started Now</h2>
        <h3>Create your bldr account</h3>
        <form (ngSubmit)="onSubmit(username, email, password)">
          <input placeholder="Username" required #username>
          <input type="email" placeholder="Email Address" required #email>
          <input type="password" placeholder="Password" required #password>
          <button>Sign Up</button>
        </form>
      </div>
      <div *ngIf="isSubmitted">
        <h3>You're almost ready to go!</h3>
        <p>Check your email to activate your account.</p>
        <p><small>
          (ok now pretend you clicked the link in that email and it took you to the
          <a [routerLink]="['Sign In']">sign in page</a>.)
        </small></p>
      <div>
    </div>
  `,
})

export class SignUpFormComponent {
  constructor(private store: AppStore) {}

  onSubmit(username, email, password) {
    this.store.dispatch(attemptSignUp(
      username.value,
      email.value,
      password.value
    ));
  }

  get isSubmitted() {
    return this.store.getState().isSignUpFormSubmitted;
  }
}
