// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import {List, Map, Record} from "immutable";
import {Origin} from "./records/Origin";

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
            })(),
            mine: Record({
                errorMessage: undefined,
                loading: true,
            })(),
        })(),
    })(),
    packages: Record({
        current: undefined,
        explore: List(),
        visible: List(),
        nextRange: 0,
        searchQuery: "",
        totalCount: 0,
        ui: Record({
            current: Record({
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
        current: Record({
            id: undefined,
            plan_path: undefined,
            description: undefined,
            latestBuild: undefined,
            sourceUrl: undefined,
            maintainer: Record({

            })(),
            sourceRepository: Record({

            })(),
            builds: List(),
            buildLogs: Map(),
            ui: Record({
                exists: false,
                loading: true,
            })()
        })(),
    })(),
    router: Record({
        requestedRoute: "",
        route: "",
    })(),
    users: Record({
        current: Record({
            email: undefined,
            isSignedIn: false,
            isSigningIn: false,
            isUserNavOpen: false,
            username: undefined,
            gitHub: Map(),
        })(),
    })(),
})();
