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
import { PackageReleaseComponent } from "./package-release.component";

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

  params = Observable.of({
    version: "1.11.10",
    release: "20170829004822"
  });
}

describe("PackageReleaseComponent", () => {
  let fixture: ComponentFixture<PackageReleaseComponent>;
  let component: PackageReleaseComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, "dispatch");
    spyOn(actions, "fetchPackage");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageReleaseComponent,
        MockComponent({ selector: "hab-package-detail", inputs: [ "package" ]})
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageReleaseComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin, name, version and release", () => {

    it("fetches the specified package", () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchPackage).toHaveBeenCalledWith({
        ident: {
          origin: "core",
          name: "nginx",
          version: "1.11.10",
          release: "20170829004822"
        }
      });
    });
  });
});
