// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

export const ROUTE_CHANGE = "ROUTE_CHANGE";
export const ROUTE_REQUESTED = "ROUTE_REQUESTED";

export function goHome() {
    return dispatch => {
        dispatch(requestRoute(["Packages"]));
    };
}

export function routeChange(newRoute) {
    return {
        type: ROUTE_CHANGE,
        payload: newRoute,
    };
}

export function requestRoute(requestedRoute: Array<any>) {
    return {
        type: ROUTE_REQUESTED,
        payload: requestedRoute
    };
}
