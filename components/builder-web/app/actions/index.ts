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

export {
  CLEAR_BUILD_LOG,
  CLEAR_BUILD,
  CLEAR_BUILDS,
  clearBuild,
  clearBuilds,
  fetchBuild,
  fetchBuildLog,
  fetchBuilds,
  POPULATE_BUILD_LOG,
  POPULATE_BUILD,
  POPULATE_BUILDS,
  STREAM_BUILD_LOG,
  streamBuildLog,
  submitJob
} from './builds';

export {
  getCookie,
  SET_COOKIE,
  setCookie,
  REMOVE_COOKIE,
  removeCookie
} from './cookies';

export {
  authenticateWithGitHub,
  CLEAR_GITHUB_INSTALLATIONS,
  fetchGitHubInstallations,
  LOAD_GITHUB_SESSION_STATE,
  loadGitHubSessionState,
  POPULATE_GITHUB_INSTALLATIONS,
  POPULATE_GITHUB_USER_DATA,
  removeSessionStorage,
  requestGitHubAuthToken,
  SET_GITHUB_AUTH_STATE,
  SET_GITHUB_AUTH_TOKEN,
  setGitHubAuthState
} from './gitHub';

export {
  ADD_NOTIFICATION,
  addNotification,
  REMOVE_NOTIFICATION,
  removeNotification
} from './notifications';

export {
  acceptOriginInvitation,
  CLEAR_DOCKER_INTEGRATIONS,
  CLEAR_MY_ORIGIN_INVITATIONS,
  CLEAR_MY_ORIGINS,
  clearIntegrationCredsValidation,
  createOrigin,
  deleteDockerIntegration,
  deleteOriginInvitation,
  deleteOriginMember,
  fetchDockerIntegration,
  fetchMyOriginInvitations,
  fetchMyOrigins,
  fetchOrigin,
  fetchOriginInvitations,
  fetchOriginMembers,
  fetchOriginPublicKeys,
  generateOriginKeys,
  ignoreOriginInvitation,
  inviteUserToOrigin,
  POPULATE_MY_ORIGIN_INVITATIONS,
  POPULATE_MY_ORIGINS,
  POPULATE_ORIGIN_DOCKER_INTEGRATIONS,
  POPULATE_ORIGIN_INVITATIONS,
  POPULATE_ORIGIN_MEMBERS,
  POPULATE_ORIGIN_PUBLIC_KEYS,
  SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY,
  SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY,
  SET_CURRENT_ORIGIN_CREATING_FLAG,
  SET_CURRENT_ORIGIN_LOADING,
  SET_CURRENT_ORIGIN,
  SET_INTEGRATION_CREDS_VALIDATION,
  SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE,
  SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE,
  SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE,
  SET_ORIGIN_USER_INVITE_ERROR_MESSAGE,
  SET_PACKAGE_COUNT_FOR_ORIGIN,
  setCurrentOrigin,
  setDockerIntegration,
  TOGGLE_ORIGIN_PICKER,
  toggleOriginPicker,
  UPDATE_ORIGIN,
  updateOrigin,
  uploadOriginPrivateKey,
  uploadOriginPublicKey,
  validateDockerCredentials
} from './origins';

export {
  CLEAR_LATEST_IN_CHANNEL,
  CLEAR_LATEST_PACKAGE,
  CLEAR_PACKAGE_VERSIONS,
  CLEAR_PACKAGES,
  fetchDashboardRecent,
  fetchExplore,
  fetchLatestInChannel,
  fetchLatestPackage,
  fetchPackage,
  fetchPackageVersions,
  filterPackagesBy,
  getUniquePackages,
  POPULATE_DASHBOARD_RECENT,
  POPULATE_EXPLORE_STATS,
  POPULATE_EXPLORE,
  populateExplore,
  populateExploreStats,
  SET_CURRENT_PACKAGE_VERSIONS,
  SET_CURRENT_PACKAGE,
  SET_LATEST_IN_CHANNEL,
  SET_LATEST_PACKAGE,
  SET_PACKAGES_NEXT_RANGE,
  SET_PACKAGES_SEARCH_QUERY,
  SET_PACKAGES_TOTAL_COUNT,
  SET_VISIBLE_PACKAGES,
  setCurrentPackage,
  setPackagesSearchQuery,
  setVisiblePackages
} from './packages';

export {
  addProject,
  CLEAR_PROJECTS,
  CLEAR_CURRENT_PROJECT,
  CLEAR_CURRENT_PROJECT_INTEGRATION,
  deleteProject,
  fetchProject,
  fetchProjects,
  SET_CURRENT_PROJECT_INTEGRATION,
  SET_CURRENT_PROJECT,
  SET_PROJECTS,
  setCurrentProject,
  setProjectIntegrationSettings,
  setProjectVisibility,
  updateProject
} from './projects';

export {
  goHome,
  requestRoute,
  RESET_REDIRECT_ROUTE,
  resetRedirectRoute,
  ROUTE_CHANGE,
  ROUTE_REQUESTED,
  routeChange,
  SET_REDIRECT_ROUTE,
  setRedirectRoute
} from './router';

export {
  SET_LAYOUT,
  setLayout
} from './ui';

export {
  attemptSignIn,
  fetchProfile,
  POPULATE_PROFILE,
  saveProfile,
  SET_PRIVILEGES,
  SET_SIGNING_IN_FLAG,
  setPrivileges,
  setSigningInFlag,
  SIGN_IN_ATTEMPT,
  signOut,
  TOGGLE_USER_NAV_MENU,
  toggleUserNavMenu
} from './users';

export {
  LOAD_BLDR_SESSION_STATE,
  loadBldrSessionState,
  SET_BLDR_SESSION_TOKEN
} from './sessions';

// Used by redux-reset to reset the app state
export const RESET = 'RESET';

export function resetAppState() {
  return {
    type: RESET,
  };
}
