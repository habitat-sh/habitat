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

import { fromJS } from 'immutable';
import * as actionTypes from '../actions/index';
import initialState from '../initialState';

export default function users(state = initialState['users'], action) {
  switch (action.type) {

    case actionTypes.POPULATE_GITHUB_USER_DATA:
      return state.setIn(['current', 'gitHub'], fromJS(action.payload));

    case actionTypes.SET_PRIVILEGES:
      return state.setIn(['current', 'flags'], action.payload);

    case actionTypes.SIGN_IN_ATTEMPT:
      return state.
        setIn(['current', 'username'], action.payload.username).
        setIn(['current', 'isSignedIn'], true);

    case actionTypes.TOGGLE_USER_NAV_MENU:
      return state.setIn(['current', 'isUserNavOpen'],
        !state.getIn(['current', 'isUserNavOpen']));

    case actionTypes.SET_SIGNING_IN_FLAG:
      return state.setIn(['current', 'isSigningIn'], action.payload);

    default:
      return state;
  }
}
