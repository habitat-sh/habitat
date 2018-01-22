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
import { DomSanitizer } from '@angular/platform-browser';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';
import {
  MatCheckbox, MatCheckboxModule, MatIconModule, MatIconRegistry, MatRadioModule,
  MatRadioGroup, MatRadioButton, MatSlideToggleModule, MatSlideToggle, MatTooltipModule, MatTabsModule,
  MatButtonModule
} from '@angular/material';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { BreadcrumbsComponent } from './breadcrumbs/breadcrumbs.component';
import { ChannelsComponent } from './channels/channels.component';
import { CheckingInputComponent } from './checking-input/checking-input.component';
import { CopyableComponent } from './copyable/copyable.component';
import { DockerExportSettingsComponent } from './docker-export-settings/docker-export-settings.component';
import { DockerExportSettingsDialog } from './docker-export-settings/dialog/docker-export-settings.dialog';
import { DisconnectConfirmDialog } from './project-settings/dialog/disconnect-confirm/disconnect-confirm.dialog';
import { IconComponent } from './icon/icon.component';
import { PackageListComponent } from './package-list/package-list.component';
import { ProjectSettingsComponent } from './project-settings/project-settings.component';
import { PlatformIconComponent } from './platform-icon/platform-icon.component';
import { VisibilitySelectorComponent } from './visibility-selector/visibility-selector.component';
import { KeysPipe } from './pipes/keys.pipe';
import { SimpleConfirmDialog } from './dialog/simple-confirm/simple-confirm.dialog';
import { OriginMemberGuard } from './guards/origin-member.guard';
import { SignedInGuard } from './guards/signed-in.guard';

@NgModule({
  imports: [
    BrowserAnimationsModule,
    CommonModule,
    FormsModule,
    MatCheckboxModule,
    MatIconModule,
    MatTabsModule,
    MatRadioModule,
    MatSlideToggleModule,
    MatTooltipModule,
    MatButtonModule,
    ReactiveFormsModule,
    RouterModule
  ],
  declarations: [
    BreadcrumbsComponent,
    ChannelsComponent,
    CheckingInputComponent,
    CopyableComponent,
    DisconnectConfirmDialog,
    DockerExportSettingsComponent,
    DockerExportSettingsDialog,
    IconComponent,
    PackageListComponent,
    ProjectSettingsComponent,
    PlatformIconComponent,
    VisibilitySelectorComponent,
    SimpleConfirmDialog,
    KeysPipe
  ],
  entryComponents: [
    DisconnectConfirmDialog,
    DockerExportSettingsDialog,
    SimpleConfirmDialog
  ],
  exports: [
    BreadcrumbsComponent,
    ChannelsComponent,
    CheckingInputComponent,
    CopyableComponent,
    DisconnectConfirmDialog,
    DockerExportSettingsComponent,
    IconComponent,
    MatCheckbox,
    MatRadioGroup,
    MatRadioButton,
    MatSlideToggle,
    PackageListComponent,
    VisibilitySelectorComponent,
    ProjectSettingsComponent,
    PlatformIconComponent,
    KeysPipe,
    SimpleConfirmDialog,
  ],
  providers: [
    OriginMemberGuard,
    SignedInGuard
  ]
})
export class SharedModule {
  constructor(private matIconRegistry: MatIconRegistry, private sanitizer: DomSanitizer) {

    // At the time of this monkeypatching, the SVG settings applied by MatIconRegistry
    // were missing the `viewBox` attribute, which is responsible for mapping the coordinate space
    // of an SVG image to that of the viewport, enabling proper scaling. While we await resolution
    // of the issue below, we'll go ahead and plow right over Angular's implementation,
    // 'cause JavaScript is awesome.
    // https://github.com/angular/material2/issues/5188
    // https://github.com/angular/material2/blob/bef6271c617f6904cc360454805ea080e2212f2a/src/lib/icon/icon-registry.ts#L424-L436
    matIconRegistry['_setSvgAttributes'] = (svg: SVGElement): SVGElement => {

      if (!svg.getAttribute('xmlns')) {
        svg.setAttribute('xmlns', 'http://www.w3.org/2000/svg');
      }

      svg.setAttribute('fit', '');
      svg.setAttribute('height', '100%');
      svg.setAttribute('width', '100%');
      svg.setAttribute('viewBox', '0 0 24 24'); // This is the one we care about.
      svg.setAttribute('preserveAspectRatio', 'xMidYMid meet');
      svg.setAttribute('focusable', 'false');

      return svg;
    };

    matIconRegistry.addSvgIconSet(
      sanitizer.bypassSecurityTrustResourceUrl('/assets/images/icons/all.svg')
    );
  }
}
