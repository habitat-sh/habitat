import { Component, OnInit, OnDestroy } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute } from '@angular/router';
import { Subscription } from 'rxjs';
import { fetchBuild } from '../../actions/index';
import { AppStore } from '../../app.store';

@Component({
  template: require('./package-build.component.html')
})
export class PackageBuildComponent implements OnInit, OnDestroy {
  private sub: Subscription;

  constructor(
    private store: AppStore,
    private route: ActivatedRoute,
    private title: Title
  ) { }

  ngOnInit() {
    this.sub = this.route.params.subscribe((p) => {
      this.store.dispatch(fetchBuild(p.id, this.token));

      const origin = this.route.parent.snapshot.params['origin'];
      const name = this.route.parent.snapshot.params['name'];
      this.title.setTitle(`Packages › ${origin}/${name} › Build Jobs › ${p.id} | Habitat`);
    });
  }

  ngOnDestroy() {
    if (this.sub) {
      this.sub.unsubscribe();
    }
  }

  get build() {
    return this.store.getState().builds.selected.info;
  }

  get info() {
    return this.store.getState().builds.selected.info;
  }

  get token() {
    return this.store.getState().session.token;
  }
}
