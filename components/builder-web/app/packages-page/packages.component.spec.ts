import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { ReactiveFormsModule } from "@angular/forms";
import { By } from "@angular/platform-browser";
import { RouterTestingModule } from "@angular/router/testing";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import { PackagesPageComponent } from "./PackagesPageComponent";

class MockAppStore {
  getState() {
    return {
      packages: []
    };
  }
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
          inputs: ["noPackages", "packages", "errorMessage"]
        }),
        PackagesPageComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(PackagesPageComponent);
    element = fixture.debugElement;
  });

  it("shows all packages", () => {
    let heading = element.query(By.css(".hab-packages h2"));
    expect(heading.nativeElement.textContent).toBe("Search Packages");
  });
});
