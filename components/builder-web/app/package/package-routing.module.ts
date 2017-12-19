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
import { PackageComponent } from './package/package.component';
import { PackageBuildComponent } from './package-build/package-build.component';
import { PackageBuildsComponent } from './package-builds/package-builds.component';
import { PackageLatestComponent } from './package-latest/package-latest.component';
import { PackageSettingsComponent } from './package-settings/package-settings.component';
import { PackageReleaseComponent } from './package-release/package-release.component';
import { PackageVersionsComponent } from './package-versions/package-versions.component';
import { OriginMemberGuard } from '../shared/guards/origin-member.guard';
import { SignedInGuard } from '../shared/guards/signed-in.guard';

const routes: Routes = [
  {
    path: 'pkgs/:origin/:name',
    component: PackageComponent,
    children: [
      {
        path: '',
        component: PackageVersionsComponent,
      },
      {
        path: 'latest',
        component: PackageLatestComponent
      },
      {
        path: 'builds',
        component: PackageBuildsComponent,
        canActivate: [SignedInGuard, OriginMemberGuard]
      },
      {
        path: 'builds/:id',
        component: PackageBuildComponent,
        canActivate: [SignedInGuard, OriginMemberGuard]
      },
      {
        path: 'settings',
        component: PackageSettingsComponent,
        canActivate: [SignedInGuard, OriginMemberGuard]
      },
      {
        path: ':version',
        component: PackageVersionsComponent
      },
      {
        path: ':version/:release',
        component: PackageReleaseComponent
      }
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class PackageRoutingModule { }
