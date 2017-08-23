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
import { PackageLatestComponent } from "./package-latest.component";

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

describe("PackageLatestComponent", () => {
  let fixture: ComponentFixture<PackageLatestComponent>;
  let component: PackageLatestComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, "dispatch");
    spyOn(actions, "fetchLatestPackage");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageLatestComponent,
        MockComponent({ selector: "hab-package-detail", inputs: [ "package" ]})
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageLatestComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    it("fetches the latest package", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchLatestPackage).toHaveBeenCalledWith("core", "nginx");
    });
  });
});
