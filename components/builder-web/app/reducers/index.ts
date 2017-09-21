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

import { combineReducers } from "redux";
import app from "./app";
import gitHub from "./gitHub";
import builds from "./builds";
import notifications from "./notifications";
import origins from "./origins";
import packages from "./packages";
import projects from "./projects";
import router from "./router";
import users from "./users";
import ui from "./ui";
import featureFlags from "./feature-flags";

export default combineReducers({
    app,
    gitHub,
    builds,
    notifications,
    origins,
    packages,
    projects,
    router,
    ui,
    users,
    featureFlags
});
