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

import * as gitHubActions from "./gitHub";
import * as buildActions from "./builds";
import * as notificationActions from "./notifications";
import * as originActions from "./origins";
import * as packageActions from "./packages";
import * as projectActions from "./projects";
import * as routerActions from "./router";
import * as usersActions from "./users";
import * as uiActions from "./ui";
import * as cookieActions from "./cookies";
import * as featureFlagActions from "./feature-flags";

// Action types
export const CLEAR_GITHUB_FILES = gitHubActions.CLEAR_GITHUB_FILES;
export const CLEAR_GITHUB_INSTALLATIONS = gitHubActions.CLEAR_GITHUB_INSTALLATIONS;
export const CLEAR_GITHUB_REPOS = gitHubActions.CLEAR_GITHUB_REPOS;
export const LOAD_SESSION_STATE = gitHubActions.LOAD_SESSION_STATE;
export const POPULATE_GITHUB_FILES = gitHubActions.POPULATE_GITHUB_FILES;
export const POPULATE_GITHUB_INSTALLATIONS = gitHubActions.POPULATE_GITHUB_INSTALLATIONS;
export const POPULATE_GITHUB_INSTALLATION_REPOSITORIES = gitHubActions.POPULATE_GITHUB_INSTALLATION_REPOSITORIES;
export const POPULATE_GITHUB_REPOS = gitHubActions.POPULATE_GITHUB_REPOS;
export const POPULATE_GITHUB_USER_DATA = gitHubActions.POPULATE_GITHUB_USER_DATA;
export const SET_GITHUB_ORGS_LOADING_FLAG = gitHubActions.SET_GITHUB_ORGS_LOADING_FLAG;
export const SET_GITHUB_REPOS_LOADING_FLAG = gitHubActions.SET_GITHUB_REPOS_LOADING_FLAG;
export const SET_GITHUB_AUTH_STATE = gitHubActions.SET_GITHUB_AUTH_STATE;
export const SET_GITHUB_AUTH_TOKEN = gitHubActions.SET_GITHUB_AUTH_TOKEN;
export const SET_SELECTED_GITHUB_ORG = gitHubActions.SET_SELECTED_GITHUB_ORG;

export const CLEAR_BUILD = buildActions.CLEAR_BUILD;
export const CLEAR_BUILD_LOG = buildActions.CLEAR_BUILD_LOG;
export const CLEAR_BUILDS = buildActions.CLEAR_BUILDS;
export const POPULATE_BUILD = buildActions.POPULATE_BUILD;
export const POPULATE_BUILDS = buildActions.POPULATE_BUILDS;
export const POPULATE_BUILD_LOG = buildActions.POPULATE_BUILD_LOG;
export const STREAM_BUILD_LOG = buildActions.STREAM_BUILD_LOG;

export const ADD_NOTIFICATION = notificationActions.ADD_NOTIFICATION;
export const REMOVE_NOTIFICATION = notificationActions.REMOVE_NOTIFICATION;

export const CLEAR_MY_ORIGINS = originActions.CLEAR_MY_ORIGINS;
export const CLEAR_MY_ORIGIN_INVITATIONS = originActions.CLEAR_MY_ORIGIN_INVITATIONS;
export const CLEAR_DOCKER_INTEGRATIONS = originActions.CLEAR_DOCKER_INTEGRATIONS;
export const POPULATE_MY_ORIGINS = originActions.POPULATE_MY_ORIGINS;
export const SET_PACKAGE_COUNT_FOR_ORIGIN = originActions.SET_PACKAGE_COUNT_FOR_ORIGIN;
export const POPULATE_MY_ORIGIN_INVITATIONS = originActions.POPULATE_MY_ORIGIN_INVITATIONS;
export const POPULATE_ORIGIN_INVITATIONS = originActions.POPULATE_ORIGIN_INVITATIONS;
export const POPULATE_ORIGIN_MEMBERS = originActions.POPULATE_ORIGIN_MEMBERS;
export const POPULATE_ORIGIN_PUBLIC_KEYS = originActions.POPULATE_ORIGIN_PUBLIC_KEYS;
export const POPULATE_ORIGIN_DOCKER_INTEGRATIONS = originActions.POPULATE_ORIGIN_DOCKER_INTEGRATIONS;
export const SET_CURRENT_ORIGIN = originActions.SET_CURRENT_ORIGIN;
export const SET_CURRENT_ORIGIN_CREATING_FLAG = originActions.SET_CURRENT_ORIGIN_CREATING_FLAG;
export const SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY = originActions.SET_CURRENT_ORIGIN_ADDING_PRIVATE_KEY;
export const SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY = originActions.SET_CURRENT_ORIGIN_ADDING_PUBLIC_KEY;
export const SET_CURRENT_ORIGIN_LOADING = originActions.SET_CURRENT_ORIGIN_LOADING;
export const SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE = originActions.SET_ORIGIN_PRIVATE_KEY_UPLOAD_ERROR_MESSAGE;
export const SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE = originActions.SET_ORIGIN_PUBLIC_KEY_UPLOAD_ERROR_MESSAGE;
export const SET_ORIGIN_USER_INVITE_ERROR_MESSAGE = originActions.SET_ORIGIN_USER_INVITE_ERROR_MESSAGE;
export const SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE = originActions.SET_ORIGIN_INTEGRATION_SAVE_ERROR_MESSAGE;
export const TOGGLE_ORIGIN_PICKER = originActions.TOGGLE_ORIGIN_PICKER;
export const UPDATE_ORIGIN = originActions.UPDATE_ORIGIN;

