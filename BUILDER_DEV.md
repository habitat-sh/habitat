# Builder Services Development Environment

## Overview
This document outlines the steps to start and run a Builder environment for development. The builder environment includes the builder services, as well as the depot web site.

## Pre-Reqs
1. Use a Linux OS - either native or VM.
2. Clone the habitat repo to your local filesystem. (If using a VM, you should use the local filesystem for the hab repo instead of using a mounted filesystem, otherwise things may fail later when running services).
3. Ensure you have a Github auth token. Create from the Github site if you don't have one already. Token Capabilities needed: read:org, user:email
4. The sample commands below use the 'httpie' tool. Install it if not present on your system (https://github.com/jkbrzt/httpie).
5. A recent version of the habitat cli should also be installed in your dev environment (https://www.habitat.sh/docs/get-habitat/)

## Bootstrap the OS with required packages
You need to make sure you have the required packages installed.
Run the appropriate shell script under the `support/linux` folder to install the packages.
Refer to [BUILDING.md](https://github.com/habitat-sh/habitat/blob/master/BUILDING.md) doc for the detailed steps.

## Create configuration files
Some capabilities (such as allowing builder permissions, and turning on auto-building when new packages are uploaded to the depot) require passing in a custom config to the Builder services.

Create the following files somewhere on your local filesystem (Note: the client_id and client_secret below are for development purposes only):

config_api.toml:
```
[cfg]
builds_enabled = true

[cfg.github]
url = "https://api.github.com"
client_id = "0c2f738a7d0bd300de10"
client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
```

config_sessionsrv.toml:
```
[cfg]
github_admin_team = 1995301
github_builder_teams = [1995301]
github_build_worker_teams = [1995301]

[cfg.github]
url = "https://api.github.com"
client_id = "0c2f738a7d0bd300de10"
client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
```

config_worker.toml:
```
[cfg]
auth_token = "<your github token>"
```

Now, modify the `Procfile` (located in your hab repo in the `support` folder) to point the api, sessionsrv, and worker services to the previously created config files.  Eg:

```
api: target/debug/bldr-api start --path /tmp/depot --config /home/your_alias/habitat/config_api.toml
sessionsrv: target/debug/bldr-session-srv start --config /home/your_alias/habitat/config_sessionsrv.toml
worker: target/debug/bldr-worker start --config /home/your_alias/habitat/config_worker.toml
```

## Run the Builder services
1. Open a new terminal window.
2. Create or import the private and public keys for your origin using `hab setup` or `hab origin key import` (for example, a pre-existing origin)
3. Export the following environment variables:
```
export HAB_AUTH_TOKEN=<your github token>
export HAB_DEPOT_URL=http://localhost:9636/v1/depot
export HAB_ORIGIN=<your origin>
```
4. Now, do a `make bldr-run` from the root of your hab repo.

The first time this command runs, it will create the required databases. Let it run for a while, and then re-start it if there are errors (this is normal for the first time setup).

When `make bldr-run` can proceed with all the services up and running, you are ready to proceed to the next step.

## Create your origin
1. Open a new terminal window.
2. Export the HAB_ORIGIN and HAB_AUTH_TOKEN as above
2. Create an origin in the DB by issuing the following command:

```
http POST http://localhost:9636/v1/depot/origins Content-Type:application/json Authorization:Bearer:${HAB_AUTH_TOKEN} name=${HAB_ORIGIN}
```

The response should be something like:
```
HTTP/1.1 201 Created
Access-Control-Allow-Headers: authorization, range
Access-Control-Allow-Methods: PUT, DELETE
Access-Control-Allow-Origin: *
Content-Length: 93
Content-Type: application/json
Date: Mon, 07 Nov 2016 19:55:03 GMT

{
    "id": 151409091187589122,
    "name": "core",
    "owner_id": "133508078967455744",
    "private_key_name": ""
}
```

## Import origin keys to the Depot
This can be done via the builder depot web UI. Make sure you have both the public and private keys available for your origin.

1. The `make bldr-run` should have started the web UI. If not, you can run it via a separate terminal window:
```
cd habitat/components/builder-web
npm install
npm start
```
The web UI should come up at http://localhost:3000. (If you need to bring up the UI at a different address or port, please refer to the builder-web [README](https://github.com/habitat-sh/habitat/blob/master/components/builder-web/README.md) for setting up a custom OAuth app).

2. Log into the web UI via your Github account
3. Go to the origins page, and select the your origin
4. Go to the keys tab, and follow the instructions for uploading the public and private keys.

## Create the project(s) you want to build
In order to build a package, there needs to be a project created in the database.
If the DB has been newly created, there will initially not be any projects available.

Create a project for the package you want to build:

1. Create a project file (eg, ```project.json```) on your local filesystem (see example below for core/nginx)
```
{
    "origin": "core",
    "plan_path": "nginx/plan.sh",
    "github": {
        "organization": "habitat-sh",
        "repo": "core-plans"
    }
}
```

2. Issue the following command:
```
http POST http://localhost:9636/v1/projects Authorization:Bearer:${HAB_AUTH_TOKEN} < project.json
```

The response should be something like this:
```
HTTP/1.1 201 Created
Access-Control-Allow-Headers: authorization, range
Access-Control-Allow-Methods: PUT, DELETE
Access-Control-Allow-Origin: *
Content-Length: 121
Content-Type: application/json
Date: Mon, 07 Nov 2016 20:01:52 GMT

{
    "id": "core/nginx",
    "plan_path": "nginx/plan.sh",
    "vcs": {
        "type": "git",
        "url": "https://github.com/habitat-sh/core-plans.git"
    }
}
```

Repeat the above steps for any other projects that you want to build.

## Run a build

Issue the following command (replace `origin/name` as needed):
```
http POST http://localhost:9636/v1/jobs Authorization:Bearer:${HAB_AUTH_TOKEN} project_id="<origin/name>"
```

This should create a build job, and then dispatch it to the build worker.

You should see a response similar to the following:

```
Response:
HTTP/1.1 201 Created
Access-Control-Allow-Headers: authorization, range
Access-Control-Allow-Methods: PUT, DELETE
Access-Control-Allow-Origin: *
Content-Length: 35
Content-Type: application/json
Date: Mon, 07 Nov 2016 20:06:32 GMT

{
    "id": 151414870149955584,
    "state": 0
}
```

## Other Commands
Here are some other sample commands to experiment with:

* Package Search:
`http GET http://localhost:9636/v1/depot/pkgs/search/foo Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Scheduling:
`
http POST http://localhost:9636/v1/depot/pkgs/schedule/core/nginx Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Get Scheduled Group status:
`
http GET http://localhost:9636/v1/depot/pkgs/schedule/15986865821137185538 Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Retrieve Secret Keys:
`
http GET http://app.acceptance.habitat.sh/v1/depot/origins/core/secret_keys/latest Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Retrieve Channels:
`
http GET http://localhost:9636/v1/depot/channels/core Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* List Channel Packages:
`
http GET http://localhost:9636/v1/depot/channels/core/unstable/pkgs Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Promote Package:
`http PUT http://localhost:9636/v1/depot/channels/core/unstable/pkgs/hab/0.18.0/20170302204108/promote Authorization:Bearer:${HAB_AUTH_TOKEN}
`
* Upload a Package:
`
http POST http://localhost:9636/v1/depot/pkgs/core/nginx/version/release Authorization:Bearer:${HAB_AUTH_TOKEN}
`
or
`
hab pkg upload -u http://localhost:9636/v1/depot ./core-hab-0.18.0-20170302204108-x86_64-darwin.hart
`
* Look up origin users
`
http GET http://localhost:9636/v1/depot/origins/core/users Authorization:Bearer:${HAB_AUTH_TOKEN}
`

## Troubleshooting
1. If you get the following error when building, check to make sure you have imported the origin keys, and that the `HAB_DEPOT_URL` export was done correctly in the terminal session that ran `make bldr-run`:
`ERROR:habitat_builder_worker::runner: Unable to retrieve secret key, err=[404 Not Found]`

2. If you get a build failing with a `401 Unauthorized`, make sure the builder worker is pointed to a valid Github token (via a config.toml in the Procfile)

3. If you get a `NOSPC` error when starting depot UI, see the following:
http://stackoverflow.com/questions/22475849/node-js-error-enospc

### Postgres troubleshooting:
1. If you are not able to connect at all, check the `pg_hba.conf` file in `/hab/svc/postgresql` or `/hab/svc/postgres/config`.
Add the following line if not present to see if it resolves your issue:
```
local all all trust
```

2. If you are not seeing any persistent data in the DB, check the `user.toml` file in `/hab/svc/postgresql`. Make sure the search_path is not set to `pg_temp` (the DB tests create this setting explicitly in order to set the DB to be in ephemeral mode for each connection, so that it resets after every test).

3. If Postgres is not starting with `make bldr-run`, see if you can start it in a separate terminal windows with `hab start core/postgresql` or `sudo hab start core/postgresql`

4. In order to connect to Postgres (to examine the DB, etc), you can do the following from a separate terminal window (the specific path to Postgres may be different):
```
su - hab
/hab/pkgs/core/postgresql/9.6.1/20170215221136/bin/psql -h 127.0.0.1 <db_name>
```
