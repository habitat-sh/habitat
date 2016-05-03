// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
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
                *ngFor="#result of org.memberSearchResults; #i = index"
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
            <li *ngFor="#member of org.members; #i = index">
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
