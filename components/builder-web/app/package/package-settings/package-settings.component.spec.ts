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
import { PackageSettingsComponent } from "./package-settings.component";

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

describe("PackageSettingsComponent", () => {
  let fixture: ComponentFixture<PackageSettingsComponent>;
  let component: PackageSettingsComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageSettingsComponent
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageSettingsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    xit("fetches current project settings", () => {

    });
  });
});
