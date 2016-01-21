import {Component} from "angular2/core";
import {Router} from "angular2/router";
import {AppStore} from "../AppStore";
import {attemptSignIn} from "../actions";

@Component({
    template: `
    <div class="bldr-sign-in">
      <h2>Sign In</h2>
      <form (ngSubmit)="onSubmit(usernameOrEmail, password)">
        <input placeholder="Username or email" required #usernameOrEmail>
        <input type="password" placeholder="Password" required #password>
        <button>Sign In</button>
      </form>
    </div>
    `
})

export class SignInComponent {
  constructor(private router: Router, private store: AppStore) {}

  ngOnInit() {
    if (this.store.getState().isSignedIn) {
      this.router.navigate(["Dashboard"])
    }
  }

  onSubmit(username) {
    this.store.dispatch(attemptSignIn(username.value));
    this.router.navigate(["Dashboard"]);
  }
}
