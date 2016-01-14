import {Component} from "angular2/core";
import {SignUpFormComponent} from "./sign-up-form.component";

@Component({
  directives: [SignUpFormComponent],
  template: `
    <div>
      <h2>Applications done correctly</h2>
      <h3>Build, deploy, and run your applications well.</h3>
      <h4>For the cloud and the data center.</h4>
    </div>
    <div>
      <sign-up-form></sign-up-form>
    </div>
  `,
})

export class HomeComponent { }
