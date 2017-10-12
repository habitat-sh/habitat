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

import { Pipe, PipeTransform } from '@angular/core';
import { GitHubRepo } from '../github/repo/shared/github-repo.model';

@Pipe({
  name: 'habGitHubRepoFilter',
  pure: false
})
export class RepoFilterPipe implements PipeTransform {
  transform(items: GitHubRepo[], filter: GitHubRepo, filterParam: string): any {
    if (!items || !filter[filterParam]) {
      return items;
    }
    // filter items array, items which match and return true will be kept, false will be filtered out
    return items.filter((item) => {
      return item.get(filterParam).toLowerCase().indexOf(filter[filterParam].toLowerCase()) > -1;
    });
  }
}
