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
