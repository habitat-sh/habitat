import * as util from "./util";

describe("util", () => {
    describe("packageString", () => {
        describe("with a fully qualified identifier", () => {
            it("returns the string", () => {
                chai.expect(util.packageString({ derivation: "testderiv",
                                       name: "testname",
                                       version: "1.0.0",
                                       release: "197001010000",
                                     })
                      ).to.eq("testderiv/testname/1.0.0/197001010000");
            });
        });
    });
});
