# Builder Docker Container

> *Warning: FOR DEVELOPMENT PURPOSES ONLY. USE AT OWN RISK. NO SUPPORT WILL BE GRANTED.*

## Pre-Requisites

### GitHub OAuth Application

1. Create a GitHub Organization
1. [Setup a GitHub application](https://github.com/settings/apps/new) for your GitHub organization.
1. Set the value of `Homepage URL` to `http://${APP_HOSTNAME}`
1. Set the value of `User authorization callback URL` to `http://${APP_HOSTNAME}/` (The trailing `/` is *important*)
1. Set the value of `Webhook URL` to `http://${APP_HOSTNAME}/`
1. Set everything to read only (this is only used for your org so it's safe)
1. Save and download the pem key
1. Copy the pem key to `${HABITAT_SRC_ROOT}/.secrets/builder-dev-app.pem`
1. Record the the client-id, client-secret and app_id. These will be used for the `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET` and `GITHUP_APP_ID` build variables (seen below).

### Setup GitHub Teams

1. Create a GitHub team in your organization for administrators and retrieve the team-id. This will be used for the value of `GITHUB_ADMIN_TEAM`

> note: The only way to retrieve the ID for a GitHub team is currently [by the API](https://github.com/rei/github-api-utils/blob/master/bash/get-team-id.sh)

## Building

* `APP_HOSTNAME` - Builder's addressable hostname. This is used for redirection back after authenticating with GitHub and proxying requests to the appropriate http gateway _(default: "localhost")_.
* `GITHUB_API_URL` - external GitHub or GitHub Enterprise API endpoint Builder will connect to _(default: "https://api.github.com")_.
* `GITHUB_WEB_URL` - external GitHub or GitHub Enterprise web URL endpoint Builder will connect to _(default: "https://github.com")_
* `GITHUB_CLIENT_ID` - GitHub OAuth application client-id *required*.
* `GITHUB_CLIENT_SECRET` - GitHub OAuth application client-secret *required*.
* `GITHUB_ADMIN_TEAM` - GitHub Team ID to grant admin gateway access to *required*.
* `GITHUB_APP_ID` - GitHub App ID to make requests from the UI *required*.

```bash
$ cd ${root}
$ docker build --no-cache \
  -f BLDR-Dockerfile \
  --build-arg GITHUB_CLIENT_ID=<your_gh_client_id> \
  --build-arg GITHUB_CLIENT_SECRET=<your_GH_client_secret> \
  --build-arg GITHUB_ADDR=<optional_GH_addr> \
  --build-arg GITHUB_API_URL=<optional_GH_api_url> \
  --build-arg GITHUB_WEB_URL=<optional_GH_web_url> \
  --build-arg GITHUB_ADMIN_TEAM=0 \
  --build-arg GITHUB_APP_ID -t habitat/builder .
```

## Running

```bash
$ docker run \
  -p 80:80 \
  -p 443:443 \
  -p 9631:9631 \
  -p 9636:9636 \
  -p 9638:9638 \
  --privileged \
  --name builder \
  habitat/builder
```
## Gotchas
This will not setup a worker as all workers must be run outside of a container/chroot.
