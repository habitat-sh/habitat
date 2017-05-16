import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { ReactiveFormsModule } from "@angular/forms";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { Observable } from "rxjs";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import { PackagesPageComponent } from "./PackagesPageComponent";

class MockAppStore {
  getState() {
    return {
      builds: {
        visible: List()
      },
      packages: {
        visible: List(),
        ui: {
          visible: {}
        }
      },
      gitHub: {
        authToken: undefined
      }
    };
  }
}

class MockRoute {
  get params() {
    return {
      subscribe: () => {}
    };
  };
}

describe("PackagesPageComponent", () => {
  let fixture: ComponentFixture<PackagesPageComponent>;
  let component: PackagesPageComponent;
  let element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        ReactiveFormsModule,
        RouterTestingModule
      ],
      declarations: [
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: ["ident"] }),
        MockComponent({ selector: "hab-spinner", inputs: ["isSpinning"] }),
        MockComponent({
          selector: "hab-packages-list",
          inputs: [ "errorMessage", "noPackages", "layout", "packages", "versions" ]
        }),
        PackagesPageComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackagesPageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given only an origin", () => {

    beforeEach(() => {
      component.origin = "core";
      fixture.detectChanges();
    });

    it("shows the Search Packages heading", () => {
      let heading = element.query(By.css(".hab-packages h2"));
      expect(heading.nativeElement.textContent).toBe("Search Packages");
    });
  });
});
