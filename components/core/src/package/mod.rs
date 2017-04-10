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

pub mod archive;
pub mod ident;
pub mod install;
pub mod metadata;
pub mod plan;
pub mod target;

pub use self::archive::{FromArchive, PackageArchive};
pub use self::ident::{Identifiable, PackageIdent};
pub use self::install::PackageInstall;
pub use self::plan::Plan;
pub use self::target::{Target, PackageTarget};

#[cfg(test)]
pub mod test_support {
    use std::path::PathBuf;

    pub fn fixture_path(name: &str) -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name);
        path
    }
}
