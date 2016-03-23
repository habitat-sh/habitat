import {expect} from "./helper";

describe("Projects", () => {
    describe("Projects list", () => {
        beforeEach(() => {
            browser.get("#/projects");
        });

        it("has links", () => {
            expect(element.all(by.css(".hab-projects ul a")).count()).to.eventually.
                be.greaterThan(0);
        });

        it("has a create link", () => {
            expect(element.all(by.css(".hab-projects a.create")).count()).to.eventually.
                equal(1);
        });
    });

    describe("Create a project", () => {
        beforeEach(() => {
            browser.get("#/projects");
            element(by.css(".hab-projects a.create")).click();
            element(by.css("input[name=name]")).sendKeys("testname");
            element(by.css(".hab-project-create form")).submit();
        });

        it("creates a list entry for the new Project", () => {
            expect(
                element.all(by.css(".hab-projects ul a")).get(0).getText()
            ).to.eventually.equal("smith / testname");
        });
    });
});
