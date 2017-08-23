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
import { PackageVersionsComponent } from "./package-versions.component";

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

describe("PackageVersionsComponent", () => {
  let fixture: ComponentFixture<PackageVersionsComponent>;
  let component: PackageVersionsComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, "dispatch");
    spyOn(actions, "fetchBuilds");
    spyOn(actions, "fetchPackageVersions");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageVersionsComponent,
        MockComponent({ selector: "hab-icon", inputs: [ "symbol", "title" ]}),
        MockComponent({ selector: "hab-channels", inputs: [ "channels" ]})
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageVersionsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    it("fetches the list of versions", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchPackageVersions).toHaveBeenCalledWith("core", "nginx");
    });

    it("fetches the list of builds", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchBuilds).toHaveBeenCalledWith("core", "nginx", "");
    });
  });
});
