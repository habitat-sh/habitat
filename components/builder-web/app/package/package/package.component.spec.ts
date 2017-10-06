import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { MdTabsModule } from "@angular/material";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { Observable } from "rxjs";
import { MockComponent } from "ng2-mock-component";
import * as actions from "../../actions/index";
import { AppStore } from "../../AppStore";
import { PackageComponent } from "./package.component";

class MockAppStore {
  dispatch() {}

  getState() {
    return {
      session: {
        token: "some-token"
      }
    };
  }
}

class MockRoute {
  params = Observable.of({
    origin: "core",
    name: "nginx"
  });
}

describe("PackageComponent", () => {
  let fixture: ComponentFixture<PackageComponent>;
  let component: PackageComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule,
        MdTabsModule
      ],
      declarations: [
        PackageComponent,
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: [ "ident" ]}),
        MockComponent({ selector: "hab-package-sidebar", inputs: [ "origin", "name", "building", "buildable" ]}),
        MockComponent({ selector: "hab-build-notice", inputs: [ "build" ] })
      ],
      providers: [
        { provide: ActivatedRoute, useClass: MockRoute },
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(PackageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    it("renders breadcrumbs and sidebar", () => {
      expect(element.query(By.css("hab-package-breadcrumbs"))).not.toBeNull();
      expect(element.query(By.css("hab-package-sidebar"))).not.toBeNull();
    });
  });
});
