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

import * as actionTypes from '../actions/index';
import initialState from '../initialState';
import { List, Record } from 'immutable';
import { Project } from '../records/Project';

export default function projects(state = initialState['projects'], action) {

  switch (action.type) {

    case actionTypes.CLEAR_PROJECTS:
      return state.set('visible', List()).
        setIn(['ui', 'visible', 'errorMessage'], undefined).
        setIn(['ui', 'visible', 'exists'], false).
        setIn(['ui', 'visible', 'loading'], true);

    case actionTypes.CLEAR_CURRENT_PROJECT:
      return state.setIn(['current'], Project()).
        setIn(['ui', 'current', 'exists'], false).
        setIn(['ui', 'current', 'loading'], true);

    case actionTypes.CLEAR_CURRENT_PROJECT_INTEGRATION:
      return state.setIn(['current', 'settings'], undefined);

    case actionTypes.SET_CURRENT_PROJECT:
      if (action.error) {
        return state.setIn(['current'], Project()).
          setIn(['ui', 'current', 'exists'], false).
          setIn(['ui', 'current', 'loading'], false);
      }
      else {
        if (action.payload.visibility !== 'public') {
          action.payload.visibility = 'private';
        }
        return state.setIn(['current'], Project(action.payload)).
          setIn(['ui', 'current', 'exists'], true).
          setIn(['ui', 'current', 'loading'], false);
      }

    case actionTypes.SET_CURRENT_PROJECT_INTEGRATION:
      return state.setIn(['current', 'settings'], action.payload);

    case actionTypes.SET_PROJECTS:
      if (action.error) {
        return state.set('visible', List()).
          setIn(['ui', 'visible', 'errorMessage'], action.error.message).
          setIn(['ui', 'visible', 'exists'], false).
          setIn(['ui', 'visible', 'loading'], false);
      } else {
        return state.set('visible', state.get('visible').concat(List(action.payload))).
          setIn(['ui', 'visible', 'errorMessage'], undefined).
          setIn(['ui', 'visible', 'exists'], true).
          setIn(['ui', 'visible', 'loading'], false);
      }

    default:
      return state;
  }
}
