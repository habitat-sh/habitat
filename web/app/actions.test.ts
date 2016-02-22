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
