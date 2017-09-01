import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { ReactiveFormsModule } from "@angular/forms";
import { By } from "@angular/platform-browser";
import { ActivatedRoute } from "@angular/router";
import { RouterTestingModule } from "@angular/router/testing";
import { Observable } from "rxjs";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import * as actions from "../../actions/index";
import { AppStore } from "../../AppStore";
import { SearchComponent } from "./search.component";

class MockAppStore {
  static state;

  getState() {
    return MockAppStore.state;
  }

  dispatch() {}
}

class MockRoute {
  get params() {
    return Observable.of({});
  };
}

describe("SearchResultsComponent", () => {
  let fixture: ComponentFixture<SearchComponent>;
  let component: SearchComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {
    MockAppStore.state = {
      packages: {
        visible: List(),
        ui: {
          visible: {}
        }
      }
    };
  });

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
          selector: "hab-search-results",
          inputs: [ "errorMessage", "noPackages", "layout", "packages", "versions" ]
        }),
        SearchComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(SearchComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  describe("given the core origin", () => {

    beforeEach(() => {
      component.origin = "core";
      fixture.detectChanges();
    });

    it("shows the Search Packages heading", () => {
      let heading = element.query(By.css(".hab-search h2"));
      expect(heading.nativeElement.textContent).toBe("Search Packages");
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
        let heading = element.query(By.css(".hab-search h2"));
        expect(heading.nativeElement.textContent).toBe("Search Packages");
      });

      it("shows the search box", () => {
        expect(element.query(By.css(".page-body input[type='search']"))).not.toBeNull();
      });

      describe("fetch", () => {

        it ("fetches with the distinct parameter", () => {
          spyOn(actions, "filterPackagesBy");

          component.fetch();

          expect(actions.filterPackagesBy).toHaveBeenCalledWith(
            { origin: undefined }, "foo", true
          );
        });
      });
    });
  });
});
