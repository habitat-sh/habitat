import appState from "../app-state"
import {Component} from "angular2/core";
import {Router} from "angular2/router";

@Component({
    template: "<h2>dashboard</h2>"
})

export class DashboardComponent {
  constructor(private router: Router) {}

  ngOnInit() {
    if (!appState.get("signed-in")) {
      this.router.navigate(["Home"])
    }
  }
}
