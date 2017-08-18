import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { ReactiveFormsModule } from "@angular/forms";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { Observable } from "rxjs";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import * as actions from "../actions/index";
import { AppStore } from "../AppStore";
import { PackagesPageComponent } from "./packages-page.component";

class MockAppStore {
  static state;

  getState() {
    return MockAppStore.state;
  }

  dispatch() {}
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
  let store: AppStore;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        ReactiveFormsModule,
        RouterTestingModule
      ],
      declarations: [
        MockComponent({ selector: "hab-package-breadcrumbs", inputs: ["ident"] }),
        MockComponent({ selector: "hab-icon", inputs: [ "symbol" ] }),
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
    store = TestBed.get(AppStore);
  });

  beforeEach(() => {
    MockAppStore.state = {
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
      },
      origins: {
        mine: List()
      }
    };
  });

  describe("given the core origin", () => {

    beforeEach(() => {
      component.origin = "core";
      fixture.detectChanges();
    });

    it("shows the Search Packages heading", () => {
      let heading = element.query(By.css(".hab-packages h2"));
      expect(heading.nativeElement.textContent).toBe("Search Packages");
    });

    describe("and a package name", () => {

      beforeEach(() => {
        component.name = "linux-headers";
      });

      describe("when the user belongs to the core origin", () => {

        beforeEach(() => {
          MockAppStore.state.origins.mine = List([ { name: "core" } ]);
          MockAppStore.state.gitHub.authToken = "some-token";
        });

        it("shows the build-request button", () => {
          fixture.detectChanges();
          expect(element.query(By.css(".page-body--sidebar button"))).not.toBeNull();
        });
      });

      describe("when the user does not belong to the core origin", () => {

        beforeEach(() => {
          MockAppStore.state.origins.mine = List([ { name: "so-not-core" } ]);
          MockAppStore.state.gitHub.authToken = "some-token";
        });

        it("does not show the build-request button", () => {
          fixture.detectChanges();
          expect(element.query(By.css(".page-body--sidebar button"))).toBeNull();
        });

        it("shows a link to the build-history list", () => {
          fixture.detectChanges();
          let link = element.query(By.css(".build-history a")).nativeElement;

          expect(link.getAttribute("href")).toBe("/pkgs/core/linux-headers/builds");
          expect(link.textContent).toContain("View full build history");
        });
      });

      describe("when the user is not signed in", () => {

        beforeEach(() => {
          MockAppStore.state.origins.mine = List();
          MockAppStore.state.gitHub.authToken = undefined;
        });

        it("does not show a link to the build-history list", () => {
          fixture.detectChanges();
          let link = element.query(By.css(".build-history a"));

          expect(link).toBeNull();
        });
      });
    });
  });

  describe("search", () => {

    describe("given a query", () => {

      beforeEach(() => {
        let query = "foo";
        component.query = query;
        MockAppStore.state.packages.searchQuery = query;
        fixture.detectChanges();
      });

      it("shows the Search Packages heading", () => {
        let heading = element.query(By.css(".hab-packages h2"));
        expect(heading.nativeElement.textContent).toBe("Search Packages");
      });

      it("shows the search box", () => {
        expect(element.query(By.css(".page-body input[type='search']"))).not.toBeNull();
      });

      describe("fetch", () => {

        it ("clears the list of builds", () => {
          spyOn(actions, "clearBuilds");

          component.fetch();

          expect(actions.clearBuilds).toHaveBeenCalled();
        });

        it ("fetches with the distinct parameter", () => {
          spyOn(actions, "filterPackagesBy");

          component.fetch();

          expect(actions.filterPackagesBy).toHaveBeenCalledWith(
            { name: undefined, origin: undefined, version: undefined }, "foo", true
          );
        });
      });
    });
  });

  describe("on destroy", () => {

    it("clears the list of visible builds", () => {
      spyOn(actions, "clearBuilds");
      component.ngOnDestroy();
      expect(actions.clearBuilds).toHaveBeenCalled();
    });
  });
});
