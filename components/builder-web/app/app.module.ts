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
import { LocationStrategy, HashLocationStrategy } from '@angular/common';
import { MatButtonModule, MAT_PLACEHOLDER_GLOBAL_OPTIONS } from '@angular/material';
import { BrowserModule } from '@angular/platform-browser';
import { HttpModule } from '@angular/http';
import { MatIconModule, MatRadioModule, MatTabsModule } from '@angular/material';
import { routing } from './routes';
import { AppStore } from './app.store';
import { AppComponent } from './app.component';
import { BannerComponent } from './banner/banner.component';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ExploreComponent } from './explore/explore.component';
import { FooterComponent } from './footer/footer.component';
import { GravatarComponent } from './gravatar/gravatar.component';
import { HeaderComponent } from './header/header.component';
import { NotificationsComponent } from './notifications/notifications.component';
import { SideNavComponent } from './side-nav/side-nav.component';
import { SignInPageComponent } from './sign-in-page/sign-in-page.component';
import { UserNavComponent } from './header/user-nav/user-nav.component';

import { OriginModule } from './origin/origin.module';
import { PackageModule } from './package/package.module';
import { ProfileModule } from './profile/profile.module';
import { SearchModule } from './search/search.module';
import { SharedModule } from './shared/shared.module';

@NgModule({
  imports: [
    MatIconModule,
    MatRadioModule,
    MatTabsModule,
    BrowserModule,
    FormsModule,
    HttpModule,
    MatButtonModule,
    OriginModule,
    PackageModule,
    ProfileModule,
    ReactiveFormsModule,
    SearchModule,
    SharedModule,
    routing
  ],
  declarations: [
    AppComponent,
    BannerComponent,
    ExploreComponent,
    FooterComponent,
    GravatarComponent,
    HeaderComponent,
    NotificationsComponent,
    SideNavComponent,
    SignInPageComponent,
    UserNavComponent
  ],
  providers: [
    { provide: LocationStrategy, useClass: HashLocationStrategy, },
    { provide: MAT_PLACEHOLDER_GLOBAL_OPTIONS, useValue: { float: 'always' } },
    AppStore
  ],
  bootstrap: [AppComponent]
})

export class AppModule {

}
