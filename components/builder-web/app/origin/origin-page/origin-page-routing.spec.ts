import { Location } from "@angular/common";
import { TestBed } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { MdButtonModule } from "@angular/material";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { Router } from "@angular/router";
import { imports, declarations } from "./origin-page.module";

imports.push(RouterTestingModule);

describe("Router: Origin Page", () => {
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

  it("navigate to origins/:origin redirects you to packages", () => {
    router.navigate(["/origins/core"]).then(() => {
      expect(location.path()).toBe("/origins/core/packages");
    });
  });

  it("navigate to origins/:origin/garbage redirects you to packages", () => {
    router.navigate(["/origins/core/garbage"]).then(() => {
      expect(location.path()).toBe("/origins/core/packages");
    });
  });
});
