import {expect} from "./helper";

describe("Projects list", () => {
    beforeEach(() => {
        browser.get("#/projects");
    });

    it("has links", () => {
        expect(element.all(by.css(".bldr-projects ul a")).count()).to.eventually.
            be.greaterThan(0);
    });
});