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

import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { MdButtonModule } from '@angular/material';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { SharedModule } from '../shared/shared.module';
import { OriginPageModule } from './origin-page/origin-page.module';
import { OriginRoutingModule } from './origin-routing.module';
import { OriginsPageComponent } from './origins-page/origins-page.component';
import { OriginCreatePageComponent } from './origin-create-page/origin-create-page.component';
import { OriginService } from './origin.service';

// This is so we can test that the ordering of the modules is correct.
// Ordering matters in this case because we have a static route 'create'
// that can get interpreted as the route variable :origin
export const imports = [
  CommonModule,
  FormsModule,
  MdButtonModule,
  OriginRoutingModule,
  OriginPageModule,
  ReactiveFormsModule,
  SharedModule
];

export const declarations = [
  OriginsPageComponent,
  OriginCreatePageComponent
];

@NgModule({
  imports,
  declarations,
  providers: [
    OriginService
  ]
})
export class OriginModule { }
