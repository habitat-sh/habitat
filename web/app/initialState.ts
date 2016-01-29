///<reference path='../node_modules/immutable/dist/immutable.d.ts'/>

import * as Immutable from "immutable";
import packages from "../fixtures/packages.ts";

export default Immutable.Record({
  appName: "bldr",
  currentPackage: null,
  currentYear: new Date().getFullYear(),
  email: null,
  isSignUpFormSubmitted: false,
  isSignedIn: true,
  isUserNavOpen: false,
  packages,
  password: null,
  requestedRoute: null,
  route: null,
  username: "smith",
  visiblePackages: [],
})();


