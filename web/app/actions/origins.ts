// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

export const SET_CURRENT_ORIGIN = "SET_CURRENT_ORIGIN";

export function setCurrentOrigin(payload) {
    return {
        type: SET_CURRENT_ORIGIN,
        payload,
    };
}