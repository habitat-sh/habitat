import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { Observable } from "rxjs";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import { Package } from "../records/Package";
import * as actions from "../actions/index";
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
  params = Observable.of({
    origin: "core",
    name: "nginx"
  });
}

describe("PackageInfoComponent", () => {
  let fixture: ComponentFixture<PackageLatestComponent>;
  let component: PackageLatestComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageLatestComponent,
        MockComponent({ selector: "hab-icon", inputs: [ "symbol", "title" ]}),
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: [ "ident" ]}),
        MockComponent({ selector: "hab-package-info", inputs: [ "package" ]}),
        MockComponent({ selector: "hab-tab", inputs: [ "tabTitle" ]}),
        MockComponent({ selector: "hab-tabs"})
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageLatestComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    it("fetches the latest package", () => {
      store = TestBed.get(AppStore);

      spyOn(store, "dispatch");
      spyOn(actions, "fetchLatestPackage");
      fixture.detectChanges();

      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchLatestPackage).toHaveBeenCalledWith("core", "nginx");
    });
  });
});
