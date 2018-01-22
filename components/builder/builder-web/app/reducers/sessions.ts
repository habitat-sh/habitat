// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

import * as actionTypes from '../actions/index';
import initialState from '../initial-state';

export default function session(state = initialState['session'], action) {
  switch (action.type) {
    case actionTypes.SET_BLDR_SESSION_TOKEN:
      return state.set('token', action.payload);
    case actionTypes.LOAD_BLDR_SESSION_STATE:
      return state.set('token', action.payload);
    default:
      return state;
  }
}
