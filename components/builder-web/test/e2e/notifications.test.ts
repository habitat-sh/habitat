// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {expect} from "./helper";

// Shortcut for creating a project
function createProject(name) {
    element(by.css(".hab-projects a.create")).click();
    element(by.css("input[name=name]")).sendKeys(name);
    element(by.css(".hab-project-create form")).submit();
}

describe("Notifications", () => {
    beforeEach(() => {
        browser.get("#/projects");
    });

    describe("Creating a notification", () => {
        it("adds the notification", () => {
            createProject("test1");
            expect(element.all(by.css(".hab-notifications li")).count()).to.
                eventually.equal(1);
        });
    });

    describe("Creating two notifications", () => {
        it("adds the notifications", () => {
            createProject("test1");
            createProject("test2");
            expect(element.all(by.css(".hab-notifications li")).count()).to.
                eventually.equal(2);
        });
    });


    describe("Dismissing a notification", () => {
        it("removes the notification", () => {
            createProject("test1");
            createProject("test2");
            let button = element.all(by.css(".hab-notifications li a.dismiss")).get(0).getWebElement();
            browser.actions().mouseMove(button).click();
            element.all(by.css(".hab-notifications li a.dismiss")).get(0).click();
            expect(element.all(by.css(".hab-notifications li")).count()).to.
                eventually.equal(1);
        });
    });
});
