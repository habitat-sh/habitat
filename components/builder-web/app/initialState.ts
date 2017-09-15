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

import { List, Map, Record } from "immutable";
import { BehaviorSubject } from "rxjs";
import { Origin } from "./records/Origin";
import { Package } from "./records/Package";
import { getBrowserCookies } from "./actions/cookies";

export default Record({
  app: Record({
    name: "Habitat",
    currentYear: new Date().getFullYear(),
  })(),
  gitHub: Record({
    authState: undefined,
    authToken: undefined,
    orgs: List(),
    repos: List(),
    selectedOrg: undefined,
    username: undefined,
    ui: Record({
      orgs: Record({
        loading: false,
      })(),
      repos: Record({
        loading: false,
      })()
    })()
  })(),
  builds: Record({
    visible: List(),
    selected: Record({
      info: Record({
        id: undefined,
        origin: undefined,
        name: undefined,
        version: undefined,
        release: undefined,
        state: undefined,
        build_start: undefined,
        build_stop: undefined,
        created_at: undefined
      })(),
      log: Record({
        start: undefined,
        stop: undefined,
        content: new BehaviorSubject([]),
        is_complete: undefined,
        stream: undefined
      })(),
      stream: false
    })()
  })(),
  notifications: Record({
    all: List(),
  })(),
  orgs: Record({
    added: List(),
    all: List(),
    current: Record({
      namespace: undefined,
      name: undefined,
      email: undefined,
      website: undefined,
      members: List(),
      availableMemberSearchResults: List([
        Record({
          username: "testUser",
          name: "Test User",
          email: "smith+chef-logo@getchef.com",
          status: "",
          canBeAdded: true,
          ui: Record({
            isActionsMenuOpen: false
          })(),
        })(),
        Record({
          username: "testUser2",
          name: "Test User 2",
          email: "nlloyds@gmail.com",
          status: "",
          canBeAdded: true,
          ui: Record({
            isActionsMenuOpen: false
          })(),
        })(),
      ]),
      memberSearchResults: List(),
    })(),
    ui: Record({
      create: Record({
        saved: false,
      })(),
    })(),
  })(),
  origins: Record({
    current: Origin(),
    currentPublicKeys: List(),
    currentMembers: List(),
    currentPendingInvitations: List(),
    mine: List(),
    myInvitations: List(),
    currentIntegrations: List(),
    ui: Record({
      current: Record({
        addingPublicKey: false,
        addingPrivateKey: false,
        creating: false,
        errorMessage: undefined,
        exists: false,
        loading: true,
        privateKeyErrorMessage: undefined,
        publicKeyErrorMessage: undefined,
        publicKeyListErrorMessage: undefined,
        userInviteErrorMessage: undefined,
        integrationsSaveErrorMessage: undefined
      })(),
      mine: Record({
        errorMessage: undefined,
        loading: true,
      })(),
    })(),
  })(),
  packages: Record({
    current: Package(),
    dashboard: Record({
      origin: undefined,
      recent: List()
    })(),
    explore: Record({
      popular: List(),
      your_app: List(),
      community: List(),
      stats: Record({
        plans: 0,
        builds: 0
      })()
    })(),
    latest: Package(),
    visible: List(),
    versions: undefined,
    nextRange: 0,
    searchQuery: "",
    totalCount: 0,
    ui: Record({
      current: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
      versions: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
      visible: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
    })(),
  })(),
  projects: Record({
    // This is a temporary hack that lets us add projects, and gets
    // concatted with projects on display. In real life we'll make another
    // server call when displaying a list after a project is added and it will
    // be there
    added: List(),
    all: List(),
    hint: {},
    current: Record({

      // TODO: Once we merge the project work into the package UI,
      // make `current` a Project() record and `ui` a sibling of that.
      id: undefined,
      name: undefined,
      origin_id: undefined,
      origin_name: undefined,
      owner_id: undefined,
      package_name: undefined,
      plan_path: undefined,
      vcs_data: undefined,
      vcs_type: undefined,

      ui: Record({
        exists: false,
        loading: true,
      })()
    })(),
  })(),
  router: Record({
    requestedRoute: "",
    route: "",
    redirectRoute: ""
  })(),
  ui: Record({
    layout: "default"
  })(),
  users: Record({
    current: Record({
      email: undefined,
      isSignedIn: false,
      isSigningIn: false,
      isUserNavOpen: false,
      username: undefined,
      flags: 0,
      gitHub: Map(),
    })(),
  })(),
  featureFlags: Record({
    current: Map({})
  })()
})();
