import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { PackagesListComponent } from "./packages-list.component";

describe("PackagesListComponent", () => {
  let fixture: ComponentFixture<PackagesListComponent>;
  let component: PackagesListComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackagesListComponent,
        MockComponent({ selector: "hab-icon", inputs: [ "symbol" ]}),
        MockComponent({ selector: "hab-channels", inputs: [ "channels" ]}),
        MockComponent({
          selector: "hab-build-status",
          inputs: [ "origin", "name", "version" ]
        })
      ]
    });

    fixture = TestBed.createComponent(PackagesListComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  beforeEach(() => {
    component.packages = List([
      {
        origin: "core",
        project: "nginx",
        version: "1.0.2",
        release: "20170101000002",
        channels: [ "stable", "unstable" ]
      },
      {
        origin: "core",
        project: "nginx",
        version: "1.0.1",
        release: "20170101000001",
        channels: [ "unstable" ]
      },
      {
        origin: "core",
        project: "nginx",
        version: "1.0.0",
        release: "20170101000000",
        channels: []
      }
    ]);
    fixture.detectChanges();
  });

  it("renders a list of packages", () => {
    expect(element.queryAll(By.css(".hab-packages-package .name")).length).toBe(3);
  });
});
