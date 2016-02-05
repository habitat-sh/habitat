import {Record} from "immutable";
import {rootReducer} from "./rootReducer";
import {SET_CURRENT_PACKAGE} from "./actions";

describe("rootReducer", () => {
    describe("SET_CURRENT_PACKAGE", () => {
        describe("when the package does not exist", () => {
            it("sets currentPackage to null", () => {
                const state = Record({
                    currentPackage: undefined, packages: [{ name: "nottest" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                chai.expect(rootReducer(state, action).get("currentPackage"))
                    .to.equal(null);
            });
        });

        describe("when a package has no dependencies key", () => {
            it("sets it to an empty array", () => {
                const state = Record({
                    currentPackage: undefined, packages: [{ name: "test" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                chai.expect(rootReducer(state, action).get("currentPackage")
                    .dependencies).to.deep.equal([]);
            });
        });

        describe("when a package has no buildDependencies key", () => {
            it("sets it to an empty array", () => {
                const state = Record({
                    currentPackage: undefined, packages: [{ name: "test" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                chai.expect(rootReducer(state, action).get("currentPackage")
                    .buildDependencies).to.deep.equal([]);
            });
        });
    });
});
