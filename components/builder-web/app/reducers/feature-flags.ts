// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

import * as actionTypes from "../actions/index";
import initialState from "../initialState";
import { Map } from "immutable";

export default function featureFlags(state = initialState["featureFlags"], action) {

  switch (action.type) {
    case actionTypes.SET_FEATURE_FLAGS:
      return state.set("current", action.payload || Map());

    case actionTypes.SET_FEATURE_FLAG:
      return state.setIn(["current", action.payload.name], action.payload.value);

    default:
      return state;
  }
}
