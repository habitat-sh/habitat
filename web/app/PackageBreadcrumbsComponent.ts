import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["ident", "params"],
    selector: "package-breadcrumbs",
    template: `
    <span class="hab-package-breadcrumbs">
        <span *ngIf="showAll">All Packages</span>
        <span *ngIf="params.filter === 'mine'">My Packages</span>
        <a *ngIf="!ident.origin && !params.filter && !showAll" [routerLink]="['Packages']">*</a>
        <a [routerLink]="['PackagesForOrigin',
            { origin: ident.origin }]">
            {{ident.origin}}
        </a>
        <span *ngIf="ident.name">/</span>
        <a [routerLink]="['PackagesForOriginAndName',
            { origin: ident.origin,
                name: ident.name }]">
            {{ident.name}}
        </a>
        <span *ngIf="ident.version">/</span>
        <a [routerLink]="['PackagesForOriginAndNameAndVersion',
            { origin: ident.origin, name: ident.name,
                version: ident.version }]">
            {{ident.version}}
        </a>
        <span *ngIf="ident.release">/</span>
        <a [routerLink]="['Package',
            { origin: ident.origin, name: ident.name,
                version: ident.version, release: ident.release }]">
            {{ident.release}}
        </a>
    </span>`
})

export class PackageBreadcrumbsComponent {
    private ident;
    private params;

    constructor() {
        this.params = this.params || {};
    }

    get showAll() {
        return Object.keys(this.ident).length === 0;
    }
}