// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Returns the directory root for fixtures for this integration
//! test. Each integration test module should have a fixture directory
//! with the same name as the module.
//!
//! Thus for an integration test module named `foo.rs`, you would put
//! your fixtures all in the `/components/sup/tests/fixtures/foo`
//! directory, and then call `FixtureRoot::new("foo")` in your test.
//!
//! Currently our fixtures are limited to testing packages. We have
//! `*.spec` files and package directories, which you can think of as
//! minimal expanded Habitat artifacts.
//!
//! For example, the current fixtures for the `compilation` test suite
//! are as follows:
//!
//!       $ ls -1 components/sup/tests/fixtures/compilation
//!       config-changes-hooks-do-not
//!       config-changes-hooks-do-not.spec
//!       config-only
//!       config-only.spec
//!       hook-changes-config-does-not
//!       hook-changes-config-does-not.spec
//!       no-changes-no-restart
//!       no-changes-no-restart.spec
//!       no-configs-only-hooks
//!       no-configs-only-hooks.spec
//!
//! We have a number of directories representing testing packages,
//! along with their corresponding spec files. A representative
//! directory looks like this:
//!
//!       tests/fixtures/compilation/config-only
//!       ├── config
//!       │   └── config.toml
//!       ├── hooks
//!       │   └── run
//!       └── TARGET
//!
//! They contain only what is needed to a) be recognized by the
//! Supervisor as packages and b) satisfy the needs of their test.
//!
//! In general, each fixture should be used by a single test.

use std::{path::{Path,
                 PathBuf},
          string::ToString};

#[derive(Clone, Debug)]
pub struct FixtureRoot(PathBuf);

impl FixtureRoot {
    pub fn new<S>(suite_name: S) -> FixtureRoot
        where S: AsRef<Path>
    {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join(suite_name.as_ref());
        FixtureRoot(root)
    }

    /// There should be a spec file in the root of the fixture directory
    pub fn spec_path(&self, package_name: &str) -> PathBuf {
        self.0
            .to_path_buf()
            .join(format!("{}.spec", &package_name.to_string()))
    }

    /// Fixture files for an expanded bundle... think of what a .hart
    /// expands into.
    pub fn expanded_package_dir<P>(&self, package_name: P) -> PathBuf
        where P: AsRef<Path>
    {
        self.0.to_path_buf().join(package_name.as_ref())
    }
}

impl AsRef<Path> for FixtureRoot {
    fn as_ref(&self) -> &Path { &self.0.as_path() }
}
