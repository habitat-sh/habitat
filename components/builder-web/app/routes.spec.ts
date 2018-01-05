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

import { SignInPageComponent } from './sign-in-page/sign-in-page.component';
import { routes } from './routes';

describe('Routes', () => {

  function route(path) {
    return routes.find((r) => r.path === path);
  }

  describe('/sign-in', () => {
    it('routes to SignInPageComponent', () => {
      let r = route('sign-in');
      expect(r.component).toBe(SignInPageComponent);
    });
  });

  describe('non-existent routes', () => {
    it('redirect to /pkgs/core', () => {
      let r = route('*');
      let lastRoute = routes[routes.length - 1];
      expect(r.redirectTo).toBe('/pkgs/core');
      expect(lastRoute).toBe(r);
    });
  });
});
