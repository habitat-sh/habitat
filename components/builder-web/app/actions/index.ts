// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import * as gitHubActions from "./gitHub";
import * as notificationActions from "./notifications";
import * as orgActions from "./orgs";
import * as originActions from "./origins";
import * as packageActions from "./packages";
import * as projectActions from "./projects";
import * as routerActions from "./router";
import * as usersActions from "./users";

// Action types
export const LOAD_SESSION_STATE = gitHubActions.LOAD_SESSION_STATE;
export const POPULATE_GITHUB_ORGS = gitHubActions.POPULATE_GITHUB_ORGS;
export const POPULATE_GITHUB_REPOS = gitHubActions.POPULATE_GITHUB_REPOS;
export const POPULATE_GITHUB_USER_DATA = gitHubActions.POPULATE_GITHUB_USER_DATA;
export const RESET_GITHUB_ORGS = gitHubActions.RESET_GITHUB_ORGS;
export const RESET_GITHUB_REPOS = gitHubActions.RESET_GITHUB_REPOS;
export const SET_GITHUB_ORGS_LOADING_FLAG =
    gitHubActions.SET_GITHUB_ORGS_LOADING_FLAG;
export const SET_GITHUB_REPOS_LOADING_FLAG =
    gitHubActions.SET_GITHUB_REPOS_LOADING_FLAG;
export const SET_GITHUB_AUTH_STATE = gitHubActions.SET_GITHUB_AUTH_STATE;
export const SET_GITHUB_AUTH_TOKEN = gitHubActions.SET_GITHUB_AUTH_TOKEN;
export const SET_SELECTED_GITHUB_ORG = gitHubActions.SET_SELECTED_GITHUB_ORG;

export const ADD_NOTIFICATION = notificationActions.ADD_NOTIFICATION;
export const REMOVE_NOTIFICATION = notificationActions.REMOVE_NOTIFICATION;

export const FINISH_CREATING_ORG = orgActions.FINISH_CREATING_ORG;
export const ORG_INVITATION_CREATED = orgActions.ORG_INVITATION_CREATED;
export const ORG_INVITATION_CANCELLED = orgActions.ORG_INVITATION_CANCELLED;
export const PERFORM_ORG_MEMBER_SEARCH = orgActions.PERFORM_ORG_MEMBER_SEARCH;
export const POPULATE_ORG = orgActions.POPULATE_ORG;
export const TOGGLE_MEMBER_ACTION_MENU = orgActions.TOGGLE_MEMBER_ACTION_MENU;

export const POPULATE_MY_ORIGINS = originActions.POPULATE_MY_ORIGINS;
export const SET_CURRENT_ORIGIN = originActions.SET_CURRENT_ORIGIN;
export const SET_CURRENT_ORIGIN_CREATING_FLAG =
    originActions.SET_CURRENT_ORIGIN_CREATING_FLAG;
export const SET_ORIGIN_ADDING_PRIVATE_KEY =
    originActions.SET_ORIGIN_ADDING_PRIVATE_KEY;
export const SET_ORIGIN_ADDING_PUBLIC_KEY =
    originActions.SET_ORIGIN_ADDING_PUBLIC_KEY;
export const TOGGLE_ORIGIN_PICKER = originActions.TOGGLE_ORIGIN_PICKER;

export const CLEAR_PACKAGES = packageActions.CLEAR_PACKAGES;
export const POPULATE_EXPLORE = packageActions.POPULATE_EXPLORE;
export const SET_CURRENT_PACKAGE = packageActions.SET_CURRENT_PACKAGE;
export const SET_PACKAGES_NEXT_RANGE = packageActions.SET_PACKAGES_NEXT_RANGE;
export const SET_PACKAGES_SEARCH_QUERY =
    packageActions.SET_PACKAGES_SEARCH_QUERY;
export const SET_PACKAGES_TOTAL_COUNT = packageActions.SET_PACKAGES_TOTAL_COUNT;

export const SET_VISIBLE_PACKAGES = packageActions.SET_VISIBLE_PACKAGES;

