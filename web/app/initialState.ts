import {List, Map, Record} from "immutable";

export default Record({
    appName: "bldr",
    currentPackage: undefined,
    currentProject: Record({
        origin: undefined,
        name: undefined,
        description: undefined,
        latestBuild: undefined,
        sourceUrl: undefined,
        maintainer: Record({

        })(),
        sourceRepository: Record({

        }),
        builds: List(),
        buildLogs: Map(),
        ui: Record({
            exists: false,
            loading: true,
        })()
    })(),
    currentYear: new Date().getFullYear(),
    email: undefined,
    explore: Record({ packages: List() })(),
    isSignUpFormSubmitted: false,
    isSignedIn: true,
    isUserNavOpen: false,
    notifications: List(),
    packages: List(),
    password: undefined,
    projects: List(),
    // This is a temporary hack that lets us add projects, and gets
    // concatted with projects on display. In real life we'll make another
    // server call when displaying a list after a project is added and it will
    // be there
    addedProjects: List(),
    requestedRoute: undefined,
    route: undefined,
    username: "smith",
    visiblePackages: List(),
})();
