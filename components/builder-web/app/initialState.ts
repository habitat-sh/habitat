// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {List, Map, Record} from "immutable";

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
        current: Record({
            name: "smith",
        })(),
        mine: List(),
        ui: Record({
            current: Record({
                creating: false,
            })(),
        })(),
    })(),
    packages: Record({
        current: undefined,
        explore: List(),
        visible: List(),
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
            origin: undefined,
            name: undefined,
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
