// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
