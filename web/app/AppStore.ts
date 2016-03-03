// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Injectable} from "angular2/core";
import {applyMiddleware, compose, createStore} from "redux";
import rootReducer from "./reducers/index";
import * as thunk from "redux-thunk";

const finalCreateStore = compose(
    // The thunk middleware allows an action to return a function that takes a
    // dispatch argument instead of returning an object directly. This allows
    // actions to make async calls.
    applyMiddleware(thunk),
    // Enable dev tools if the extension is installed.
    window["devToolsExtension"] ? window["devToolsExtension"]() : (f) => f
)(createStore);
const appStore = finalCreateStore(rootReducer);

@Injectable()
export class AppStore {
    private store = appStore;

    getState() {
        return this.store.getState();
    }

    dispatch(action) {
        this.store.dispatch(action);
    }

    subscribe(listener: Function) {
        this.store.subscribe(() => listener(this.getState()));
    }
}
