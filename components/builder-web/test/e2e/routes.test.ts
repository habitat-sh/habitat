// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
