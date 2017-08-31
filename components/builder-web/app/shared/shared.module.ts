import { NgModule } from "@angular/core";
import { DomSanitizer } from "@angular/platform-browser";
import { CommonModule } from "@angular/common";
import { RouterModule } from "@angular/router";
import { MdIconModule, MdIconRegistry, MdProgressBarModule, MdTooltipModule } from "@angular/material";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { BreadcrumbsComponent } from "./breadcrumbs/breadcrumbs.component";
import { BuildListComponent } from "./build-list/build-list.component";
import { BuildStatusComponent } from "./build-status/build-status.component";
import { ChannelsComponent } from "./channels/channels.component";
import { CopyableComponent } from "./copyable/copyable.component";
import { IconComponent } from "./icon/icon.component";
import { PackageInfoComponent } from "./package-info/package-info.component";
import { PackageListComponent } from "./package-list/package-list.component";
import { ProgressBarComponent } from "./progress-bar/progress-bar.component";
import { PlatformIconComponent } from "./platform-icon/platform-icon.component";

@NgModule({
  imports: [
    BrowserAnimationsModule,
    CommonModule,
    MdIconModule,
    MdProgressBarModule,
    MdTooltipModule,
    RouterModule
  ],
  declarations: [
    BreadcrumbsComponent,
    BuildListComponent,
    BuildStatusComponent,
    ChannelsComponent,
    CopyableComponent,
    IconComponent,
    PackageInfoComponent,
    PackageListComponent,
    ProgressBarComponent,
    PlatformIconComponent
  ],
  exports: [
    BreadcrumbsComponent,
    BuildListComponent,
    BuildStatusComponent,
    ChannelsComponent,
    CopyableComponent,
    IconComponent,
    PackageInfoComponent,
    PackageListComponent,
    ProgressBarComponent,
    PlatformIconComponent
  ]
})
export class SharedModule {
  constructor(private mdIconRegistry: MdIconRegistry, private sanitizer: DomSanitizer) {
    mdIconRegistry.addSvgIconSet(
        sanitizer.bypassSecurityTrustResourceUrl("/assets/images/icons/all.svg")
    );
  }
}
