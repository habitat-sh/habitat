// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use protocol::originsrv::OriginPackageVisibility;

pub fn transition_visibility(
    incoming: OriginPackageVisibility,
    existing: OriginPackageVisibility,
) -> OriginPackageVisibility {
    match incoming { // this is the _new_ setting that we're wanting to change it to
        OriginPackageVisibility::Hidden => {
            match existing { // this is what it is currently
                OriginPackageVisibility::Hidden => OriginPackageVisibility::Hidden,    // hidden to hidden, fine
                OriginPackageVisibility::Public => OriginPackageVisibility::Hidden,    // public to hidden, fine
                OriginPackageVisibility::Private => OriginPackageVisibility::Private,  // private to hidden doesn't really make sense. keep it private
            }
        }
        OriginPackageVisibility::Public => OriginPackageVisibility::Public,            // setting anything to public is fine
        OriginPackageVisibility::Private => {
            match existing {
                OriginPackageVisibility::Private => OriginPackageVisibility::Private,  // no change, no problem
                OriginPackageVisibility::Public => OriginPackageVisibility::Hidden,    // public packages can't be made private, only hidden
                OriginPackageVisibility::Hidden => OriginPackageVisibility::Hidden,    // if it's hidden, keep it hidden
            }
        }
    }
}
