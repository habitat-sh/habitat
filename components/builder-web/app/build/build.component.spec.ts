import { TestBed, ComponentFixture } from "@angular/core/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { ActivatedRoute, Router } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { BehaviorSubject, Observable } from "rxjs";
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
            content: new BehaviorSubject([])
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
        { provide: AppStore, useClass: MockAppStore }
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

    beforeEach(() => {
      component.build = {
        origin: "core",
        name: "nginx",
        id: "123"
      };
    });
  });

  describe("on changes", () => {

    describe("when a build is provided", () => {
      let changes;

      beforeEach(() => {
        changes = {
          build: {
            currentValue: {
              id: "123"
            }
          }
        };
      });

      it("fetches the specified build log", () => {
        spyOn(actions, "fetchBuildLog");
        component.ngOnChanges(changes);

        expect(actions.fetchBuildLog).toHaveBeenCalledWith(
          store.getState().builds.selected.info.id,
          store.getState().gitHub.authToken,
          0
        );
      });

      describe("log streaming", () => {

        describe("by default", () => {

          it("is set to false", () => {
            spyOn(actions, "streamBuildLog");
            component.ngOnChanges(changes);

            expect(actions.streamBuildLog).toHaveBeenCalledWith(false);
          });
        });

        describe("when requested", () => {

          beforeEach(() => {
            component.stream = true;
          });

          it("is set to true", () => {
            spyOn(actions, "streamBuildLog");
            component.ngOnChanges(changes);

            expect(actions.streamBuildLog).toHaveBeenCalledWith(true);
          });
        });
      });
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
