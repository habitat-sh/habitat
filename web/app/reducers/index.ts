// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {combineReducers} from "redux";
import app from "./app";
import notifications from "./notifications";
import orgs from "./orgs";
import packages from "./packages";
import projects from "./projects";
import router from "./router";
import user from "./user";

export default combineReducers({
    app,
    notifications,
    orgs,
    packages,
    projects,
    router,
    user,
});