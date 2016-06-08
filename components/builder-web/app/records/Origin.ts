// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {List, Record} from "immutable";

export const Origin = Record({
    id: undefined,
    name: undefined,
    owner_id: undefined,
    privateKeys: List(),
    publicKeys: List(),
});
