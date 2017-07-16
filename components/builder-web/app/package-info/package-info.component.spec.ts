import { TestBed, ComponentFixture } from "@angular/core/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { MockComponent } from "ng2-mock-component";
import { Package } from "../records/Package";
import { PackageInfoComponent } from "./package-info.component";

describe("PackageInfoComponent", () => {
  let fixture: ComponentFixture<PackageInfoComponent>;
  let component: PackageInfoComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      declarations: [
        PackageInfoComponent,
        MockComponent({
          selector: "hab-package-list",
          inputs: [ "packages", "currentPackage" ]
        })
      ]
    });

    fixture = TestBed.createComponent(PackageInfoComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("when the package has one or more channels", () => {

    beforeEach(() => {
      component.package = Package({ channels: ["stable", "unstable"] });
      fixture.detectChanges();
    });

    it("renders them", () => {
      expect(element.query(By.css(".hab-package-channels"))).not.toBeNull();
    });
  });

  describe("when the package has no channels", () => {

    beforeEach(() => {
      component.package = Package({ channels: [] });
      fixture.detectChanges();
    });

    it("suppresses the channels section", () => {
      expect(element.query(By.css(".hab-package-channels"))).toBeNull();
    });
  });
});
