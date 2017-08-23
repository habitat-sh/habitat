import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { Observable } from "rxjs";
import { MockComponent } from "ng2-mock-component";
import { Package } from "../../records/Package";
import * as actions from "../../actions/index";
import { PackageDetailComponent } from "./package-detail.component";

class MockRoute {
  params = Observable.of({
    origin: "core",
    name: "nginx"
  });
}

describe("PackageDetailComponent", () => {
  let fixture: ComponentFixture<PackageDetailComponent>;
  let component: PackageDetailComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      declarations: [
        PackageDetailComponent,
        MockComponent({ selector: "hab-icon", inputs: [ "symbol" ]}),
        MockComponent({ selector: "hab-channels", inputs: [ "channels" ]}),
        MockComponent({ selector: "hab-package-list", inputs: [ "currentPackage", "packages" ]}),
      ]
    });

    fixture = TestBed.createComponent(PackageDetailComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given a package", () => {

    beforeEach(() => {

      component.package = Package({
        ident: {
          origin: "core",
          name: "nginx",
          version: "1.11.10",
          release: "20170829004822"
        },
        checksum: "some-checksum",
        channels: [ "unstable", "stable" ]
      });
    });

    it("renders it", () => {
      fixture.detectChanges();

      function textOf(selector) {
        return element.query(By.css(`.hab-package-detail ${selector}`)).nativeElement.textContent;
      }

      expect(textOf("h2")).toContain("core/nginx");
      expect(textOf(".metadata")).toContain("1.11.10");
      expect(textOf(".metadata")).toContain("20170829004822");
      expect(textOf(".metadata")).toContain("some-checksum");
    });
  });
});
