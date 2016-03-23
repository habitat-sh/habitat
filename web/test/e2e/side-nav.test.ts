import {expect} from "./helper";

describe("Side nav", () => {
    beforeEach(() => {
        browser.get("#/");
    });

    it("has links", () => {
        expect(element.all(by.css(".hab-side-nav ul a")).count()).to.eventually.
            be.greaterThan(0);
    });
});
