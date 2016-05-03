// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {deleteOrigin, setCurrentOrigin} from "../actions/index";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-origins">
        <div class="page-title">
            <h2>
                Origins
            </h2>
            <a class="button create"
               [routerLink]="['OriginCreate']">Add Origin</a>
        </div>
        <div class="page-body">
            <div *ngIf="origins.size === 0">
                <div class="hero">
                    <h3>You don't currently have any origins, let's add one now.</h3>
                    <p>
                        <a class="button cta" [routerLink]='["OriginCreate"]'>
                            Add Origin
                        </a>
                    </p>
                </div>
            </div>
            <div *ngIf="origins.size > 0">
                <ul class="hab-origins-list">
                    <li *ngFor="#origin of origins">
                        <a class="hab-item-list hab-no-select"
                        [class.active]="origin.name === currentOrigin.name">
                            <div class="hab-item-list--title">
                                <h3>{{origin.name}}</h3>
                            </div>
                            <div class="button hab-item-list--controls">
                                <button
                                class="confirm hab-origins-list--default"
                                (click)="setCurrentOrigin(origin)"
                                [disabled]="origin.name === currentOrigin.name">
                                    <span *ngIf="origin.name === currentOrigin.name">
                                        <i class="octicon octicon-star"></i> Default
                                    </span>
                                    <span *ngIf="origin.name !== currentOrigin.name">
                                        Set as Default
                                    </span>
                            </button>
                            <button
                                class="danger hab-origins-list--delete"
                                (click)="deleteOrigin(origin)"
                                [disabled]="origin.name === currentOrigin.name">
                                <i class="octicon octicon-trashcan"></i> Delete
                            </button>
                            </div>
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    </div>`,
})

export class OriginsPageComponent implements OnInit {
    constructor(private store: AppStore, private routeParams: RouteParams) { }

    public ngOnInit() {
        requireSignIn(this);
    }

    get currentOrigin() { return this.store.getState().origins.current; }

    get origins() { return this.store.getState().origins.mine; }

    private deleteOrigin(origin) {
        if (prompt(`Deleting an origin deletes all of its packages.
 Type the name of the origin to confirm`) === origin.name) {
            this.store.dispatch(deleteOrigin(origin));
        }
        return false;
    }

    private setCurrentOrigin(origin) {
        this.store.dispatch(setCurrentOrigin(origin));
        return false;
    }
}
