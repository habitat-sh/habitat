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
import { MdDialog } from '@angular/material';
import { List } from 'immutable';
import { ActivatedRoute, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { MockComponent } from 'ng2-mock-component';
import { AppStore } from '../../app.store';
import { Origin } from '../../records/Origin';
import { OriginsPageComponent } from './origins-page.component';
import * as actions from '../../actions';

class MockAppStore {
  getState() {
    return {
      session: {
        token: 'token'
      },
      gitHub: {
        authToken: 'token'
      },
      origins: {
        mine: List([Origin({ name: 'test' })]),
        myInvitations: [],
        ui: {
          mine: {
            loading: false,
            errorMessage: undefined
          }
        }
      }
    };
  }

  dispatch() { }
}

class MockDialog { }

describe('OriginsPageComponent', () => {
  let fixture: ComponentFixture<OriginsPageComponent>;
  let component: OriginsPageComponent;
  let element: DebugElement;
  let store: MockAppStore;

  beforeEach(() => {

    store = new MockAppStore();
    spyOn(store, 'dispatch');
    spyOn(actions, 'fetchMyOriginInvitations');
    spyOn(actions, 'fetchMyOrigins');

    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        OriginsPageComponent,
        MockComponent({ selector: 'hab-icon', inputs: ['symbol', 'chevron-right'] })
      ],
      providers: [
        { provide: AppStore, useValue: store },
        { provide: MdDialog, useClass: MockDialog }
      ]
    });

    fixture = TestBed.createComponent(OriginsPageComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe('given origin and name', () => {

    it('fetches the list of origins', () => {
      fixture.detectChanges();
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchMyOrigins).toHaveBeenCalledWith('token');
    });

    it('fetches the list of invitations', () => {
      fixture.detectChanges();
      expect(store.dispatch).toHaveBeenCalled();
      expect(actions.fetchMyOriginInvitations).toHaveBeenCalledWith('token');
    });
  });

  it('routes to the correct origin', () => {
    fixture.detectChanges();
    spyOn(component, 'navigateTo');
    element.query(By.css('li:last-child')).nativeElement.click();
    fixture.detectChanges();
    expect(component.navigateTo).toHaveBeenCalledWith(Origin({ name: 'test' }));
  });
});
