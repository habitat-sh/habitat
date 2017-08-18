import { Component, DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { By } from "@angular/platform-browser";
import { ActivatedRoute, Router } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { Observable } from "rxjs";
import { List, Record } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import * as util from "../util";
import * as actions from "../actions/index";
import { PackageBuildsComponent } from "./package-builds.component";

class MockAppStore {
  getState() {
    return {
      builds: {
        visible: List()
      },
      gitHub: {
        authToken: "some-token"
      }
    };
  }
  dispatch() {}
}

class MockRoute {
  params = Observable.of({
    origin: "core",
    name: "nginx"
  });
}

describe("PackageBuildsComponent", () => {
  let component: PackageBuildsComponent;
  let fixture: ComponentFixture<PackageBuildsComponent>;
  let element: DebugElement;
  let router: Router;

  beforeEach(() => {
    spyOn(util, "requireSignIn");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageBuildsComponent,
        MockComponent({ selector: "hab-build-list", inputs: [ "builds" ] }),
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: [ "ident" ] })
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageBuildsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    router = TestBed.get(Router);
  });

  it("dispatches a request for builds by origin and name", () => {
    let store = TestBed.get(AppStore);

    spyOn(store, "dispatch");
    spyOn(actions, "fetchBuilds");
    fixture.detectChanges();

    expect(store.dispatch).toHaveBeenCalled();
    expect(actions.fetchBuilds).toHaveBeenCalledWith("core", "nginx", "some-token");
  });

  it("includes a build-list component", () => {
    let el = element.query(By.css("hab-build-list"));
    fixture.detectChanges();

    expect(el.nativeElement.getAttribute("ng-reflect-builds")).toBeTruthy();
  });

  describe("when a build is selected", () => {

    beforeEach(() => {
      spyOn(router, "navigate");
      component.onSelect({ id: "123" });
    });

    it("navigates to it", () => {
      expect(router.navigate).toHaveBeenCalledWith(["builds", "123"]);
    });
  });
});
