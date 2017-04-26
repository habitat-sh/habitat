import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { By } from "@angular/platform-browser";
import { FormsModule } from "@angular/forms";
import { Router } from "@angular/router";
import { AppStore } from "../AppStore";
import { fetchExplore } from "../actions/index";
import { ExplorePageComponent } from "./explore-page.component";

class MockAppStore {
  getState() {
    return {
      packages: {
        explore: {
          popular: [
            {
                "name": "glibc",
                "originCount": 4,
                "starCount": 2345
            },
            {
                "name": "mongodb",
                "originCount": 3,
                "starCount": 2340
            },
            {
                "name": "redis",
                "originCount": 16,
                "starCount": 234
            },
            {
                "name": "couchdb",
                "originCount": 1,
                "starCount": 23
            }
          ],
          stats: {
            plans: 324,
            builds: 12378
          }
        }
      }
    };
  }
  dispatch() {}
}

describe("ExplorePageComponent", () => {
  let fixture: ComponentFixture<ExplorePageComponent>;
  let component: ExplorePageComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        FormsModule,
        RouterTestingModule
      ],
      declarations: [
        ExplorePageComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(ExplorePageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  describe("init", () => {
    it("dispatches a request for the data the view needs", () => {
      spyOn(store, "dispatch");
      fixture.detectChanges();
      expect(store.dispatch).toHaveBeenCalledTimes(1);
    });
  });

  describe("heading", () => {
    let heading;

    beforeEach(() => {
      heading = element.query(By.css("h1")).nativeElement;
    });

    it("exists", () => {
      expect(heading.textContent).not.toBeFalsy();
    });
  });

  describe("search form", () =>  {
    let input, button;

    beforeEach(() => {
      input = element.query(By.css("form input")).nativeElement;
      button = element.query(By.css("form button")).nativeElement;
    });

    it("exists", () => {
      expect(input.getAttribute("placeholder")).not.toBeFalsy();
      expect(button.textContent).not.toBeFalsy();
    });

    describe("submission", () => {

      it("navigates to the package-search view", () => {
        let router = TestBed.get(Router);
        spyOn(router, "navigate");

        input.value = " g++ ";
        button.click();

        expect(router.navigate).toHaveBeenCalledWith(["pkgs", "search", "g%2B%2B"]);
      });
    });
  });

  describe("packages section", () => {

    it("exists", () => {
      expect(element.query(By.css("section.packages"))).not.toBeNull();
    });

    it("renders the popular, top-dependencies and recently-added lists", () => {
      fixture.detectChanges();

      function listFor(selector) {
        return element.queryAll(By.css(`.packages .${selector} li a`));
      }

      expect(listFor("popular").length).toBe(4);
      expect(listFor("top").length).toBe(4);
      expect(listFor("recent").length).toBe(4);
    });
  });

  describe("stats section", () => {
    it("exists", () => {
      expect(element.query(By.css("section.stats"))).not.toBeNull();
    });

    it("renders plan and build counts", () => {
      fixture.detectChanges();

      function countFor(selector) {
        return element.query(By.css(`.stats .${selector} strong`)).nativeElement.textContent;
      }

      expect(countFor("plans")).toBe("324");
      expect(countFor("builds")).toBe("12378");
    });
  });

  describe("getting-started section", () => {
    it("exists", () => {
      expect(element.query(By.css("section.getting-started"))).not.toBeNull();
    });
  });

  describe("scaffolding section", () => {
    it("exists", () => {
      expect(element.query(By.css("section.scaffolding"))).not.toBeNull();
    });
  });

  describe("compliance section", () => {
    it("exists", () => {
      expect(element.query(By.css("section.compliance"))).not.toBeNull();
    });
  });

  describe("community section", () => {
    let heading, button;

    it("exists", () => {
      expect(element.query(By.css("section.community"))).not.toBeNull();
    });

    describe("call-to-action button", () => {

      beforeEach(() => {
        button = element.query(By.css("section.community a.button")).nativeElement;
      });

      it("links to the community view", () => {
        fixture.detectChanges();

        expect(button.textContent).not.toBeFalsy();
        expect(button.getAttribute("href")).toBe("https://www.habitat.sh/community/");
      });
    });
  });
});

