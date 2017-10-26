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

import * as Cookies from 'js-cookie';
import { merge } from 'lodash';

export const SET_COOKIE = 'SET_COOKIE';
export const REMOVE_COOKIE = 'REMOVE_COOKIE';
export const domain = cookieDomain();
export const secure = window.location.protocol === 'https';

export function getCookie(name: string) {
  return Cookies.get(name);
}

export function getBrowserCookies() {
  return Cookies.get();
}

export function setCookie(name: string, value: any, opts: Object = {}) {
  Cookies.set(name, value, merge({ domain, secure }, opts));

  return {
    type: SET_COOKIE,
    payload: {
      name,
      value
    }
  };
}

export function removeCookie(name: string, opts: Object = {}) {
  return {
    type: REMOVE_COOKIE,
    payload: Cookies.remove(name, merge({ domain, secure }, opts))
  };
}

function cookieDomain() {
  let delim = '.';
  let hostname = location.hostname;
  let tld = hostname.split(delim).pop();

  if (isNaN(Number(tld))) {
    let domain = hostname.split(delim);
    domain.shift();
    return domain.join(delim);
  } else {
    return hostname;
  }
}
