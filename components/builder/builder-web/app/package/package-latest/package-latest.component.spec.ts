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
import { ActivatedRoute } from '@angular/router';
import { Observable } from 'rxjs';
import { MockComponent } from 'ng2-mock-component';
import { AppStore } from '../../app.store';
import { Package } from '../../records/Package';
import * as actions from '../../actions/index';
import { PackageLatestComponent } from './package-latest.component';

class MockAppStore {

  getState() {
    return {
      packages: {
        current: Package()
      }
    };
  }

  dispatch() { }
}

class MockRoute {
  parent = {
    params: Observable.of({
      origin: 'core',
      name: 'nginx'
    })
  };
}

describe('PackageLatestComponent', () => {
  let fixture: ComponentFixture<PackageLatestComponent>;
  let component: PackageLatestComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, 'dispatch');
    spyOn(actions, 'fetchLatestPackage');

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageLatestComponent,
        MockComponent({ selector: 'hab-icon', inputs: ['symbol'] }),
        MockComponent({ selector: 'hab-package-detail', inputs: ['package'] })
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(PackageLatestComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe('given origin and name', () => {

    it('fetches the latest package', () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchLatestPackage).toHaveBeenCalledWith('core', 'nginx');
    });
  });
});
