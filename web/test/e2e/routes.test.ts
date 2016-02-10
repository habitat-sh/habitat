import {expect} from "./helper";

describe("routes", () => {
    describe("/pkgs", () => {
        beforeEach(() => {
            browser.get("#/pkgs");
        });

        it("has a page title", () => {
            expect(browser.getTitle()).to.eventually.equal("bldr");
        });

        it("shows all packages", () => {
            expect(element(by.css(".bldr-packages h2")).getText()).to.
                eventually.equal("All Packages");
        });
    });

    describe("/pkgs/chef", () => {
        it("shows all packages for /chef", () => {
            browser.get("#/pkgs/chef");
            expect(element(by.css(".bldr-packages h2")).getText()).to.
                eventually.equal("chef");
        });
    });

    describe("/pkgs/chef/zlib/1.2.8/20160111220313", () => {
        it("shows the single package", () => {
            browser.get("#/pkgs/chef/zlib/1.2.8/20160111220313");
            expect(element(by.css(".bldr-package h2")).getText()).to.
                eventually.equal("chef / zlib / 1.2.8 / 20160111220313");
        });
    });
});