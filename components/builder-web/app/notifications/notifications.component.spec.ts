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
import { NotificationsComponent } from './notifications.component';

describe('NotificationsComponent', () => {
  let fixture: ComponentFixture<NotificationsComponent>;
  let component: NotificationsComponent;
  let element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [
        NotificationsComponent
      ]
    });

    fixture = TestBed.createComponent(NotificationsComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe('when one or more notifications are provided', () => {
    let notifications = [
      {
        type: 'success',
        title: 'Woohoo!',
        body: 'It worked.'
      },
      {
        type: 'danger',
        title: 'Oh no!',
        body: 'Something went wrong.'
      }
    ];

    beforeEach(() => {
      component.notifications = notifications;
      component.removeNotification = () => { };
      fixture.detectChanges();
    });

    it('renders them', () => {
      let items = element.queryAll(By.css('ul.hab-notifications li'));
      expect(items.length).toBe(2);

      let first = items[0], second = items[1];
      expect(first.query(By.css('small')).nativeElement.textContent)
        .toBe(notifications[0].type);

      expect(first.query(By.css('h1')).nativeElement.textContent)
        .toBe(notifications[0].title);

      expect(first.query(By.css('p')).nativeElement.textContent)
        .toBe(notifications[0].body);

      expect(second.query(By.css('small')).nativeElement.textContent)
        .toBe(notifications[1].type);
    });

    it('delegates to the supplied dismiss function', () => {
      spyOn(component, 'removeNotification');

      let second = element.queryAll(By.css('ul.hab-notifications li a'))[1];
      second.triggerEventHandler('click', {});
      fixture.detectChanges();

      expect(component.removeNotification).toHaveBeenCalledWith(1);
    });
  });
});
