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

import { fromJS, List } from 'immutable';
import initialState from '../initialState';
import * as actionTypes from '../actions/index';
import config from '../config';

export default function gitHub(state = initialState['gitHub'], action) {
  switch (action.type) {

    case actionTypes.CLEAR_GITHUB_INSTALLATIONS:
      return state.set('installations', List()).
        setIn(['ui', 'installations', 'loading'], true);

    case actionTypes.LOAD_GITHUB_SESSION_STATE:
      return state.set('authState', action.payload.gitHubAuthState).
        set('authToken', action.payload.gitHubAuthToken);

    case actionTypes.POPULATE_GITHUB_INSTALLATIONS:
      const filtered = action.payload.filter((i) => {
        return i.app_id.toString() === config['github_app_id'];
      });

      return state.set('installations', fromJS(filtered)).
        setIn(['ui', 'installations', 'loading'], false);

    case actionTypes.SET_GITHUB_AUTH_STATE:
      return state.set('authState', action.payload);

    case actionTypes.SET_GITHUB_AUTH_TOKEN:
      return state.set('authToken', action.payload);

    default:
      return state;
  }
}
