import appState from "../app-state"
import {Component} from "angular2/core";
import {Router} from "angular2/router";

@Component({
    template: `
    <h2>Sign In</h2>
    <form (ngSubmit)="onSubmit(usernameOrEmail, password)">
      <input placeholder="Username or email" required #usernameOrEmail>
      <input type="password" placeholder="Password" required #password>
      <button>Sign In</button>
    </form>
    `
})

export class SignInComponent {
  constructor(private router: Router) {}

  ngOnInit() {
    if (appState.get("signed-in")) {
      this.router.navigate(["Dashboard"])
    }
  }

  onSubmit(username) {
    if (!appState.get("username")) {
      appState.set("username", username.value);
    }
    appState.set("signed-in", true);
    this.router.navigate(["Dashboard"]);
  }
}
