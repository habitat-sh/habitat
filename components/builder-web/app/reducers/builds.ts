import { List, Record } from "immutable";
import * as AnsiUp from "ansi_up";
import * as actionTypes from "../actions/index";
import initialState from "../initialState";

export default function builds(state = initialState["builds"], action) {
    switch (action.type) {

        case actionTypes.CLEAR_BUILD:
            return state
                .setIn(["selected", "info"], Record({})());

        case actionTypes.CLEAR_BUILD_LOG:
            state.get("selected").log.content.next([]);

            return state.setIn(["selected", "log"], {
                start: undefined,
                stop: undefined,
                content: state.get("selected").log.content,
                is_complete: undefined
            });

        case actionTypes.CLEAR_BUILDS:
            return state
                .setIn(["visible"], List());

        case actionTypes.POPULATE_BUILD:
            return state.setIn(["selected", "info"], action.payload);

        case actionTypes.POPULATE_BUILD_LOG:
            let payload = action.payload;
            let content = state.get("selected").log.content;

            // It'll be common to get log requests for builds that haven't
            // started yet (which will surface as errors), so in that case,
            // we'll just hand back the current state.
            if (action.error) {
                return state;
            }

            if (payload.start === 0 && !payload.is_complete) {
                content.next(payload.content || []);
            }
            else if (payload.content.length) {
                content.next(payload.content);
            }

            return state.setIn(["selected", "log"], {
                start: payload.start,
                stop: payload.stop,
                content: content,
                is_complete: payload.is_complete
            });

        case actionTypes.POPULATE_BUILDS:
            return state.setIn(["visible"], List(action.payload));

        case actionTypes.STREAM_BUILD_LOG:
            return state.setIn(["selected", "stream"], action.payload);

        default:
            return state;
    }
}
