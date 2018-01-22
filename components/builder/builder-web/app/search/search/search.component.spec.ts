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
import { ReactiveFormsModule } from '@angular/forms';
import { MatInputModule } from '@angular/material';
import { By } from '@angular/platform-browser';
import { NoopAnimationsModule } from '@angular/platform-browser/animations';
import { ActivatedRoute } from '@angular/router';
import { RouterTestingModule } from '@angular/router/testing';
import { Observable } from 'rxjs';
import { List } from 'immutable';
import { MockComponent } from 'ng2-mock-component';
import * as actions from '../../actions/index';
import { AppStore } from '../../app.store';
import { SearchComponent } from './search.component';

class MockAppStore {
  static state;

  getState() {
    return MockAppStore.state;
  }

  dispatch() { }
}

class MockRoute {
  get params() {
    return Observable.of({});
  }
}

describe('SearchComponent', () => {
  let fixture: ComponentFixture<SearchComponent>;
  let component: SearchComponent;
  let element: DebugElement;
  let store: AppStore;

  beforeEach(() => {
    MockAppStore.state = {
      packages: {
        visible: List(),
        ui: {
          visible: {}
        }
      }
    };
  });

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        ReactiveFormsModule,
        RouterTestingModule,
        MatInputModule,
        NoopAnimationsModule
      ],
      declarations: [
        MockComponent({ selector: 'hab-package-breadcrumbs', inputs: ['ident'] }),
        MockComponent({ selector: 'hab-icon', inputs: ['symbol'] }),
        MockComponent({
          selector: 'hab-search-results',
          inputs: ['errorMessage', 'noPackages', 'layout', 'packages', 'versions']
        }),
        SearchComponent
      ],
      providers: [
        { provide: AppStore, useClass: MockAppStore },
        { provide: ActivatedRoute, useClass: MockRoute }
      ]
    });

    fixture = TestBed.createComponent(SearchComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
    store = TestBed.get(AppStore);
  });

  describe('given the core origin', () => {

    beforeEach(() => {
      component.origin = 'core';
      fixture.detectChanges();
    });

    it('shows the Search Packages heading', () => {
      let heading = element.query(By.css('.search-component h1'));
      expect(heading.nativeElement.textContent).toBe('Search Packages');
    });
  });

  describe('search', () => {

    describe('given a query', () => {

      beforeEach(() => {
        let query = 'foo';
        component.query = query;
        MockAppStore.state.packages.searchQuery = query;
        fixture.detectChanges();
      });

      it('shows the Search Packages heading', () => {
        let heading = element.query(By.css('.search-component h1'));
        expect(heading.nativeElement.textContent).toBe('Search Packages');
      });

      it('shows the search box', () => {
        expect(element.query(By.css('.body input[type=\'search\']'))).not.toBeNull();
      });

      describe('fetchPackages', () => {

        it('fetches with the distinct parameter', () => {
          spyOn(actions, 'filterPackagesBy');

          component.fetchPackages();

          expect(actions.filterPackagesBy).toHaveBeenCalledWith(
            { origin: 'core' }, 'foo', true, 0
          );
        });
      });
    });
  });
});
