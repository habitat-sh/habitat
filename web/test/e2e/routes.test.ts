import {expect} from "./helper";

describe("Routes", () => {
    describe("/linked-accounts", () => {
        it("shows linked accounts", () => {
            browser.get("#//linked-accounts");
            expect(element(by.css(".hab-linked-accounts h2")).getText()).to.
                eventually.equal("Linked Accounts");
        });
    });

    describe("/pkgs", () => {
        beforeEach(() => {
            browser.get("#/pkgs");
        });

        it("has a page title", () => {
            expect(browser.getTitle()).to.eventually.equal("hab");
        });

        it("shows all packages", () => {
            expect(element(by.css(".hab-packages h2")).getText()).to.
                eventually.equal("All Packages");
        });
    });

    describe("/pkgs/chef", () => {
        it("shows all packages for /chef", () => {
            browser.get("#/pkgs/chef");
            expect(element(by.css(".hab-packages h2")).getText()).to.
                eventually.equal("chef");
        });
    });

    describe("/pkgs/chef/zlib/1.2.8/20160111220313", () => {
        it("shows the single package", () => {
            browser.get("#/pkgs/chef/zlib/1.2.8/20160111220313");
            expect(element(by.css(".hab-package h2")).getText()).to.
                eventually.equal("chef / zlib / 1.2.8 / 20160111220313");
        });
    });

    describe("/projects", () => {
        it("shows projects", () => {
            browser.get("#/projects");
            expect(element(by.css(".hab-projects h2")).getText()).to.
                eventually.equal("Projects");
        });
    });

    describe("/projects/:origin/:name", () => {
        describe("When the project exists", () => {
            it("shows a project", () => {
                browser.get("#/projects/smith/nethack");
                expect(element(by.css(".hab-project h2")).getText()).to.
                    eventually.equal("smith / nethack");
            });
        });

        describe("When the project does not exist", () => {
            it("shows a not found page", () => {
                browser.get("#/projects/smith/nothing");
                expect(element(by.css(".hab-project h2")).getText()).to.
                    eventually.equal("Project Not Found");
            });
        });
    });
});
