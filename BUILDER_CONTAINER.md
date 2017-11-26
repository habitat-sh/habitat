# Builder Docker Container

> *Warning: FOR DEVELOPMENT PURPOSES ONLY. USE AT OWN RISK. NO SUPPORT WILL BE GRANTED.*

## Pre-Requisites

### GitHub OAuth Application

1. Create a GitHub Organization
1. [Setup an OAuth application](https://developer.github.com/apps/building-integrations/setting-up-and-registering-oauth-apps/registering-oauth-apps) for your GitHub organization.
1. Set the value of `Homepage URL` to `http://${APP_HOSTNAME}`
1. Set the value of `Authorization callback URL` to `http://${APP_HOSTNAME}/#/sign-in`
1. Record the the client-id and client-secret. These will be used for the `GITHUB_CLIENT_ID` and `GITHUB_CLIENT_SECRET` build variables (seen below).

### Setup GitHub Teams

1. Create a GitHub team in your organization for bots and retrieve the team-id. This will be used for the value of `GITHUB_WORKER_TEAM`. You can retrieve the team-id with curl and an authorization token: `curl -H "Authorization: token <personal access token here>" https://api.github.com/orgs/orgName/teams`
1. Create a GitHub team in your organization for administrators and retrieve the team-id. This will be used for the value of `GITHUB_ADMIN_TEAM`

> note: The only way to retrieve the ID for a GitHub team is currently [by the API](https://github.com/rei/github-api-utils/blob/master/bash/get-team-id.sh)

### Setup Worker GitHub User

1. Create a new "bot" user
1. Add the user to the GitHub bot team created in the previous step
1. [Create a Personal Access Token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line) for the new user. This will be used to configure the `WORKER_AUTH_TOKEN` build variable which gives the build worker access to publish packages to the depot.

## Building

* `APP_HOSTNAME` - Builder's addressable hostname. This is used for redirection back after authenticating with GitHub and proxying requests to the appropriate http gateway _(default: "localhost")_.
* `GITHUB_API_URL` - external GitHub or GitHub Enterprise API endpoint Builder will connect to _(default: "https://api.github.com")_.
* `GITHUB_WEB_URL` - external GitHub or GitHub Enterprise web URL endpoint Builder will connect to _(default: "https://github.com")_
* `GITHUB_CLIENT_ID` - GitHub OAuth application client-id *required*.
* `GITHUB_CLIENT_SECRET` - GitHub OAuth application client-secret *required*.
* `GITHUB_ADMIN_TEAM` - GitHub Team ID to grant admin gateway access to *required*.
* `GITHUB_WORKER_TEAM` - GitHub Team ID to grant worker publishing access to *required*.
* `WORKER_AUTH_TOKEN` - GitHub personal access token for authenticating worker publish requests *required*.

```bash
$ cd ${root}
$ docker build --no-cache -f BLDR-Dockerfile --build-arg GITHUB_CLIENT_ID=02fb7d9e1fde99e8d395 --build-arg GITHUB_CLIENT_SECRET=8d7611da338fd741330804ec470236da58ffdd68 --build-arg GITHUB_ADDR=10.48.236.67 --build-arg GITHUB_API_URL=https://10.48.236.67/api/v3 --build-arg GITHUB_WEB_URL=https://10.48.236.67 --build-arg WORKER_AUTH_TOKEN=e96301212f26a8da99925d04fb5857e869792a47 --build-arg GITHUB_ADMIN_TEAM=0 --build-arg GITHUB_WORKER_TEAM=0 -t habitat/builder .
```

## Running

```
$ docker run -p 80:80 -p 443:443 -p 9631:9631 -p 9638:9638 --privileged --name builder habitat/builder
```
