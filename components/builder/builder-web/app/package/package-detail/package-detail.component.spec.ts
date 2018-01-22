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
import { Package } from '../../records/Package';
import * as actions from '../../actions/index';
import { PackageDetailComponent } from './package-detail.component';

class MockRoute {
  params = Observable.of({
    origin: 'core',
    name: 'nginx'
  });
}

describe('PackageDetailComponent', () => {
  let fixture: ComponentFixture<PackageDetailComponent>;
  let component: PackageDetailComponent;
  let element: DebugElement;

  beforeEach(() => {

    TestBed.configureTestingModule({
      declarations: [
        PackageDetailComponent,
        MockComponent({ selector: 'hab-platform-icon', inputs: ['platform'] }),
        MockComponent({ selector: 'hab-channels', inputs: ['channels'] }),
        MockComponent({ selector: 'hab-package-list', inputs: ['currentPackage', 'packages'] }),
        MockComponent({ selector: 'hab-copyable', inputs: ['text', 'style'] })
      ]
    });

    fixture = TestBed.createComponent(PackageDetailComponent);
    component = fixture.componentInstance;
    element = fixture.debugElement;
  });

  describe('given a package', () => {

    beforeEach(() => {

      component.package = Package({
        ident: {
          origin: 'core',
          name: 'nginx',
          version: '1.11.10',
          release: '20170829004822'
        },
        checksum: 'some-checksum',
        channels: ['unstable', 'stable']
      });
    });

    it('renders it', () => {
      fixture.detectChanges();

      function textOf(selector) {
        return element.query(By.css(`.package-detail-component ${selector}`)).nativeElement.textContent;
      }

      expect(textOf('.metadata')).toContain('1.11.10');
      expect(textOf('.metadata')).toContain('20170829004822');
      expect(textOf('.metadata')).toContain('some-checksum');
    });
  });
});
