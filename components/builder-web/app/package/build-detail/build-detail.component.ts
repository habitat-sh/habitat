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

import { Component, HostListener, Input, OnChanges, OnDestroy, ElementRef } from '@angular/core';
import { Subscription } from 'rxjs';
import * as AnsiUp from 'ansi_up';
import * as moment from 'moment';
import { fetchBuildLog, streamBuildLog } from '../../actions/index';
import { iconForBuildState } from '../../util';
import { AppStore } from '../../app.store';

@Component({
  selector: 'hab-build-detail',
  template: require('./build-detail.component.html')
})
export class BuildDetailComponent implements OnChanges, OnDestroy {
  @Input() build;
  @Input() stream: boolean = false;

  followLog: boolean = false;

  private fetched: boolean = false;
  private logSub: Subscription;

  constructor(
    private store: AppStore,
    private elementRef: ElementRef) {
  }

  ngOnChanges(change) {
    let id = change.build.currentValue.id;

    if (id) {
      this.fetch(id);
    }
  }

  ngOnDestroy() {
    if (this.logSub) {
      this.logSub.unsubscribe();
    }

    this.store.dispatch(streamBuildLog(false));
  }

  @HostListener('window:scroll')
  @HostListener('window:resize')
  onScroll() { }

  @HostListener('window:wheel')
  onWheel() {
    this.followLog = false;
  }

  get controlsStyles() {
    let output = rectFor('.output');
    let controls = rectFor('.controls');
    let offsetY = window.innerHeight - output.top;
    let margin = 8;

    let props: any = {
      bottom: `${margin}px`
    };

    // To get the behavior we want (i.e., controls "pinned" to the bottom
    // of either the viewport or the output element), we switch between
    // fixed and absolute positioning, respectively.
    if (offsetY <= output.height) {
      props.position = 'fixed';
      props.left = `${output.right - controls.width - margin}px`;
    }
    else {
      props.position = 'absolute';
      props.right = `${margin}px`;
    }

    function rectFor(selector) {
      return document.querySelector(`.build-detail-component ${selector}`).getBoundingClientRect();
    }

    return props;
  }

  get statusIcon() {
    return iconForBuildState(this.info.state);
  }

  get statusClass() {
    return this.info.state ? this.info.state.toLowerCase() : '';
  }

  toggleFollow() {
    this.followLog = !this.followLog;

    if (this.followLog) {
      this.scrollToEnd();
    }
  }

  get buildsLink() {
    return ['/pkgs', this.build.origin, this.build.name, 'builds'];
  }

  get elapsed() {
    let started = this.build.build_started_at;
    let finished = this.build.build_finished_at;
    let e;

    if (started && finished) {
      let s = +moment.utc(started);
      let f = +moment.utc(finished);
      e = moment.utc(f - s).format('m [min], s [sec]');
    }

    return e;
  }

  get completed() {
    let finished = this.build.build_finished_at;
    let f;

    if (finished) {
      f = moment.utc(finished).format('dddd, MMMM D, YYYY [at] h:mm:ss A');
    }

    return f;
  }

  get info() {
    return this.store.getState().builds.selected.info;
  }

  get token() {
    return this.store.getState().session.token;
  }

  public scrollToTop() {
    this.followLog = false;
    window.scrollTo(0, 0);
  }

  private fetch(id) {
    if (!this.fetched) {
      this.store.dispatch(streamBuildLog(this.stream));
      this.store.dispatch(fetchBuildLog(id, this.token, 0));
      this.fetched = true;
      this.watchForLogs();
    }
  }

  private watchForLogs() {
    let pre = this.elementRef.nativeElement.querySelector('pre');
    let content = this.store.getState().builds.selected.log.content;

    this.logSub = content.subscribe((lines) => {
      let fragment = document.createDocumentFragment();

      lines.forEach((line) => {
        let el = document.createElement('div');
        el.innerHTML = AnsiUp.ansi_to_html(line);
        fragment.appendChild(el);
      });

      pre.appendChild(fragment);

      if (this.followLog) {
        this.scrollToEnd();
      }
    });
  }

  private scrollToEnd() {
    let contentHeight = heightOf('.hab-container');
    let footerHeight = heightOf('.hab-footer');
    let navHeight = heightOf('#main-nav');

    window.scrollTo(0, contentHeight - footerHeight - navHeight * 2);

    function heightOf(selector) {
      return document.querySelector(selector).getBoundingClientRect().height;
    }
  }
}
