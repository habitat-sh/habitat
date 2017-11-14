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
import { Component, DebugElement, SimpleChange } from '@angular/core';
import { By } from '@angular/platform-browser';
import { RouterTestingModule } from '@angular/router/testing';
import { List } from 'immutable';
import { MockComponent } from 'ng2-mock-component';
import { AppStore } from '../../app.store';
import * as actions from '../../actions/index';
import { Package } from '../../records/Package';
import { PackageSidebarComponent } from './package-sidebar.component';

class MockAppStore {
  static state;

  getState() {
    return MockAppStore.state;
  }

  dispatch() { }
}

describe('PackageSidebarComponent', () => {
  let fixture: ComponentFixture<PackageSidebarComponent>;
  let component: PackageSidebarComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        PackageSidebarComponent,
        MockComponent({ selector: 'hab-copyable', inputs: ['command'] }),
        MockComponent({ selector: 'hab-platform-icon', inputs: ['platform'] })
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore }
      ]
    });

    fixture = TestBed.createComponent(PackageSidebarComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  beforeEach(() => {
    MockAppStore.state = {
      packages: {
        latestInChannel: {
          stable: {
            ident: {
              origin: 'core',
              name: 'nginx',
              version: '1.11.10'
            }
          }
        }
      },
      session: {
        token: undefined
      },
      gitHub: {
        authToken: undefined
      },
      origins: {
        mine: List()
      }
    };
  });

  describe('given an origin and name', () => {

    beforeEach(() => {
      spyOn(store, 'dispatch');
      spyOn(actions, 'fetchLatestInChannel');

      component.ngOnChanges({
        origin: new SimpleChange(undefined, 'core', true),
        name: new SimpleChange(undefined, 'nginx', true)
      });
    });

    it('fetches the latest package', () => {
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchLatestInChannel).toHaveBeenCalledWith('core', 'nginx', 'stable');
    });

    it('hides the build button', () => {
      expect(element.query(By.css('.package-sidebar-component button.build'))).toBeNull();
    });

    describe('when buildable', () => {

      beforeEach(() => {
        component.buildable = true;
      });

      it('shows the build button', () => {
        fixture.detectChanges();
        expect(element.query(By.css('.package-sidebar-component button.build'))).not.toBeNull();
      });

      describe('and building', () => {

        beforeEach(() => {
          component.building = true;
        });

        it('disables the build button', () => {
          fixture.detectChanges();

          let el = element.query(By.css('.package-sidebar-component button.build')).nativeElement;
          expect(el.getAttribute('disabled')).not.toBeNull();
        });
      });
    });
  });
});
