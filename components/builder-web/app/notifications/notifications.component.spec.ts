import { DebugElement } from "@angular/core";
import { TestBed, ComponentFixture } from "@angular/core/testing";
import { By } from "@angular/platform-browser";
import { NotificationsComponent } from "./NotificationsComponent";

describe("NotificationsComponent", () => {
  let fixture: ComponentFixture<NotificationsComponent>;
  let component: NotificationsComponent;
  let element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [
        NotificationsComponent
      ]
    });

    fixture = TestBed.createComponent(NotificationsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe("when one or more notifications are provided", () => {
    let notifications = [
      {
        type: "success",
        title: "Woohoo!",
        body: "It worked."
      },
      {
        type: "danger",
        title: "Oh no!",
        body: "Something went wrong."
      }
    ];

    beforeEach(() => {
      component.notifications = notifications;
      component.removeNotification = () => {};
      fixture.detectChanges();
    });

    it("renders them", () => {
      let items = element.queryAll(By.css("ul.hab-notifications li"));
      expect(items.length).toBe(2);

      let first = items[0], second = items[1];
      expect(first.query(By.css("small")).nativeElement.textContent)
        .toBe(notifications[0].type);

      expect(first.query(By.css("h1")).nativeElement.textContent)
        .toBe(notifications[0].title);

      expect(first.query(By.css("p")).nativeElement.textContent)
        .toBe(notifications[0].body);

      expect(second.query(By.css("small")).nativeElement.textContent)
        .toBe(notifications[1].type);
    });

    it("delegates to the supplied dismiss function", () => {
      spyOn(component, "removeNotification");

      let second = element.queryAll(By.css("ul.hab-notifications li a"))[1];
      second.triggerEventHandler("click", {});
      fixture.detectChanges();

      expect(component.removeNotification).toHaveBeenCalledWith(1);
    });
  });
});
