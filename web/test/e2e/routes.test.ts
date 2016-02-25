import {expect} from "./helper";

describe("Routes", () => {
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

    describe("/projects", () => {
        it("shows projects", () => {
            browser.get("#/projects");
            expect(element(by.css(".bldr-projects h2")).getText()).to.
                eventually.equal("Projects");
        });
    });

    describe("/projects/:origin/:name", () => {
        describe("When the project exists", () => {
            it("shows a project", () => {
                browser.get("#/projects/smith/nethack");
                expect(element(by.css(".bldr-project h2")).getText()).to.
                    eventually.equal("smith / nethack");
            });
        });

        describe("When the project does not exist", () => {
            it("shows a not found page", () => {
                browser.get("#/projects/smith/nothing");
                expect(element(by.css(".bldr-project h2")).getText()).to.
                    eventually.equal("Project Not Found");
            });
        });
    });
});
