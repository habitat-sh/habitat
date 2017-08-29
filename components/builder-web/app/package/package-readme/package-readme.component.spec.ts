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
import { PackageReadmeComponent } from "./package-readme.component";

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

describe("PackageReadmeComponent", () => {
  let fixture: ComponentFixture<PackageReadmeComponent>;
  let component: PackageReadmeComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageReadmeComponent
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageReadmeComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {

    xit("fetches the latest readme", () => {

    });
  });
});
