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

import { DebugElement } from '@angular/core';
import { TestBed, ComponentFixture } from '@angular/core/testing';
import { By } from '@angular/platform-browser';
import { RouterTestingModule } from '@angular/router/testing';
import { MockComponent } from 'ng2-mock-component';
import { SideNavComponent } from './side-nav.component';

describe('SideNavComponent', () => {
  let fixture: ComponentFixture<SideNavComponent>;
  let element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule,
      ],
      declarations: [
        SideNavComponent,
        MockComponent({ selector: 'hab-icon', inputs: [ 'symbol' ] })
      ]
    });

    fixture = TestBed.createComponent(SideNavComponent);
    element = fixture.debugElement;
  });

  it('has links', () => {
    let links = element.queryAll(By.css('ul li a'));
    expect(links.length).toBeGreaterThan(0);
  });
});
