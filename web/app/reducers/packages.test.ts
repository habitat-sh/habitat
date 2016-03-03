import {Record} from "immutable";
import packagesReducer from "./packages";
import {SET_CURRENT_PACKAGE} from "../actions/index";

describe("rootReducer", () => {
    describe(SET_CURRENT_PACKAGE, () => {
        describe("when the package does not exist", () => {
            it("sets currentPackage to null", () => {
                const state = Record({
                    current: undefined, all: [{ name: "nottest" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                expect(packagesReducer(state, action).get("current"))
                    .to.equal(null);
            });
        });

        describe("when a package has no dependencies key", () => {
            it("sets it to an empty array", () => {
                const state = Record({
                    current: undefined, all: [{ name: "test" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                expect(packagesReducer(state, action).get("current")
                    .dependencies).to.deep.equal([]);
            });
        });

        describe("when a package has no buildDependencies key", () => {
            it("sets it to an empty array", () => {
                const state = Record({
                    current: undefined, all: [{ name: "test" }]
                })();
                const action = {
                    type: SET_CURRENT_PACKAGE, payload: { name: "test" }
                };
                expect(packagesReducer(state, action).get("current")
                    .buildDependencies).to.deep.equal([]);
            });
        });
    });
});
