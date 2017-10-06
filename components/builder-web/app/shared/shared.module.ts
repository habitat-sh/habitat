import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { DomSanitizer } from "@angular/platform-browser";
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { RouterModule } from "@angular/router";
import {
  MdCheckbox, MdCheckboxModule, MdIconModule, MdIconRegistry, MdProgressBarModule, MdRadioModule,
  MdRadioGroup, MdRadioButton, MdSlideToggleModule, MdSlideToggle, MdTooltipModule, MdTabsModule,
  MdButtonModule
} from "@angular/material";
import { BrowserAnimationsModule } from "@angular/platform-browser/animations";
import { BreadcrumbsComponent } from "./breadcrumbs/breadcrumbs.component";
import { ChannelsComponent } from "./channels/channels.component";
import { CheckingInputComponent } from "./checking-input/checking-input.component";
import { CopyableComponent } from "./copyable/copyable.component";
import { DockerExportSettingsComponent } from "./docker-export-settings/docker-export-settings.component";
import { DisconnectConfirmDialog } from "./project-settings/dialog/disconnect-confirm/disconnect-confirm.dialog";
import { IconComponent } from "./icon/icon.component";
import { PackageListComponent } from "./package-list/package-list.component";
import { ProgressBarComponent } from "./progress-bar/progress-bar.component";
import { ProjectSettingsComponent } from "./project-settings/project-settings.component";
import { PlatformIconComponent } from "./platform-icon/platform-icon.component";
import { VisibilitySelectorComponent } from "./visibility-selector/visibility-selector.component";
import { RepoFilterPipe } from "../pipes/repoFilter.pipe";
import { SimpleConfirmDialog } from "./dialog/simple-confirm/simple-confirm.dialog";
import { TabsComponent } from "./tabs/TabsComponent";
import { TabComponent } from "./tabs/TabComponent";
import { FormProgressComponent } from "./form-progress/form-progress.component";
import { GitHubRepoPickerComponent } from "./github-repo-picker/github-repo-picker.component";
import { PackagePlanSelectComponent } from "./plan-select/plan-select.component";
import { UserLoggedInGuard } from "./user/user.guard";

@NgModule({
  imports: [
    BrowserAnimationsModule,
    CommonModule,
    FormsModule,
    MdCheckboxModule,
    MdIconModule,
    MdTabsModule,
    MdProgressBarModule,
    MdRadioModule,
    MdSlideToggleModule,
    MdTooltipModule,
    MdRadioModule,
    MdButtonModule,
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
    IconComponent,
    PackageListComponent,
    ProgressBarComponent,
    ProjectSettingsComponent,
    PlatformIconComponent,
    VisibilitySelectorComponent,
    RepoFilterPipe,
    SimpleConfirmDialog,
    TabsComponent,
    TabComponent,
    FormProgressComponent,
    GitHubRepoPickerComponent,
    PackagePlanSelectComponent,
    RepoFilterPipe
  ],
  entryComponents: [
    DisconnectConfirmDialog,
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
    MdCheckbox,
    MdRadioGroup,
    MdRadioButton,
    MdSlideToggle,
    PackageListComponent,
    VisibilitySelectorComponent,
    ProgressBarComponent,
    ProjectSettingsComponent,
    PlatformIconComponent,
    RepoFilterPipe,
    SimpleConfirmDialog,
    TabsComponent,
    TabComponent,
    FormProgressComponent,
    PackagePlanSelectComponent,
    GitHubRepoPickerComponent
  ],
  providers: [
    UserLoggedInGuard
  ]
})
export class SharedModule {
  constructor(private mdIconRegistry: MdIconRegistry, private sanitizer: DomSanitizer) {

    // At the time of this monkeypatching, the SVG settings applied by MdIconRegistry
    // were missing the `viewBox` attribute, which is responsible for mapping the coordinate space
    // of an SVG image to that of the viewport, enabling proper scaling. While we await resolution
    // of the issue below, we'll go ahead and plow right over Angular's implementation,
    // 'cause JavaScript is awesome.
    // https://github.com/angular/material2/issues/5188
    // https://github.com/angular/material2/blob/bef6271c617f6904cc360454805ea080e2212f2a/src/lib/icon/icon-registry.ts#L424-L436
    mdIconRegistry["_setSvgAttributes"] = (svg: SVGElement): SVGElement => {

      if (!svg.getAttribute("xmlns")) {
        svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
      }

      svg.setAttribute("fit", "");
      svg.setAttribute("height", "100%");
      svg.setAttribute("width", "100%");
      svg.setAttribute("viewBox", "0 0 24 24"); // This is the one we care about.
      svg.setAttribute("preserveAspectRatio", "xMidYMid meet");
      svg.setAttribute("focusable", "false");

      return svg;
    };

    mdIconRegistry.addSvgIconSet(
      sanitizer.bypassSecurityTrustResourceUrl("/assets/images/icons/all.svg")
    );
  }
}
