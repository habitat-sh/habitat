import appState from "../app-state";
import {Component} from "angular2/core";
import {Router} from "angular2/router";
import {SignUpFormComponent} from "../sign-up-form/sign-up-form.component";

@Component({
  directives: [SignUpFormComponent],
  template: `
    <div class="bldr-hero">
      <div class="bldr-home">
        <h2>Applications done correctly</h2>
        <h3>Build, deploy, and run your applications well.</h3>
        <h4>For containers, for the cloud, for the data center.</h4>
      </div>
      <div class="bldr-sign-up-form">
        <sign-up-form></sign-up-form>
      </div>
    </div>
  `,
})

export class HomeComponent {
  constructor(private router: Router) {}

  ngOnInit() {
    if (appState.get("signed-in")) {
      this.router.navigate(["Dashboard"])
    }
  }
}
