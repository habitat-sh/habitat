// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
