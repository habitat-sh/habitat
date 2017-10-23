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

import { CommonModule, } from '@angular/common';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { NgModule } from '@angular/core';
import { RouterModule } from '@angular/router';
import { MatTabsModule, MatRadioModule, MatButtonModule, MatDialogModule, MatGridListModule } from '@angular/material';
import { IntegrationDeleteConfirmDialog } from './origin-integrations-tab/dialog/integration-delete-confirm/integration-delete-confirm.dialog';
import { GenerateKeysConfirmDialog } from './origin-keys-tab/dialog/generate-keys-confirm/generate-keys-confirm.dialog';
import { KeyAddFormDialog } from './origin-keys-tab/key-add-form/key-add-form.dialog';
import { OriginPageRoutingModule } from './origin-page-routing.module';
import { OriginPageComponent } from './origin-page.component';
import { OriginPackagesTabComponent } from './origin-packages-tab/origin-packages-tab.component';
import { OriginMembersTabComponent } from './origin-members-tab/origin-members-tab.component';
import { OriginKeysTabComponent } from './origin-keys-tab/origin-keys-tab.component';
import { OriginSettingsTabComponent } from './origin-settings-tab/origin-settings-tab.component';
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

import { OriginIntegrationsTabComponent } from './origin-integrations-tab/origin-integrations-tab.component';
import { IntegrationCredentialsFormDialog } from './integration-credentials-form/integration-credentials-form.dialog';
import { SharedModule } from '../../shared/shared.module';
import { MatMenuModule } from '@angular/material';

export const imports = [
  BrowserAnimationsModule,
  CommonModule,
  FormsModule,
  MatTabsModule,
  MatRadioModule,
  MatDialogModule,
  MatButtonModule,
  MatMenuModule,
  MatGridListModule,
  ReactiveFormsModule,
  RouterModule,
  OriginPageRoutingModule,
  SharedModule
];

export const declarations = [
  IntegrationCredentialsFormDialog,
  GenerateKeysConfirmDialog,
  IntegrationDeleteConfirmDialog,
  KeyAddFormDialog,
  OriginKeysTabComponent,
  OriginMembersTabComponent,
  OriginPackagesTabComponent,
  OriginPageComponent,
  OriginSettingsTabComponent,
  OriginIntegrationsTabComponent
];

const entryComponents = [
  IntegrationCredentialsFormDialog,
  GenerateKeysConfirmDialog,
  IntegrationDeleteConfirmDialog,
  KeyAddFormDialog
];

@NgModule({
  imports,
  declarations,
  entryComponents
})
export class OriginPageModule { }
