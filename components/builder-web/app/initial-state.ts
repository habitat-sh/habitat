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

import { List, Map, Record } from 'immutable';
import { BehaviorSubject } from 'rxjs';
import { Origin } from './records/Origin';
import { Package } from './records/Package';
import { Project } from './records/Project';

export default Record({
  app: Record({
    name: 'Habitat',
    currentYear: new Date().getFullYear(),
  })(),
  session: Record({
    token: undefined
  })(),
  gitHub: Record({
    authState: undefined,
    authToken: undefined,
    installations: List(),
    username: undefined,
    ui: Record({
      installations: Record({
        loading: false
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
  origins: Record({
    current: Origin(),
    currentPublicKeys: List(),
    currentMembers: List(),
    currentPendingInvitations: List(),
    mine: List(),
    myInvitations: List(),
    currentIntegrations: Record({
      integrations: undefined,
      ui: Record({
        creds: Record({
          validating: false,
          validated: false,
          valid: false,
          message: undefined
        })()
      })()
    })(),
    ui: Record({
      current: Record({
        addingPublicKey: false,
        addingPrivateKey: false,
        creating: false,
        errorMessage: undefined,
        exists: false,
        loading: true,
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
      popular: List([
        {
          'origin': 'core',
          'name': 'python2',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'ruby',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'go',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'node',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'jdk8',
          'originCount': 4,
          'starCount': 2345
        }
      ]),
      your_app: List([
        {
          'origin': 'core',
          'name': 'scaffolding-ruby',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'scaffolding-node',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'nginx',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'tomcat8',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'core',
          'name': 'docker',
          'originCount': 4,
          'starCount': 2345
        }
      ]),
      community: List([
        {
          'origin': 'endocode',
          'name': 'drupal',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'endocode',
          'name': 'jenkins',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'starkandwayne',
          'name': 'wordpress',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'starkandwayne',
          'name': 'postgresql',
          'originCount': 4,
          'starCount': 2345
        },
        {
          'origin': 'starkandwayne',
          'name': 'mysql',
          'originCount': 4,
          'starCount': 2345
        }
      ]),
      stats: Record({
        plans: 0,
        builds: 0
      })()
    })(),
    latest: Package(),
    latestInChannel: Record({
      stable: undefined,
      unstable: undefined
    })(),
    visible: List(),
    versions: undefined,
    nextRange: 0,
    searchQuery: '',
    totalCount: 0,
    ui: Record({
      current: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
      latest: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
      latestInChannel: Record({
        stable: Record({
          errorMessage: undefined,
          exists: false,
          loading: true,
        })(),
        unstable: Record({
          errorMessage: undefined,
          exists: false,
          loading: true,
        })()
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
    current: Project(),
    visible: List(),
    ui: Record({
      current: Record({
        exists: false,
        loading: true,
      })(),
      visible: Record({
        errorMessage: undefined,
        exists: false,
        loading: true,
      })(),
    })()
  })(),
  router: Record({
    requestedRoute: undefined,
    route: Record({
      id: undefined,
      description: undefined,
      url: undefined,
      urlAfterRedirects: undefined
    })()
  })(),
  ui: Record({
    layout: 'default'
  })(),
  users: Record({
    current: Record({
      email: undefined,
      failedSignIn: false,
      isSigningIn: false,
      isUserNavOpen: false,
      username: undefined,
      flags: 0,
      gitHub: Map(),
      profile: Record({
        id: undefined,
        name: undefined,
        email: undefined
      })()
    })(),
  })(),
})();
