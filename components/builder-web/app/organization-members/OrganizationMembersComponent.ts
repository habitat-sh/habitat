// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import {Component} from "@angular/core";
import {GravatarComponent} from "../GravatarComponent";
import {icon} from "../util";

@Component({
    directives: [GravatarComponent],
    selector: "hab-org-members",
    inputs: ["cancelInvitation", "inviteMemberToOrg", "org", "performSearch",
        "searchResults", "toggleMemberActionMenu"],
    template: `
    <div class="hab-org-members">
        <h4>Search by username, full name, or email address</h4>
        <input (search)="searchKeyup(q.value)" (keyup)="searchKeyup(q.value)" type=search #q>
        <ul class="results" *ngIf="org.memberSearchResults.size > 0">
            <li (click)="addClick(result, i)"
                *ngFor="let result of org.memberSearchResults; let i = index"
                [class.addable]="result.canBeAdded">
                <span class="grav">
                    <gravatar size=16 [email]="result.email"></gravatar>
                </span>
                <p class="info">
                    <span class="username">{{result.username}}</span>
                    <span class="resultName">{{result.name}}</span>
                    <span class="status">{{result.status}}&nbsp;</span>
                </p>
                <span class="indicator">
                    <img height=16 *ngIf="result.canBeAdded" src='{{icon("plus")}}'>
                    <img height=16 *ngIf="!result.canBeAdded" src='{{icon("check")}}'>
                </span>
            </li>
        </ul>
        <ul class="members">
            <li *ngFor="let member of org.members; let i = index">
                <gravatar size=16 [email]="member.email"></gravatar>
                <span class="username">{{member.username}}</span>
                <span class="name">{{member.name}}</span>
                <span class="status">{{member.status}}</span>
                <span class="actions">
                    <a (click)="actionClick(i)" class="actionsButton" href="#">
                        <img src='{{icon("gear")}}'>
                        <img *ngIf="!isMemberActionMenuOpenAt(i)" src='{{icon("triangle-down")}}'>
                        <img *ngIf="isMemberActionMenuOpenAt(i)" src='{{icon("triangle-up")}}'>
                    </a>
                    <ul *ngIf="isMemberActionMenuOpenAt(i)" class="actionsMenu actions--menu">
                        <li>
                            <a (click)="cancelInvitationClick(i)" href="#">
                                Cancel invitation
                            </a>
                        </li>
                    </ul>
                </span>
            </li>
        </ul>
    </div>`
})

export class OrganizationMembersComponent {
    private cancelInvitation: Function;
    private inviteMemberToOrg: Function;
    private org;
    private performSearch: Function;
    private toggleMemberActionMenu: Function;

    private actionClick(index: number): boolean {
        this.toggleMemberActionMenu(index);
        return false;
    }

    private addClick(result, index): boolean {
        if (result.canBeAdded) {
            this.inviteMemberToOrg(result, index);
        }
        return false;
    }

    private cancelInvitationClick(index): boolean {
        this.cancelInvitation(index);
        return false;
    }

    private icon(x) { return icon(x); }

    private isMemberActionMenuOpenAt(index) {
        return this.org.members.get(index).ui.isActionsMenuOpen;
    }

    private searchKeyup(q: string): boolean {
        this.performSearch(q);
        return false;
    }
}
