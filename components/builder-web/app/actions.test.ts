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

import * as cookies from 'js-cookie';
import * as gitHub from './actions/gitHub';
import * as actions from './actions/index';
import * as depotApi from './client/depot-api';

describe('actions', () => {

  xdescribe('populateBuildLog', () => {
    describe('when data is undefined', () => {
      it('has an undefined payload', () => {

      });
    });

    describe('when data is a string', () => {
      it('has a string payload', () => {

      });
    });
  });

  xdescribe('populateExploreStats', () => {
    it('has an object payload', () => {
      let data = { plans: 123, builds: 456 };
      expect(actions.populateExploreStats(data)).toEqual({
        type: actions.POPULATE_EXPLORE_STATS,
        payload: data
      });
    });
  });

  describe('filterPackagesBy', () => {

    describe('given a query parameter', () => {

      it('encodes the parameter before sending it', () => {
        spyOn(depotApi, 'get').and.returnValue(new Promise(() => { }));
        actions.filterPackagesBy({}, 'core/awesome', false)(() => { });
        expect(depotApi.get).toHaveBeenCalledWith({ query: 'core%2Fawesome' }, 0);
      });
    });
  });

  describe('gitHub', () => {

    describe('setCookie', () => {

      it('applies the proper domain', () => {
        spyOn(cookies, 'set');

        spyOn(gitHub, 'currentHostname').and.returnValues(
          'localhost',
          'builder.habitat.sh',
          'builder.acceptance.habitat.foo',
          '1.2.3.4'
        );

        gitHub.setCookie('gitHubAuthToken', 'some-token');
        gitHub.setCookie('gitHubAuthToken', 'some-token');
        gitHub.setCookie('gitHubAuthToken', 'some-token');
        gitHub.setCookie('gitHubAuthToken', 'some-token');

        expect(cookies.set.calls.allArgs()).toEqual(
          [
            ['gitHubAuthToken', 'some-token', { domain: 'localhost', secure: false }],
            ['gitHubAuthToken', 'some-token', { domain: 'habitat.sh', secure: false }],
            ['gitHubAuthToken', 'some-token', { domain: 'habitat.foo', secure: false }],
            ['gitHubAuthToken', 'some-token', { domain: '1.2.3.4', secure: false }]
          ]
        );
      });
    });
  });
});
