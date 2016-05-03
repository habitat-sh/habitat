// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as actions from "./actions/index";

describe("actions", () => {
    describe("populateBuildLog", () => {
        describe("when data is undefined", () => {
            it("has an undefined payload", () => {
                chai.expect(actions.populateBuildLog(1, undefined)).to.deep.equal({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: undefined },
                });
            });
        });

        describe("when data is a string", () => {
            it("has a string payload", () => {
                chai.expect(actions.populateBuildLog(1, "hello")).to.deep.equal({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: "hello" },
                });
            });
        });
    });
});
