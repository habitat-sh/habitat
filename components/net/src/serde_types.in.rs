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

pub mod github {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `oauth/github.rs`

    #[derive(Deserialize, Serialize)]
    pub struct Contents {
        pub name: String,
        pub path: String,
        pub sha: String,
        pub size: usize,
        pub url: String,
        pub html_url: String,
        pub git_url: String,
        pub download_url: String,
        pub content: String,
        pub encoding: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Repo {
        pub id: u64,
        pub name: String,
        pub full_name: String,
        pub owner: User,
        pub private: bool,
        pub html_url: String,
        pub description: Option<String>,
        pub fork: bool,
        pub url: String,
        pub forks_url: String,
        pub keys_url: String,
        pub collaborators_url: String,
        pub teams_url: String,
        pub hooks_url: String,
        pub issue_events_url: String,
        pub events_url: String,
        pub assignees_url: String,
        pub branches_url: String,
        pub tags_url: String,
        pub blobs_url: String,
        pub git_tags_url: String,
        pub git_refs_url: String,
        pub trees_url: String,
        pub statuses_url: String,
        pub languages_url: String,
        pub stargazers_url: String,
        pub contributors_url: String,
        pub subscribers_url: String,
        pub subscription_url: String,
        pub commits_url: String,
        pub git_commits_url: String,
        pub comments_url: String,
        pub issue_comment_url: String,
        pub contents_url: String,
        pub compare_url: String,
        pub merges_url: String,
        pub archive_url: String,
        pub downloads_url: String,
        pub issues_url: String,
        pub pulls_url: String,
        pub milestones_url: String,
        pub notifications_url: String,
        pub labels_url: String,
        pub releases_url: String,
        pub deployments_url: String,
        pub created_at: String,
        pub updated_at: String,
        pub pushed_at: String,
        pub git_url: String,
        pub ssh_url: String,
        pub clone_url: String,
        pub svn_url: String,
        pub homepage: Option<String>,
        pub size: u32,
        pub stargazers_count: u32,
        pub watchers_count: u32,
        pub language: String,
        pub has_issues: bool,
        pub has_downloads: bool,
        pub has_wiki: bool,
        pub has_pages: bool,
        pub forks_count: u32,
        pub mirror_url: Option<String>,
        pub open_issues_count: u32,
        pub forks: u32,
        pub open_issues: u32,
        pub watchers: u32,
        pub default_branch: String,
        pub permissions: Permissions,
        pub organization: Option<Organization>,
        pub network_count: u32,
        pub subscribers_count: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Organization {
        pub login: String,
        pub id: u64,
        pub avatar_url: String,
        pub url: String,
        pub company: Option<String>,
        pub description: Option<String>,
        pub gravatar_id: Option<String>,
        pub hooks_url: Option<String>,
        pub html_url: Option<String>,
        pub followers_url: Option<String>,
        pub following_url: Option<String>,
        pub gists_url: Option<String>,
        pub starred_url: Option<String>,
        pub subscriptions_url: Option<String>,
        pub organizations_url: Option<String>,
        pub repos_url: Option<String>,
        pub events_url: Option<String>,
        pub received_events_url: Option<String>,
        pub members_url: Option<String>,
        pub site_admin: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Permissions {
        pub admin: bool,
        pub push: bool,
        pub pull: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Team {
        pub id: u64,
        pub url: String,
        pub name: String,
        pub slug: String,
        pub description: Option<String>,
        pub privacy: String,
        pub permission: String,
        pub members_url: String,
        pub repositories_url: String,
        pub members_count: u64,
        pub repos_count: u64,
        pub organization: Organization,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        pub login: String,
        pub id: u64,
        pub avatar_url: String,
        pub gravatar_id: String,
        pub url: String,
        pub html_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub organizations_url: String,
        pub repos_url: String,
        pub events_url: String,
        pub received_events_url: String,
        pub site_admin: bool,
        pub name: Option<String>,
        pub company: Option<String>,
        pub blog: Option<String>,
        pub location: Option<String>,
        pub email: Option<String>,
        pub hireable: Option<bool>,
        pub bio: Option<String>,
        pub public_repos: Option<u32>,
        pub public_gists: Option<u32>,
        pub followers: Option<u32>,
        pub following: Option<u32>,
        pub created_at: Option<String>,
        pub updated_at: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Email {
        pub email: String,
        pub primary: bool,
        pub verified: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AuthOk {
        pub access_token: String,
        pub scope: String,
        pub token_type: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct AuthErr {
        pub error: String,
        pub error_description: String,
        pub error_uri: String,
    }

    #[derive(Deserialize, Serialize)]
    pub enum AuthResp {
        AuthOk,
        AuthErr,
    }
}
