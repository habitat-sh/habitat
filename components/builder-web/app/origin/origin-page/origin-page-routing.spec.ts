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

import { Location } from '@angular/common';
import { TestBed } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { Router } from '@angular/router';
import { imports, declarations } from './origin-page.module';

imports.push(RouterTestingModule);

describe('Router: Origin Page', () => {
  let location: Location;
  let router: Router;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports, declarations
    });
    router = TestBed.get(Router);
    location = TestBed.get(Location);
    router.initialNavigation();
  });

  it('navigate to origins/:origin redirects you to packages', () => {
    router.navigate(['/origins/core']).then(() => {
      expect(location.path()).toBe('/origins/core/packages');
    });
  });

  it('navigate to origins/:origin/garbage redirects you to packages', () => {
    router.navigate(['/origins/core/garbage']).then(() => {
      expect(location.path()).toBe('/origins/core/packages');
    });
  });
});
