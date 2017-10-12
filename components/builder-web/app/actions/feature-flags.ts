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

import { merge } from 'lodash';

import { setCookie } from './index';
import { getBrowserCookies } from './cookies';
import { Map } from 'immutable';
import config from '../config';

export const SET_FEATURE_FLAG = 'SET_FEATURE_FLAG';
export const SET_FEATURE_FLAGS = 'SET_FEATURE_FLAGS';

const FEATURE_FLAG_KEY = 'feature-flag';

export function setFeatureFlag(name, value) {
  setCookie(`${FEATURE_FLAG_KEY}.${name}`, value);

  return {
    type: SET_FEATURE_FLAG,
    payload: { name, value }
  };
}

export function setFeatureFlags(payload) {
  const keys = Object.keys(payload);

  if (keys.length) {
    keys.forEach((key) => {
      setCookie(`${FEATURE_FLAG_KEY}.${key}`, payload[key]);
    });
  }

  return {
    type: SET_FEATURE_FLAGS,
    payload,
  };
}

export function loadFeatureFlags() {
  const cookies = getBrowserCookies();
  let payload = Map();

  Object.keys(cookies)
    .filter((key) => key.startsWith(FEATURE_FLAG_KEY))
    .forEach((key) => {
      payload = payload.set(key.substring(FEATURE_FLAG_KEY.length + 1, key.length), cookies[key]);
    });

  if (config.feature_flags) {
    Object.keys(config.feature_flags).forEach((key) => {
      payload = payload.set(key, config.feature_flags[key]);
    });
  }

  return {
    type: SET_FEATURE_FLAGS,
    payload
  };
}

