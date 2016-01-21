export const ROUTE_CHANGE = "ROUTE_CHANGE";
export const SIGN_IN_ATTEMPT = "SIGN_IN_ATTEMPT";
export const SIGN_UP_ATTEMPT = "SIGN_UP_ATTEMPT";
export const SIGN_OUT = "SIGN_OUT";
export const TOGGLE_USER_NAV_MENU = "TOGGLE_USER_NAV_MENU";

export function attemptSignIn(username) {
  return {
    type: SIGN_IN_ATTEMPT,
    payload: { username: username },
  };
}

export function attemptSignUp(username, email, password) {
  return {
    type: SIGN_UP_ATTEMPT,
    payload: {
      username: username,
      email: email,
      password: password,
    }
  };
}

export function routeChange(newRoute) {
  return {
    type: ROUTE_CHANGE,
    payload: newRoute,
  };
}

export function toggleUserNavMenu() {
  return {
    type: TOGGLE_USER_NAV_MENU
  };
}

export function signOut() {
  return {
    type: SIGN_OUT
  };
}
