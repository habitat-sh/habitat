import { Location } from "@angular/common";
import { TestBed, fakeAsync, tick } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { MdButtonModule } from "@angular/material";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { Router } from "@angular/router";
import { imports, declarations } from "./origin.module";

imports.push(RouterTestingModule);

describe("Router: Origin", () => {
  let location: Location;
  let router: Router;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports, declarations
    });
    router = TestBed.get(Router);
    location = TestBed.get(Location);
    router.initialNavigation();
  });

  it("navigate to origins/create takes you to create page", () => {
    router.navigate(["/origins/create"]).then(() => {
      expect(location.path()).toBe("/origins/create");
    });
  });

  it("navigate to origins/create takes you to the correct place", () => {
    router.navigate(["/origins/create"]).then(() => {
      expect(location.path()).toBe("/origins/create");
    });
  });
});
