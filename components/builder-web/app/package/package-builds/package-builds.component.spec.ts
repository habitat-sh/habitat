import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { Observable } from "rxjs";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../../AppStore";
import { Package } from "../../records/Package";
import * as actions from "../../actions/index";
import { PackageBuildsComponent } from "./package-builds.component";

class MockAppStore {

  getState() {
    return {
      packages: {
        current: Package()
      }
    };
  }

  dispatch() {}
}

class MockRoute {
  parent = {
    params: Observable.of({
      origin: "core",
      name: "nginx"
    })
  };
}

describe("PackageBuildsComponent", () => {
  let fixture: ComponentFixture<PackageBuildsComponent>;
  let component: PackageBuildsComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, "dispatch");
    spyOn(actions, "fetchBuilds");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageBuildsComponent,
        MockComponent({ selector: "hab-build-list", inputs: [ "builds" ]})
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageBuildsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    it("fetches the list of builds", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchBuilds).toHaveBeenCalledWith("core", "nginx", "");
    });
  });
});
