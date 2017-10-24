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

import { Component, DebugElement } from '@angular/core';
import { TestBed, ComponentFixture } from '@angular/core/testing';
import { RouterTestingModule } from '@angular/router/testing';
import { By } from '@angular/platform-browser';
import { MockComponent } from 'ng2-mock-component';
import { List } from 'immutable';
import { BuildListComponent } from './build-list.component';

describe('BuildListComponent', () => {
  let component: BuildListComponent,
    fixture: ComponentFixture<BuildListComponent>,
    element: DebugElement;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        RouterTestingModule
      ],
      declarations: [
        MockComponent({ selector: 'hab-icon', inputs: ['symbol'] }),
        BuildListComponent
      ]
    });

    fixture = TestBed.createComponent(BuildListComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe('given a list of builds', () => {

    let builds;

    beforeEach(() => {
      builds = [
        {
          'id': '123',
          'origin': 'core',
          'project': 'nginx',
          'version': '1.0.0',
          'release': '20170505001756',
          'state': 'complete',
          'build_start': '2017-05-05 00:43:11.729835+00',
          'build_stop': '2017-05-05 00:44:00.896919+00'
        },
        {
          'id': '456',
          'origin': 'core',
          'project': 'nginx',
          'version': '1.0.0',
          'release': '20170505001756',
          'state': 'pending',
          'build_start': '2017-05-05 00:43:11.729835+00',
          'build_stop': '2017-05-05 00:44:00.896919+00'
        },
        {
          'id': '789',
          'origin': 'core',
          'project': 'nginx',
          'version': '1.0.0',
          'release': '20170505001756',
          'state': 'failed',
          'build_started_at': '2017-05-05 00:43:11.729835+00',
          'build_stopped_at': '2017-05-05 00:44:00.896919+00'
        }
      ];

      component.builds = List(builds);

      fixture.detectChanges();
    });

    it('renders them', () => {
      let items = element.queryAll(By.css('.hab-build-list ol li.item'));

      expect(items.length).toBe(3);
      expect(items[0].query(By.css('.version')).nativeElement.textContent).toContain('1.0.0');
      expect(items[0].query(By.css('.status')).nativeElement.getAttribute('title')).toBe('Complete');
      expect(items[0].query(By.css('.status')).nativeElement.getAttribute('class')).toContain('complete');
      expect(items[1].query(By.css('.status')).nativeElement.getAttribute('class')).toContain('pending');
      expect(items[2].query(By.css('.status')).nativeElement.getAttribute('class')).toContain('failed');
    });

    describe('when a build item is clicked', () => {

      it('emits an event containing the build', () => {
        let items = element.queryAll(By.css('.hab-build-list ol li.item'));

        spyOn(component.select, 'emit');
        items[1].nativeElement.click();

        expect(component.select.emit).toHaveBeenCalledWith(builds[1]);
      });
    });
  });

  describe('given an empty list of builds', () => {

    beforeEach(() => {
      component.builds = List();
      fixture.detectChanges();
    });

    it('hides the list', () => {
      let el = element.query(By.css('.hab-build-list ol li.item'));
      expect(el).toBeNull();
    });

    it('renders an appropriate message', () => {
      let el = element.query(By.css('.hab-build-list ol li.none'));
      expect(el.nativeElement.textContent).toContain('There are no available build jobs for this package.');
    });
  });
});
