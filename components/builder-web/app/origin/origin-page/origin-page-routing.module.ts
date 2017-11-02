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
import { OriginPageComponent } from '../origin-page/origin-page.component';
import { OriginKeysTabComponent } from './origin-keys-tab/origin-keys-tab.component';
import { OriginMembersTabComponent } from './origin-members-tab/origin-members-tab.component';
import { OriginPackagesTabComponent } from './origin-packages-tab/origin-packages-tab.component';
import { OriginSettingsTabComponent } from './origin-settings-tab/origin-settings-tab.component';
import { OriginIntegrationsTabComponent } from './origin-integrations-tab/origin-integrations-tab.component';
import { SignedInGuard } from '../../shared/guards/signed-in.guard';

const routes: Routes = [
  {
    path: 'origins/:origin',
    component: OriginPageComponent,
    canActivate: [SignedInGuard],
    children: [
      {
        path: '',
        redirectTo: 'packages',
        pathMatch: 'full'
      },
      {
        path: 'packages',
        component: OriginPackagesTabComponent
      },
      {
        path: 'keys',
        component: OriginKeysTabComponent
      },
      {
        path: 'members',
        component: OriginMembersTabComponent
      },
      {
        path: 'settings',
        component: OriginSettingsTabComponent
      },
      {
        path: 'integrations',
        component: OriginIntegrationsTabComponent
      },
      {
        path: '**',
        redirectTo: 'packages'
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class OriginPageRoutingModule { }
