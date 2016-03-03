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
        it("adds the notifications", () => {
            createProject("test1");
            createProject("test2");
            expect(element.all(by.css(".bldr-notifications li")).count()).to.
                eventually.equal(2);
        });
    });


    describe("Dismissing a notification", () => {
        it("removes the notification", () => {
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
