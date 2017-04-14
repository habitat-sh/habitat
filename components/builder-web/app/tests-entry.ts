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

let testing = require("@angular/core/testing");
let browser = require("@angular/platform-browser-dynamic/testing");

// Initialize the test environment
testing.TestBed.initTestEnvironment(
  browser.BrowserDynamicTestingModule,
  browser.platformBrowserDynamicTesting()
);

// Ensure config global exists
window["Habitat"] = { config: {} };

// Load all tests
let testContext = (<{ context?: Function }>require).context("./", true, /\.(test|spec)\.ts$/);
testContext.keys().forEach(testContext);
