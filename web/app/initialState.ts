import {List, Record} from "immutable";

export default Record({
    appName: "bldr",
    currentPackage: null,
    currentYear: new Date().getFullYear(),
    email: null,
    explore: Record({ packages: List() })(),
    isSignUpFormSubmitted: false,
    isSignedIn: true,
    isUserNavOpen: false,
    notifications: List(),
    packages: [],
    password: null,
    projects: List(),
    // This is a temporary hack that lets us add projects, and gets
    // concatted with projects on display. In real life we'll make another
    // server call when displaying a list after a project is added and it will
    // be there
    addedProjects: List(),
    requestedRoute: null,
    route: null,
    username: "smith",
    visiblePackages: [],
})();
