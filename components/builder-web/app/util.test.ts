// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