export const APPEND_TO_BUILD_LOG = projectActions.APPEND_TO_BUILD_LOG;
export const FINISH_BUILD_STREAM = projectActions.FINISH_BUILD_STREAM;
export const POPULATE_BUILDS = projectActions.POPULATE_BUILDS;
export const POPULATE_BUILD_LOG = projectActions.POPULATE_BUILD_LOG;
export const POPULATE_PROJECT = projectActions.POPULATE_PROJECT;
export const SET_CURRENT_PROJECT = projectActions.SET_CURRENT_PROJECT;
export const SET_PROJECTS = projectActions.SET_PROJECTS;

export const ROUTE_CHANGE = routerActions.ROUTE_CHANGE;
export const ROUTE_REQUESTED = routerActions.ROUTE_REQUESTED;

export const SET_SIGNING_IN_FLAG = usersActions.SET_SIGNING_IN_FLAG;
export const SIGN_IN_ATTEMPT = usersActions.SIGN_IN_ATTEMPT;
export const TOGGLE_USER_NAV_MENU = usersActions.TOGGLE_USER_NAV_MENU;

// Used by redux-reset to reset the app state
export const RESET = "RESET";

// Actions
export const authenticateWithGitHub = gitHubActions.authenticateWithGitHub;
export const fetchGitHubOrgs = gitHubActions.fetchGitHubOrgs;
export const fetchGitHubRepos = gitHubActions.fetchGitHubRepos;
export const loadSessionState = gitHubActions.loadSessionState;
export const onGitHubOrgSelect = gitHubActions.onGitHubOrgSelect;
export const onGitHubRepoSelect = gitHubActions.onGitHubRepoSelect;
export const removeSessionStorage = gitHubActions.removeSessionStorage;
export const requestGitHubAuthToken = gitHubActions.requestGitHubAuthToken;
export const setGitHubAuthState = gitHubActions.setGitHubAuthState;
export const setSelectedGitHubOrg = gitHubActions.setSelectedGitHubOrg;

export const addNotification = notificationActions.addNotification;
export const removeNotification = notificationActions.removeNotification;

export const addOrg = orgActions.addOrg;
export const cancelOrgInvitation = orgActions.cancelOrgInvitation;
export const finishAddingOrg = orgActions.finishAddingOrg;
export const inviteMemberToOrg = orgActions.inviteMemberToOrg;
export const performOrgMemberSearch = orgActions.performOrgMemberSearch;
export const toggleMemberActionMenu = orgActions.toggleMemberActionMenu;

export const createOrigin = originActions.createOrigin;
export const deleteOrigin = originActions.deleteOrigin;
export const fetchMyOrigins = originActions.fetchMyOrigins;
export const toggleOriginPicker = originActions.toggleOriginPicker;
export const setCurrentOrigin = originActions.setCurrentOrigin;
export const setOriginAddingPrivateKey = originActions.setOriginAddingPrivateKey;
export const setOriginAddingPublicKey = originActions.setOriginAddingPublicKey;

export const fetchExplore = packageActions.fetchExplore;
export const fetchPackage = packageActions.fetchPackage;
export const filterPackagesBy = packageActions.filterPackagesBy;
export const populateExplore = packageActions.populateExplore;
export const setCurrentPackage = packageActions.setCurrentPackage;
export const setPackagesSearchQuery = packageActions.setPackagesSearchQuery;
export const setVisiblePackages = packageActions.setVisiblePackages;

export const addProject = projectActions.addProject;
export const fetchBuilds = projectActions.fetchBuilds;
export const fetchProject = projectActions.fetchProject;
export const fetchProjects = projectActions.fetchProjects;
export const populateBuildLog = projectActions.populateBuildLog;
export const setCurrentProject = projectActions.setCurrentProject;

export const goHome = routerActions.goHome;
export const routeChange = routerActions.routeChange;
export const requestRoute = routerActions.requestRoute;

export const attemptSignIn = usersActions.attemptSignIn;
export const setSigningInFlag = usersActions.setSigningInFlag;
export const toggleUserNavMenu = usersActions.toggleUserNavMenu;
export const signOut = usersActions.signOut;

export function resetAppState() {
    return {
        type: RESET,
    };
}
