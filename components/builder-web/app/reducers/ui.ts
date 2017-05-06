import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function notifications(state = initialState["ui"], action) {
    switch (action.type) {
        case actionTypes.SET_LAYOUT:
            return state.set("layout", action.payload);
        default:
            return state;
    }
}
