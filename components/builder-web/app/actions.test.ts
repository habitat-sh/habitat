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

import * as actions from "./actions/index";

describe("actions", () => {
    describe("populateBuildLog", () => {
        describe("when data is undefined", () => {
            it("has an undefined payload", () => {
                expect(actions.populateBuildLog(1, undefined)).toEqual({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: undefined },
                });
            });
        });

        describe("when data is a string", () => {
            it("has a string payload", () => {
                expect(actions.populateBuildLog(1, "hello")).toEqual({
                    type: actions.POPULATE_BUILD_LOG,
                    payload: { id: 1, data: "hello" },
                });
            });
        });
    });

    describe("populateExploreStats", () => {
        it("has an object payload", () => {
            let data = { plans: 123, builds: 456 };
            expect(actions.populateExploreStats(data)).toEqual({
                type: actions.POPULATE_EXPLORE_STATS,
                payload: data
            });
        });
    });
});
