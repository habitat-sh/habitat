# Habitat Builder Vagrant Setup

## Prerequisites

### A working hab CLI setup on the host

The vagrant setup bind mounts a couple of directories into the VM.
`~/.hab/etc` is one of the directories and contains your hab CLI settings
(origin and GitHub API key) which will be used to correctly setup your
direnv (see `/src/.envrc`).

### GitHub App

1. Create a GitHub Organization
1. [Setup a GitHub application](https://github.com/settings/apps/new) for your GitHub organization.
1. Set the value of `Homepage URL` to `http://${APP_HOSTNAME}`
1. Set the value of `User authorization callback URL` to `http://${APP_HOSTNAME}/`
1. Set the value of `Webhook URL` to `http://${APP_HOSTNAME}` (this doesn't really matter for a local setup behind NAT where webhooks don't work out of the box)
1. Set the value of `Webhook secret` (this doesn't really matter for a local setup behind NAT where webhooks don't work out of the box)
1. Set everything to read only (this is only used for your org so it's safe)
1. Save and download the PEM key
1. Copy the PEM key to `${HABITAT_SRC_ROOT}/.secrets/builder-github-app.pem`
1. Record the the `Client ID`, `Client secret`, app `ID` (found under "About") as well as "Public link". These will be used for the `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`, `GITHUP_APP_ID` and `github_app_url` build variables (seen below). The data can be found on the app page, i.e. `https://github.com/organizations/YOUR_ORG/settings/apps/YOUR_APP`.

TODO(schu): explain the GitHub configuration in more detail or point to it.


### Prepare the `.secrets` directory

`habitat/.secrets` is used to store and provide user specific configuration
to the provision scripts. Two files are expected there:

First, the secret key of your GitHub App. It should be named `builder-github-app.pem`.

Second. an environment file `habitat-env` that exports required configuration
values. Example:

```
export APP_HOSTNAME=localhost:3000
export GITHUB_API_URL=https://api.github.com
export GITHUB_WEB_URL=https://github.com
export GITHUB_APP_ID=5629
export GITHUB_CLIENT_ID=Iv1.732260b62f84db15
export GITHUB_CLIENT_SECRET=fc7654ed8c65ccfe014cd339a55e3538f935027a
export WORKER_AUTH_TOKEN=fc7654ed8c65ccfe014cd339a55e3538f935027a
export GITHUB_ADMIN_TEAM=1995301
export GITHUB_WORKER_TEAM=2555389
export GITHUB_WEBHOOK_SECRET=58d4afaf5e5617ab0f8c39e505605e78a054d003
```

You should now have two files in the `.secrets` directory with the exact
names as shown below:

```
$ ls .secrets/
builder-github-app.pem  habitat-env
```

### Configure the web frontend

Create a file `components/builder-web/habitat.conf.js` from
`components/builder-web/habitat.conf.sample.js` and change the following
settings according to your environment:

* `github_client_id`: should be set to the same value as `GITHUB_CLIENT_ID`
  in the `habitat-env` file.
* `github_app_id`: should be set to the same value as `GITHUB_APP_ID`
  in the `habitat-env` file.
* `github_app_url`: should be set to the public page of your GitHub App.

## Setup

From project root run:

```
vagrant destroy -f
vagrant up
vagrant ssh
```

Then, in the VM:

```
sudo su -
cd /src
direnv allow .
make build-bin build-srv
make bldr-run-no-build
```

Wait until the web interface fully started. This can take a few minutes:

```
[...]
web.1       |  ----------------------------------
web.1       |        Local: http://localhost:3000
web.1       |     External: http://10.0.2.15:3000
web.1       |  ----------------------------------
web.1       |           UI: http://localhost:3001
web.1       |  UI External: http://10.0.2.15:3001
web.1       |  ----------------------------------
```

Now go to http://localhost:3000/#/pkgs and click Sign-in. You should be
redirected to GitHub and asked if you allow the GitHub app for you account. The
name differs between installations and should match what you set above. After
accepting, you shold be redirected to Habitat Builder (i.e. localhost:3000) and
be logged-in.

Finally, you should now be able to create an origin, connect a plan from
GitHub, etc.

## Building a package

To be able to build a package, your Vagrant instance needs to provide the
required `core/...` packages (since it's an independent Habitat instance).

First, create an origin `core` for that.

Second, for each required package, download it from upstream Habitat Builder
(or build it yourself, if necessary) and upload the package to your local
instance. Make sure `$HAB_BLDR_URL` is **not** set in order to install from
upstream. Example for `core/hab-backline`, which is always needed:

```
test -z "$HAB_BLDR_URL" || echo "\$HAB_BLDR_URL is set, unset it first"
hab pkg install core/hab-backline
hab pkg upload --url http://localhost:9636/v1 --auth "${HAB_AUTH_TOKEN}" "/hab/cache/artifacts/core-hab-backline-0.40.0-20171128175957-x86_64-linux.hart" --channel stable
# The package + all dependencies will be uploaded to your *local*
# core origin
```

Now, trigger a new build. For a package with no dependencies, above should
be enough. Otherwise, repeat the process for every package reported
missing during the build.

If the build fails due to a missing public key, make sure you have both
a public and a private key in the `/home/krangschnak/.hab/cache/keys/`
directory, e.g.

```
cp /hab/cache/keys/foo-20171103084851.* /home/krangschnak/.hab/cache/keys/
```

TODO(schu): fix the weird key setup above ^

## Troubleshooting

* If you experience authentication failures, make sure the direnv is
  setup correctly and `HAB_AUTH_TOKEN` as well as `HAB_ORIGIN` set
  correctly.
* Sometimes you will encounter an npm error, e.g.
  ```
	web.1       | sh: 1: concurrently: not found
	web.1       | npm ERR! file sh
	web.1       | npm ERR! code ELIFECYCLE
	web.1       | npm ERR! errno ENOENT
	web.1       | npm ERR! syscall spawn
	web.1       | npm ERR! habitat@0.8.0 build: `concurrently "npm run build-js" "npm run build-css"`
	web.1       | npm ERR! spawn ENOENT
  ```
  That could be a spurious error, try again (`make bldr-run-no-build`)
