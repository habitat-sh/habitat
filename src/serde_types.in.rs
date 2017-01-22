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

pub mod package_ident {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `package/ident.rs`

    #[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Hash)]
    pub struct PackageIdent {
        pub origin: String,
        pub name: String,
        pub version: Option<String>,
        pub release: Option<String>,
    }
}

pub mod package_install {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `package/install.rs`

    use std::path::PathBuf;
    use package::PackageIdent;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PackageInstall {
        pub ident: PackageIdent,
        pub fs_root_path: PathBuf,
        pub package_root_path: PathBuf,
        pub installed_path: PathBuf,
    }
}

pub mod package_target {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `package/target.rs`

    use os::system::{Architecture, Platform};

    /// Describes the platform (operating system/kernel)
    /// and architecture (x86_64, i386, etc..) that a package is built for
    #[derive(Serialize, Deserialize, Debug, Clone, Hash)]
    pub struct PackageTarget {
        pub platform: Platform,
        pub architecture: Architecture,
    }
}

pub mod service {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `service.rs`

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct ServiceGroup {
        pub service: String,
        pub group: String,
        pub organization: Option<String>,
    }
}

pub mod system {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `os/system/mod.rs`

    #[allow(non_camel_case_types)]
    #[derive(Debug, Hash, Clone, Serialize, Deserialize, Eq, PartialEq)]
    pub enum Architecture {
        X86_64,
    }

    #[derive(Debug, Hash, Clone, Serialize, Deserialize, Eq, PartialEq)]
    pub enum Platform {
        Linux,
        Windows,
        MacOS,
    }
}
