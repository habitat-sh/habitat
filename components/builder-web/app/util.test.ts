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
import * as util from "./util";

describe("util", () => {
    describe("packageString", () => {
        describe("with a fully qualified identifier", () => {
            it("returns the string", () => {
                expect(util.packageString({
                    origin: "testorigin",
                    name: "testname",
                    version: "1.0.0",
                    release: "197001010000",
                })
                ).to.eq("testorigin/testname/1.0.0/197001010000");
            });
        });

        describe("with a missing parts", () => {
            it("returns the partial string", () => {
                expect(util.packageString({
                    origin: "testorigin",
                    name: "testname",
                })
                ).to.eq("testorigin/testname");
            });
        });
    });

    describe("parseKey", () => {
        describe("with an invalid key", () => {
            it("has a valid:false property", () => {
                expect(util.parseKey("").valid).to.eq(false);
            });
        });

        describe("with a valid key", () => {
            let keyString;

            beforeEach(() => {
                keyString = `SIG-PUB-1
core-20160423193745

Jpmj1gD9oTFCgz3wSLltt/QB6RTmNRWoUTe+xhDTIHc=`;
            });

            it("has a name property", () => {
                expect(util.parseKey(keyString).name).to.eq(
                    "core-20160423193745"
                );
            });

            it("has a valid:true property", () => {
                expect(util.parseKey(keyString).valid).to.eq(true);
            });

            it("has an origin property", () => {
                expect(util.parseKey(keyString).origin).to.eq("core");
            });

            it("has a text property", () => {
                expect(util.parseKey(keyString).text).to.eq(keyString);
            });

            describe("with a private key", () => {
                beforeEach(() => {
                    keyString = `SIG-SEC-1
core-20160423193745

NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN==`;
                });

                it("has an uploadPath property", () => {
                    expect(util.parseKey(keyString).uploadPath).to.eq(
                        "core/secret_keys/20160423193745"
                    );
                });
            });

            describe("with a public key", () => {
                it("has a type property", () => {
                    expect(util.parseKey(keyString).type).to.eq("SIG-PUB-1");
                });

                it("has an uploadPath property", () => {
                    expect(util.parseKey(keyString).uploadPath).to.eq(
                        "core/keys/20160423193745"
                    );
                });
            });
        });
    });
});
