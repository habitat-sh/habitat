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
import { Routes, RouterModule } from '@angular/router';
import { OriginPageComponent } from './origin-page/origin-page.component';
import { OriginsPageComponent } from './origins-page/origins-page.component';
import { OriginCreatePageComponent } from './origin-create-page/origin-create-page.component';
import { UserLoggedInGuard } from '../shared/user/user.guard';

const routes: Routes = [
  {
    path: 'origins',
    component: OriginsPageComponent,
    canActivate: [UserLoggedInGuard]
  },
  {
    path: 'origins/create',
    component: OriginCreatePageComponent
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class OriginRoutingModule { }
