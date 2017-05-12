import { TestBed, ComponentFixture } from "@angular/core/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { ActivatedRoute, Router } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { Observable } from "rxjs";
import { Record } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import * as actions from "../actions/index";
import * as util from "../util";
import { BuildComponent } from "./build.component";

class MockAppStore {
  getState() {
    return {
      builds: {
        selected: Record({
          info: {
            id: "123"
          },
          log: {
            content: []
          }
        })()
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
    name: "nginx",
    id: "123"
  });
}

describe("BuildComponent", () => {
  let fixture: ComponentFixture<BuildComponent>;
  let component: BuildComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {
    spyOn(util, "requireSignIn");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        BuildComponent,
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: [ "ident" ] })
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(BuildComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  it("requires sign-in", () => {
    expect(util.requireSignIn).toHaveBeenCalledWith(fixture.componentInstance);
  });

  describe("on init", () => {

    it("fetches the specified build", () => {
      spyOn(actions, "fetchBuild");
      fixture.detectChanges();

      expect(actions.fetchBuild).toHaveBeenCalledWith(
        store.getState().builds.selected.info.id,
        store.getState().gitHub.authToken
      );
    });

    it("fetches the specified build log", () => {
      spyOn(actions, "fetchBuildLog");
      fixture.detectChanges();

      expect(actions.fetchBuildLog).toHaveBeenCalledWith(
        store.getState().builds.selected.info.id,
        store.getState().gitHub.authToken,
        0
      );
    });

    it("initiates log streaming", () => {
      spyOn(actions, "streamBuildLog");
      fixture.detectChanges();

      expect(actions.streamBuildLog).toHaveBeenCalledWith(true);
    });
  });

  describe("on destroy", () => {

    it("terminates log streaming", () => {
      spyOn(actions, "streamBuildLog");
      component.ngOnDestroy();

      expect(actions.streamBuildLog).toHaveBeenCalledWith(false);
    });
  });

  xit("shows the selected build status", () => {

  });

  xit("shows the selected build log", () => {

  });
});