export const CLEAR_PACKAGES = packageActions.CLEAR_PACKAGES;
export const CLEAR_LATEST_IN_CHANNEL = packageActions.CLEAR_LATEST_IN_CHANNEL;
export const CLEAR_LATEST_PACKAGE = packageActions.CLEAR_LATEST_PACKAGE;
export const POPULATE_DASHBOARD_RECENT = packageActions.POPULATE_DASHBOARD_RECENT;
export const CLEAR_PACKAGE_VERSIONS = packageActions.CLEAR_PACKAGE_VERSIONS;
export const POPULATE_EXPLORE = packageActions.POPULATE_EXPLORE;
export const POPULATE_EXPLORE_STATS = packageActions.POPULATE_EXPLORE_STATS;
export const SET_CURRENT_PACKAGE = packageActions.SET_CURRENT_PACKAGE;
export const SET_CURRENT_PACKAGE_VERSIONS = packageActions.SET_CURRENT_PACKAGE_VERSIONS;
export const SET_LATEST_IN_CHANNEL = packageActions.SET_LATEST_IN_CHANNEL;
export const SET_LATEST_PACKAGE = packageActions.SET_LATEST_PACKAGE;
export const SET_PACKAGES_NEXT_RANGE = packageActions.SET_PACKAGES_NEXT_RANGE;
export const SET_PACKAGES_SEARCH_QUERY = packageActions.SET_PACKAGES_SEARCH_QUERY;
export const SET_PACKAGES_TOTAL_COUNT = packageActions.SET_PACKAGES_TOTAL_COUNT;

export const SET_VISIBLE_PACKAGES = packageActions.SET_VISIBLE_PACKAGES;

export const CLEAR_CURRENT_PROJECT = projectActions.CLEAR_CURRENT_PROJECT;
export const CLEAR_CURRENT_PROJECT_INTEGRATION = projectActions.CLEAR_CURRENT_PROJECT_INTEGRATION;
export const SET_CURRENT_PROJECT = projectActions.SET_CURRENT_PROJECT;
export const SET_CURRENT_PROJECT_INTEGRATION = projectActions.SET_CURRENT_PROJECT_INTEGRATION;
export const SET_PROJECTS = projectActions.SET_PROJECTS;
export const DELETE_PROJECT = projectActions.DELETE_PROJECT;

export const ROUTE_CHANGE = routerActions.ROUTE_CHANGE;
export const ROUTE_REQUESTED = routerActions.ROUTE_REQUESTED;
export const SET_REDIRECT_ROUTE = routerActions.SET_REDIRECT_ROUTE;
export const RESET_REDIRECT_ROUTE = routerActions.RESET_REDIRECT_ROUTE;

export const SIGN_IN_ATTEMPT = usersActions.SIGN_IN_ATTEMPT;
export const SET_SIGNING_IN_FLAG = usersActions.SET_SIGNING_IN_FLAG;
export const TOGGLE_USER_NAV_MENU = usersActions.TOGGLE_USER_NAV_MENU;

export const SET_LAYOUT = uiActions.SET_LAYOUT;

export const SET_COOKIE = cookieActions.SET_COOKIE;
export const REMOVE_COOKIE = cookieActions.REMOVE_COOKIE;

export const SET_FEATURE_FLAG = featureFlagActions.SET_FEATURE_FLAG;
export const SET_FEATURE_FLAGS = featureFlagActions.SET_FEATURE_FLAGS;

// Used by redux-reset to reset the app state
export const RESET = "RESET";

