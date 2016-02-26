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
});
