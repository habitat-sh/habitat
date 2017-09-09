import { TestBed, ComponentFixture } from "@angular/core/testing";
import { RouterTestingModule } from "@angular/router/testing";
import { Component, DebugElement } from "@angular/core";
import { By } from "@angular/platform-browser";
import { List } from "immutable";
import { ActivatedRoute, Router } from "@angular/router";
import { Observable } from "rxjs";
import { MockComponent } from "ng2-mock-component";
import { AppStore } from "../../AppStore";
import { Origin } from "../../records/origin";
import { OriginsPageComponent } from "./origins-page.component";
import * as actions from "../../actions";

class MockAppStore {

  getState() {
    return {
      gitHub: {
        authToken: "token"
      },
      origins: {
        mine: List([Origin({name: "test"})]),
        myInvitations: [],
        ui: {
          mine: {
            loading: false,
            errorMessage: undefined
          }
        }
      }
    };
  }
  dispatch() {}
}

describe("OriginsPageComponent", () => {
  let fixture: ComponentFixture<OriginsPageComponent>;
  let component: OriginsPageComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, "dispatch");
    spyOn(actions, "fetchMyOriginInvitations");
    spyOn(actions, "fetchMyOrigins");

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        OriginsPageComponent,
        MockComponent({ selector: "hab-icon", inputs: [ "symbol", "chevron-right" ]})
      ],
      providers: [
        { provide: AppStore, useValue: store }
      ]
    });

    fixture = TestBed.createComponent(OriginsPageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("given origin and name", () => {
    it("fetches the list of origins", () => {
      fixture.detectChanges();
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchMyOrigins).toHaveBeenCalledWith("token");
      expect(component.origins.size).toEqual(1);
    });

    it("fetches the list of invitations", () => {
      fixture.detectChanges();
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchMyOriginInvitations).toHaveBeenCalledWith("token");
      expect(component.invitations.length).toEqual(0);
    });
  });

  it("routes to the correct origin", () => {
    fixture.detectChanges();
    spyOn(component, "routeToOrigin");
    // one element is the header and the other is the origin
    expect(element.query(By.css(".origins")).children.length).toBe(2);
    element.query(By.css(".origins > li:last-child")).nativeElement.click();
    fixture.detectChanges();
    expect(component.routeToOrigin).toHaveBeenCalledWith("test");
  });
});
