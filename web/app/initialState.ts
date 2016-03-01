import {List, Map, Record} from "immutable";

export default Record({
    app: Record({
        name: "bldr",
        currentYear: new Date().getFullYear(),
    })(),
    gitHub: Record({
        isLinked: true,
        repos: List(),
        selectedOrg: undefined,
        username: undefined,
    })(),
    notifications: Record({
        all: List(),
    })(),
    orgs: Record({
        added: List(),
        all: List(),
        beingCreated: Record({
            namespace: undefined,
            name: undefined,
            email: undefined,
            website: undefined,
            members: List(),
        })(),
        ui: Record({
            create: Record({
                saved: false,
            })(),
        })(),
    })(),
    packages: Record({
        all: List(),
        current: undefined,
        explore: List(),
        visible: List(),
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
        requestedRoute: undefined,
        route: undefined,
    })(),
    user: Record({
        email: undefined,
        isSignedIn: true,
        isSignUpFormSubmitted: false,
        isUserNavOpen: false,
        password: undefined,
        username: "smith",
    })(),
})();
