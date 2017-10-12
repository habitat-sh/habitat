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

import { TestBed, ComponentFixture } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { Component, DebugElement } from '@angular/core';
import { By } from '@angular/platform-browser';
import { List } from 'immutable';
import { MockComponent } from 'ng2-mock-component';
import { SearchResultsComponent } from './results.component';

describe('SearchResultsListComponent', () => {
  let fixture: ComponentFixture<SearchResultsComponent>;
  let component: SearchResultsComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        SearchResultsComponent,
        MockComponent({ selector: 'hab-icon', inputs: ['symbol'] }),
        MockComponent({ selector: 'hab-channels', inputs: ['channels'] }),
        MockComponent({
          selector: 'hab-build-status',
          inputs: ['origin', 'name', 'version']
        })
      ]
    });

    fixture = TestBed.createComponent(SearchResultsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  beforeEach(() => {
    component.packages = List([
      {
        origin: 'core',
        project: 'nginx',
        version: '1.0.2',
        release: '20170101000002',
        channels: ['stable', 'unstable']
      },
      {
        origin: 'core',
        project: 'nginx',
        version: '1.0.1',
        release: '20170101000001',
        channels: ['unstable']
      },
      {
        origin: 'core',
        project: 'nginx',
        version: '1.0.0',
        release: '20170101000000',
        channels: []
      }
    ]);
    fixture.detectChanges();
  });

  it('renders a list of packages', () => {
    expect(element.queryAll(By.css('.hab-search-results li .name')).length).toBe(3);
  });
});
