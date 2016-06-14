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

declare var expect;
import * as actions from "./actions/index";

describe("actions", () => {
    describe("populateBuildLog", () => {
        describe("when data is undefined", () => {
            it("has an undefined payload", () => {
                expect(actions.populateBuildLog(1, undefined)).to.deep.equal({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: undefined },
                });
            });
        });

        describe("when data is a string", () => {
            it("has a string payload", () => {
                expect(actions.populateBuildLog(1, "hello")).to.deep.equal({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: "hello" },
                });
            });
        });
    });
});
