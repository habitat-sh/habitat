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

import { BuilderApiClient } from './BuilderApiClient';

describe('BuilderApiClient', () => {

  describe('acceptOriginInvitation', () => {

    describe('on success', () => {
      let myObj;

      beforeEach(() => {
        myObj = { myCallback: () => { } };
        spyOn(myObj, 'myCallback');
        spyOn(window, 'fetch').and.callFake(() => {
          return Promise.resolve({ ok: true });
        });
      });

      it('resolves true', (done) => {
        new BuilderApiClient('mytoken')
          .acceptOriginInvitation('123', 'myorigin')
          .then(myObj.myCallback)
          .then(() => {
            expect(myObj.myCallback).toHaveBeenCalledWith(true);
            done();
          });
      });
    });

    describe('on failure', () => {
      let myObj;

      beforeEach(() => {
        myObj = { myCallback: () => { } };
        spyOn(myObj, 'myCallback');
        spyOn(window, 'fetch').and.callFake(() => {
          return Promise.resolve({ ok: false });
        });
      });

      it('resolves with an instance of Error', (done) => {
        new BuilderApiClient('some-token')
          .acceptOriginInvitation('123', 'myorigin')
          .catch(myObj.myCallback)
          .then(() => {
            expect(myObj.myCallback).toHaveBeenCalledWith(jasmine.any(Error));
            done();
          });
      });
    });
  });
});
