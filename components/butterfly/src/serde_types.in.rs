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

pub mod rumor {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `rumor/mod.rs`

    pub mod tests {
        #[derive(Clone, Debug, Serialize)]
        pub struct FakeRumor {
            pub id: String,
            pub key: String,
        }

        #[derive(Clone, Debug, Serialize)]
        pub struct TrumpRumor {
            pub id: String,
            pub key: String,
        }
    }
}
pub mod rumor_election {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `rumor/election.rs`

    use message::swim::Rumor as ProtoRumor;

    /// An election.
    #[derive(Debug, Clone, Serialize)]
    pub struct Election(pub ProtoRumor);

    #[derive(Debug, Clone, Serialize)]
    pub struct ElectionUpdate(pub Election);
}

pub mod rumor_service_config {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `rumor/service_config.rs`

    use message::swim::Rumor as ProtoRumor;

    /// The service rumor
    #[derive(Debug, Clone, Serialize)]
    pub struct ServiceConfig(pub ProtoRumor);
}

pub mod rumor_service_file {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `rumor/service_file.rs`

    use message::swim::Rumor as ProtoRumor;

    /// The service rumor
    #[derive(Debug, Clone, Serialize)]
    pub struct ServiceFile(pub ProtoRumor);
}

pub mod rumor_service {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `rumor/service.rs`

    use message::swim::Rumor as ProtoRumor;

    /// The service rumor
    #[derive(Debug, Clone, Serialize)]
    pub struct Service(pub ProtoRumor);
}
