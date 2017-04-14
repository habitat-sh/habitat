import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { By } from "@angular/platform-browser";
import { RouterTestingModule } from "@angular/router/testing";
import { List } from "immutable";
import { AppStore } from "../AppStore";
import { ProjectsPageComponent } from "./ProjectsPageComponent";

class MockAppStore {
  projects = List();

  getState() {
    return {
      projects: {
        all: this.projects
      },
      gitHub: {
        authToken: "some-token"
      }
    };
  }

  dispatch() {}
}

describe("ProjectsPageComponent", () => {
  let fixture: ComponentFixture<ProjectsPageComponent>;
  let component: ProjectsPageComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        ProjectsPageComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(ProjectsPageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  it("has an add-project button", () => {
    let button = element.query(By.css("a.button.create"));
    fixture.detectChanges();

    expect(button.nativeElement.textContent).toBe("Add Project");
    expect(button.nativeElement.getAttribute("href")).toBe("/projects/create");
  });

  describe("projects list", () => {

    describe("when projects exist", () => {

      beforeEach(() => {
        store.projects = List([
          {
            origin: "some-origin",
            name: "some-name"
          },
          {
            origin: "some-other-origin",
            name: "some-other-name"
          }
        ]);

        fixture.detectChanges();
      });

      it("lists them", () => {
        let list = element.queryAll(By.css("ul.hab-projects-list li"));
        expect(list.length).toBe(2);
      });
    });

    describe("when no projects exist", () => {

      beforeEach(() => {
        store.projects = List();
        fixture.detectChanges();
      });

      it("shows an appropriate message", () => {
        expect(element.query(By.css(".hab-projects-list li")).nativeElement.textContent)
          .toContain("You do not have any projects yet.");
      });
    });
  });
});
