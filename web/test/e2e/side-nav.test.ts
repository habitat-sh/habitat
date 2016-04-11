// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
