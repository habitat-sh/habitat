import {Component} from "angular2/core";
import {Router} from "angular2/router";
import {SignUpFormComponent} from "../sign-up-form/SignUpFormComponent";
import {AppStore} from "../AppStore";

@Component({
  directives: [SignUpFormComponent],
  template: `
    <div class="bldr-hero">
      <div class="bldr-home">
        <h2>Applications done correctly</h2>
        <h3>Build, deploy, and run your applications well.</h3>
        <h4>For containers, for the cloud, for the data center.</h4>
      </div>
      <sign-up-form></sign-up-form>
    </div>
  `,
})

export class HomeComponent {
  constructor(private router: Router, private store: AppStore) {}

  ngOnInit() {
    if (this.store.getState().isSignedIn) {
      this.router.navigate(["Dashboard"])
    }
  }
}
