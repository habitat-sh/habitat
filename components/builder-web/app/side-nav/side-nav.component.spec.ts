import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { By } from "@angular/platform-browser";
import { RouterTestingModule } from "@angular/router/testing";
import { SideNavComponent } from "./SideNavComponent";

describe("SideNavComponent", () => {
  let fixture: ComponentFixture<SideNavComponent>;
  let element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        SideNavComponent
      ]
    });

    fixture = TestBed.createComponent(SideNavComponent);
    element = fixture.debugElement;
  });

  it("has links", () => {
    let links = element.queryAll(By.css("ul li a"));
    expect(links.length).toBeGreaterThan(0);
  });
});
