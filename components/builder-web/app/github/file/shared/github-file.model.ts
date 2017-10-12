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

export class GitHubFile {
  name: string;
  path: string;
  sha: string;
  url: string;
  git_url: string;
  html: string;
  repository: object;
  score: number;

  type: string;
}


/* Full model:

{
  "name": string,
  "path": string,
  "sha": string,
  "url": string,
  "git_url": string,
  "html_url": string,
  "repository": {
    "id": number,
    "name": string,
    "full_name": string,
    "owner": {
      "login": string,
      "id": number,
      "avatar_url": string,
      "gravatar_id": string,
      "url": string,
      "html_url": string,
      "followers_url": string,
      "following_url": string,
      "gists_url": string,
      "starred_url": string,
      "subscriptions_url": string,
      "organizations_url": string,
      "repos_url": string,
      "events_url": string,
      "received_events_url": string,
      "type": string,
      "site_admin": boolean
    },
    "private": boolean,
    "html_url": string,
    "description": string,
    "fork": boolean,
    "url": string,
    "forks_url": string,
    "keys_url": string,
    "collaborators_url": string,
    "teams_url": string,
    "hooks_url": string,
    "issue_events_url": string,
    "events_url": string,
    "assignees_url": string,
    "branches_url": string,
    "tags_url": string,
    "blobs_url": string,
    "git_tags_url": string,
    "git_refs_url": string,
    "trees_url": string,
    "statuses_url": string,
    "languages_url": string,
    "stargazers_url": string,
    "contributors_url": string,
    "subscribers_url": string,
    "subscription_url": string,
    "commits_url": string,
    "git_commits_url": string,
    "comments_url": string,
    "issue_comment_url": string,
    "contents_url": string,
    "compare_url": string,
    "merges_url": string,
    "archive_url": string,
    "downloads_url": string,
    "issues_url": string,
    "pulls_url": string,
    "milestones_url": string,
    "notifications_url": string,
    "labels_url": string,
    "releases_url": string,
    "deployments_url": string"
  },
  "score": number
}

*/
