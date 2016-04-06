// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {attemptSignUp} from "../actions/index";

@Component({
    directives: [RouterLink],
    inputs: ["appName"],
    selector: "sign-up-form",
    template: `
    <div class="hab-sign-up-form">
        <div *ngIf="!isSubmitted">
            <h2>Get Started Now</h2>
            <h3>Create your {{appName}} account</h3>
            <form (ngSubmit)="onSubmit(username, email, password)">
                <input placeholder="Username" autofocus required #username>
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
                <a [routerLink]="['SignIn']">sign in page</a>.)
            </small></p>
        <div>
    </div>`,
})

export class SignUpFormComponent {
    constructor(private store: AppStore) { }

    onSubmit(username, email, password) {
        this.store.dispatch(attemptSignUp(
            username.value,
            email.value,
            password.value
        ));
    }

    get isSubmitted() {
        return this.store.getState().user.isSignUpFormSubmitted;
    }
}
