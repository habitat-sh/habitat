// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

/// Maximum allowed size for a file to be uploaded to a service (in bytes).
pub const MAX_FILE_PUT_SIZE_BYTES: usize = 64 * 1024;
/// Maximum allowed size for a configuration to be applied to a service (in bytes).
pub const MAX_SVC_CFG_SIZE: usize = 64 * 1024;
