import {expect} from "./helper";

// Shortcut for creating a project
function createProject(name) {
    element(by.css(".bldr-projects a.create")).click();
    element(by.css("input[name=name]")).sendKeys(name);
    element(by.css(".bldr-project-create form")).submit();
}

describe("Notifications", () => {
    beforeEach(() => {
        browser.get("#/projects");
    });

    describe("Creating a notification", () => {
        it("adds the notification", () => {
            createProject("test1");
            expect(element.all(by.css(".bldr-notifications li")).count()).to.
                eventually.equal(1);
        });
    });

    describe("Creating two notifications", () => {
        xit("adds the notifications (disabled because of UnknownError: unknown error: Element is not clickable at point)", () => {
            createProject("test1");
            createProject("test2");
            expect(element.all(by.css(".bldr-notifications li")).count()).to.
                eventually.equal(2);
        });
    });


    describe("Dismissing a notification", () => {
        xit("removes the notification (disabled because of UnknownError: unknown error: Element is not clickable at point)", () => {
            createProject("test1");
            createProject("test2");
            let button = element.all(by.css(".bldr-notifications li a.dismiss")).get(0).getWebElement();
            browser.actions().mouseMove(button).click();
            element.all(by.css(".bldr-notifications li a.dismiss")).get(0).click();
            expect(element.all(by.css(".bldr-notifications li")).count()).to.
                eventually.equal(1);
        });
    });
});
