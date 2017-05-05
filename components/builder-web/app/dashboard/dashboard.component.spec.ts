import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { By } from "@angular/platform-browser";
import { RouterTestingModule } from "@angular/router/testing";
import { List } from "immutable";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../AppStore";
import *  as actions from "../actions";
import { DashboardComponent } from "./dashboard.component";

class MockAppStore {
  static selectedOrigin = "core";
  static recentPackages = List();
  static myOrigins = List();

  getState() {
    return {
      origins: {
        mine: MockAppStore.myOrigins,
        ui: {
          mine: {
            loading: false
          }
        }
      },
      packages: {
        dashboard: {
          get recent() {
            return MockAppStore.recentPackages;
          }
        }
      },
      users: {
        current: {
          username: "cnunciato"
        }
      }
    };
  }
  dispatch() {}
  subscribe() {}
}

describe("DashboardComponent", () => {
  let fixture: ComponentFixture<DashboardComponent>;
  let component: DashboardComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        DashboardComponent,
        MockComponent({ selector: "hab-spinner", inputs: [ "isSpinning" ]})
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(DashboardComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  beforeEach(() => {
    component._hiddenSections = [];
    fixture.detectChanges();
  });

  it("displays the signed-in username", () => {
    fixture.detectChanges();
    expect(element.query(By.css("h3.username")).nativeElement.textContent)
      .toBe("cnunciato");
  });

  describe("my-origins section", () => {

    describe("when one or more origins exists", () => {

      function originsList() {
        return element.queryAll(By.css(".origins ul li"));
      }

      beforeEach(() => {
        MockAppStore.myOrigins = List([{ name: "core" }, { name: "cnunciato" }]);
        fixture.detectChanges();
      });

      it("lists them", () => {
        let names = originsList().map(o => o.nativeElement.textContent.trim());
        expect(names).toEqual(["core", "cnunciato"]);
      });

      it("links each one to the detail view", () => {
        expect(originsList()[0].query(By.css("a")).nativeElement.getAttribute("href"))
          .toBe("/origins/core");
      });
    });

    describe("when no origins exist", () => {

      beforeEach(() => {
        MockAppStore.myOrigins = List([]);
        fixture.detectChanges();
      });

      it("shows appropriate messaging", () => {
        expect(element.query(By.css(".origins .none")).nativeElement.textContent)
          .toContain("Create your first origin, then begin uploading packages.");
      });
    });
  });

  describe("my recent packages", () => {

    describe("origin selector", () => {

      function originsOptions() {
        return element.queryAll(By.css(".recent select option"));
      }

      beforeEach(() => {
        MockAppStore.myOrigins = List([{ name: "core" }, { name: "cnunciato" }]);
        fixture.detectChanges();
      });

      it("contains a list of my origins", () => {
        let names = originsOptions().map(o => o.nativeElement.textContent.trim());
        expect(names).toEqual(["core", "cnunciato"]);
      });

      describe("when the selected origin has packages", () => {

        beforeEach(() => {
          MockAppStore.recentPackages = List([
            {
              "name": "thing1"
            },
            {
              "name": "thing2"
            }
          ]);

          fixture.detectChanges();
        });

        it("lists them", () => {
          expect(element.queryAll(By.css(".recent ul li.item .package-name"))
            .map(a => a.nativeElement.textContent.trim()))
            .toEqual(["thing1", "thing2"]);
        });
      });

      describe("when the selected origin has no packages", () => {

        beforeEach(() => {
          MockAppStore.recentPackages = List();
          fixture.detectChanges();
        });

        it("shows an appropriate message", () => {
          expect(element.query(By.css(".recent .none")).nativeElement.textContent)
            .toContain("You haven't uploaded any packages to this origin yet.");
        });
      });

      describe("on change", () => {

        beforeEach(() => {
          MockAppStore.selectedOrigin = "core";
          spyOn(actions, "fetchDashboardRecent");
          spyOn(store, "dispatch");

          element.query(By.css(".recent select"))
            .triggerEventHandler("change", MockAppStore.selectedOrigin);

          fixture.detectChanges();
        });

        it("dispatches a request for packages for that origin", () => {
          expect(store.dispatch).toHaveBeenCalledWith(actions.fetchDashboardRecent("core"));
        });
      });
    });
  });
});
