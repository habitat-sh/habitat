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

import { Map, Record } from 'immutable';

export const Project = Record({
  id: undefined,
  name: undefined,
  origin_id: undefined,
  origin_name: undefined,
  owner_id: undefined,
  package_name: undefined,
  plan_path: undefined,
  vcs_auth_token: undefined,
  vcs_data: undefined,
  vcs_type: undefined,
  vcs_username: undefined,
  settings: Map(),
  visibility: undefined
});
