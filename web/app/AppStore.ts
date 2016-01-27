// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

///<reference path="../vendor/typings/linq/linq.d.ts"/>

import {Injectable} from "angular2/core";
import {createStore} from "redux";
import {rootReducer} from "./rootReducer";

const appStore = createStore(rootReducer);

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
