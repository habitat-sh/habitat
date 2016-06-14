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

import {Injectable} from "angular2/core";
import {applyMiddleware, compose, createStore} from "redux";
import rootReducer from "./reducers/index";
import * as thunk from "redux-thunk";

const resetMiddleware = require("redux-reset").default;

const finalCreateStore = compose(
    // The thunk middleware allows an action to return a function that takes a
    // dispatch argument instead of returning an object directly. This allows
    // actions to make async calls.
    applyMiddleware(thunk),
    // Allows resetting of the store
    resetMiddleware(),
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
