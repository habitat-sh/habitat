// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::fmt;

use base64;

use error::{HubError, HubResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub id: u32,
    pub owner: AppOwner,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct AppInstallationToken {
    pub token: String,
    pub expires_at: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppAuthErr {
    pub message: String,
    pub documentation_url: String,
}

impl fmt::Display for AppAuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "message={}, documentation_url={}",
            self.message,
            self.documentation_url
        )
    }
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

impl fmt::Display for AuthErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "err={}, desc={}, uri={}",
            self.error,
            self.error_description,
            self.error_uri
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppOwner {
    pub login: String,
    pub id: u32,
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
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
}

#[derive(Debug, Deserialize, Serialize)]
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

impl Contents {
    pub fn decode(&self) -> HubResult<Vec<u8>> {
        base64::decode(&self.content).map_err(HubError::ContentDecode)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubWebhookPush {
    /// The full Git ref that was pushed. Example: "refs/heads/master"
    #[serde(rename = "ref")]
    pub git_ref: String,
    /// The SHA of the most recent commit on ref before the push
    pub before: String,
    /// The SHA of the most recent commit on ref after the push
    pub after: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Option<String>,
    pub compare: String,
    /// An array of commit objects describing the pushed commits (The array includes a maximum
    /// of 20 commits. If necessary, you can use the Commits API to fetch additional commits.
    /// This limit is applied to timeline events only and isn't applied to webhook deliveries)
    pub commits: Vec<GitHubWebhookCommit>,
    pub head_commit: Option<GitHubWebhookCommit>,
    pub repository: PushRepository,
    pub pusher: GitHubOwner,
    pub sender: GitHubWebhookSender,
    pub installation: GitHubAppInstallation,
}

impl GitHubWebhookPush {
    pub fn changed(&self) -> Vec<&String> {
        let mut paths = vec![];
        for commit in self.commits.iter() {
            paths.extend(&commit.added);
            paths.extend(&commit.removed);
            paths.extend(&commit.modified);
        }
        paths.sort();
        paths.dedup();
        paths
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubWebhookCommit {
    pub id: String,
    pub tree_id: String,
    /// Whether this commit is distinct from any that have been pushed before
    pub distinct: bool,
    /// The commit message
    pub message: String,
    pub timestamp: String,
    /// Points to the commit API resource
    pub url: String,
    /// The git author of the commit
    pub author: GitHubAuthor,
    pub committer: GitHubAuthor,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubAuthor {
    /// Public name of commit author
    pub name: String,
    /// Public email of commit author
    pub email: String,
    /// Display name of commit author
    pub username: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GitHubAppInstallation {
    pub id: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubWebhookSender {
    pub login: String,
    pub id: u32,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
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
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GitHubOwner {
    /// Public name of commit author
    pub name: String,
    /// Public email of commit author
    pub email: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PushRepository {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub owner: Organization,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub master_branch: String,
    pub organization: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub master_branch: String,
    pub organization: Organization,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Organization {
    pub login: String,
    pub id: u32,
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

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Team {
    pub id: u32,
    pub url: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
    pub members_count: u32,
    pub repos_count: u32,
    pub organization: Organization,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u32,
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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserPlan {
    pub name: String,
    pub space: u32,
    pub private_repos: u32,
    pub collaborators: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Search {
    pub total_count: u32,
    pub incomplete_results: bool,
    pub items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchItem {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub repository: Repository,
    pub score: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamMembership {
    pub url: String,
    pub role: String,
    pub state: String,
}

impl TeamMembership {
    pub fn active(&self) -> bool {
        self.state == "active"
    }
}
