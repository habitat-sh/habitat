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
    packages: [],
    password: null,
    requestedRoute: null,
    route: null,
    username: "smith",
    visiblePackages: [],
})();