// Actions
export const authenticateWithGitHub = gitHubActions.authenticateWithGitHub;
export const fetchGitHubFiles = gitHubActions.fetchGitHubFiles;
export const fetchGitHubInstallations = gitHubActions.fetchGitHubInstallations;
export const fetchGitHubInstallationRepositories = gitHubActions.fetchGitHubInstallationRepositories;
export const loadSessionState = gitHubActions.loadSessionState;
export const removeSessionStorage = gitHubActions.removeSessionStorage;
export const requestGitHubAuthToken = gitHubActions.requestGitHubAuthToken;
export const setGitHubAuthState = gitHubActions.setGitHubAuthState;
export const setSelectedGitHubOrg = gitHubActions.setSelectedGitHubOrg;

export const clearBuild = buildActions.clearBuild;
export const clearBuilds = buildActions.clearBuilds;
export const fetchBuild = buildActions.fetchBuild;
export const fetchBuildLog = buildActions.fetchBuildLog;
export const fetchBuilds = buildActions.fetchBuilds;
export const streamBuildLog = buildActions.streamBuildLog;
export const submitJob = buildActions.submitJob;

export const addNotification = notificationActions.addNotification;
export const removeNotification = notificationActions.removeNotification;

export const acceptOriginInvitation = originActions.acceptOriginInvitation;
export const createOrigin = originActions.createOrigin;
export const deleteOriginInvitation = originActions.deleteOriginInvitation;
export const deleteOriginMember = originActions.deleteOriginMember;
export const deleteDockerIntegration = originActions.deleteDockerIntegration;
export const ignoreOriginInvitation = originActions.ignoreOriginInvitation;
export const fetchDockerIntegration = originActions.fetchDockerIntegration;
export const fetchOrigin = originActions.fetchOrigin;
export const fetchOriginInvitations = originActions.fetchOriginInvitations;
export const fetchOriginMembers = originActions.fetchOriginMembers;
export const fetchOriginPublicKeys = originActions.fetchOriginPublicKeys;
export const fetchMyOrigins = originActions.fetchMyOrigins;
export const fetchMyOriginInvitations = originActions.fetchMyOriginInvitations;
export const generateOriginKeys = originActions.generateOriginKeys;
export const inviteUserToOrigin = originActions.inviteUserToOrigin;
export const toggleOriginPicker = originActions.toggleOriginPicker;
export const setCurrentOrigin = originActions.setCurrentOrigin;
export const updateOrigin = originActions.updateOrigin;
export const uploadOriginPrivateKey = originActions.uploadOriginPrivateKey;
export const uploadOriginPublicKey = originActions.uploadOriginPublicKey;
export const setDockerIntegration = originActions.setDockerIntegration;

export const fetchDashboardRecent = packageActions.fetchDashboardRecent;
export const fetchExplore = packageActions.fetchExplore;
export const fetchPackage = packageActions.fetchPackage;
export const fetchLatestInChannel = packageActions.fetchLatestInChannel;
export const fetchLatestPackage = packageActions.fetchLatestPackage;
export const fetchPackageVersions = packageActions.fetchPackageVersions;
export const filterPackagesBy = packageActions.filterPackagesBy;
export const populateExplore = packageActions.populateExplore;
export const populateExploreStats = packageActions.populateExploreStats;
export const setCurrentPackage = packageActions.setCurrentPackage;
export const setPackagesSearchQuery = packageActions.setPackagesSearchQuery;
export const setVisiblePackages = packageActions.setVisiblePackages;
export const getUniquePackages = packageActions.getUniquePackages;

export const addProject = projectActions.addProject;
export const fetchProject = projectActions.fetchProject;
export const fetchProjects = projectActions.fetchProjects;
export const fetchProjectIntegration = projectActions.fetchProjectIntegration;
export const setCurrentProject = projectActions.setCurrentProject;
export const setProjectIntegrationSettings = projectActions.setProjectIntegrationSettings;
export const deleteProject = projectActions.deleteProject;
export const updateProject = projectActions.updateProject;

export const goHome = routerActions.goHome;
export const routeChange = routerActions.routeChange;
export const requestRoute = routerActions.requestRoute;
export const setRedirectRoute = routerActions.setRedirectRoute;
export const resetRedirectRoute = routerActions.resetRedirectRoute;

export const setSigningInFlag = usersActions.setSigningInFlag;
export const attemptSignIn = usersActions.attemptSignIn;
export const toggleUserNavMenu = usersActions.toggleUserNavMenu;
export const signOut = usersActions.signOut;

export const setLayout = uiActions.setLayout;

export const getCookie = cookieActions.getCookie;
export const setCookie = cookieActions.setCookie;
export const removeCookie = cookieActions.removeCookie;

export const loadFeatureFlags = featureFlagActions.loadFeatureFlags;
export const setFeatureFlag = featureFlagActions.setFeatureFlag;
export const setFeatureFlags = featureFlagActions.setFeatureFlags;

export function resetAppState() {
    return {
        type: RESET,
    };
}
