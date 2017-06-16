# Builder Services Development Environment

## Overview
This document outlines the steps to start and run a Builder environment for development. The builder environment includes the builder services, as well as the depot web site.

## Pre-Reqs
1. Use a Linux OS - either native or VM.
1. Clone the habitat repo to your local filesystem. (If using a VM, you should use the local filesystem for the hab repo instead of using a mounted filesystem, otherwise things may fail later when running services).
1. Ensure you have a Github auth token. Create from the Github site if you don't have one already. Token Capabilities needed: read:org, user:email
1. The sample commands below use the 'httpie' tool. Install it if not present on your system (https://github.com/jkbrzt/httpie).
1. A recent version of the habitat cli should also be installed in your dev environment (https://www.habitat.sh/docs/get-habitat/)

## Bootstrap the OS with required packages
You need to make sure you have the required packages installed.
Run the appropriate shell script under the `support/linux` folder to install the packages.
Refer to [BUILDING.md](./BUILDING.md) doc for the detailed steps.

## Create configuration files
Some capabilities (such as allowing builder permissions, and turning on auto-building when new packages are uploaded to the depot) require passing in a custom config to the Builder services.

Create the following files somewhere on your local filesystem (Note: the client_id and client_secret below are for development purposes only):

`config_api.toml`
```toml
[depot]
builds_enabled = true

[github]
url = "https://api.github.com"
client_id = "0c2f738a7d0bd300de10"
client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
```

`config_sessionsrv.toml`
```toml
[permissions]
admin_team = 1995301
build_worker_teams = [1995301]
early_access_teams = [1995301]

[github]
url = "https://api.github.com"
client_id = "0c2f738a7d0bd300de10"
client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
```

`config_worker.toml`
```toml
auth_token = "<your github token>"
depot_url = "http://localhost:9636/v1/depot"
auto_publish = true
```

`config_jobsrv.toml`
```toml
[archive]
backend = "local"
local_dir = "/tmp"
```

(Note: If you want your log files to persist across restarts of your development machine, replace `/tmp` with some other directory. It *must* exist and be writable before you start the job server).

Now, modify the `Procfile` (located in your hab repo in the `support` folder) to point the api, sessionsrv, jobsrv, and worker services to the previously created config files, e.g.

```
api: target/debug/bldr-api start --path /tmp/depot --config /home/your_alias/habitat/config_api.toml
sessionsrv: target/debug/bldr-session-srv start --config /home/your_alias/habitat/config_sessionsrv.toml
worker: target/debug/bldr-worker start --config /home/your_alias/habitat/config_worker.toml
jobsrv: target/debug/bldr-job-srv start --config /home/your_alias/habitat/config_jobsrv.toml
```

## Run the Builder services
1. Open a new terminal window.
1. Export the following environment variables:

```
export HAB_AUTH_TOKEN=<your github token>
export HAB_DEPOT_URL=http://localhost:9636/v1/depot
export HAB_ORIGIN=<your origin>
```

1. Now, switch to the root user and do a `make bldr-run` from the root of your hab repo.

The first time this command runs, it will create the required databases. Let it run for a while, and then re-start it if there are errors (this is normal for the first time setup).

When `make bldr-run` can proceed with all the services up and running, you are ready to proceed to the next step.

## Create your origin
There are a couple of ways to do this.

### From the command line
1. Open a new terminal window.
1. Export the `HAB_ORIGIN` and `HAB_AUTH_TOKEN` as above
1. Create an origin in the DB by issuing the following command:

    ```
    http POST http://localhost:9636/v1/depot/origins Content-Type:application/json Authorization:Bearer:${HAB_AUTH_TOKEN} name=${HAB_ORIGIN}
    ```

    The response should be something like:

    ```
    HTTP/1.1 201 Created

    {
        "id": 151409091187589122,
        "name": "core",
        "owner_id": "133508078967455744",
        "private_key_name": ""
    }
    ```

### In the browser
The `make bldr-run` command you ran above also starts the Builder Web UI, which should be listening on port 3000. If that process is running in a container or VM, and you've forwarded port 3000 to your host, you should be able to browse to http://localhost:3000/#/pkgs, sign in with GitHub, and click My Origins in the sidebar to create an origin.

## Import origin keys to the Depot
This can be done via the Builder Web UI. Make sure you have both the public and private keys available for your origin.

The web UI should be running at http://localhost:3000/#/pkgs. (If you need to bring up the UI at a different address or port, please refer to the builder-web [README](./components/builder-web/README.md) for setting up a custom OAuth app).

1. Log into the web UI via your GitHub account
1. Go to the origins page, and select your origin
1. Go to the keys tab, and follow the instructions for uploading the public and private keys.

## Create the project(s) you want to build
In order to build a package, there needs to be a project created in the database.
If the DB has been newly created, there will initially not be any projects available.

Create a project for the package you want to build:

1. Create a project file (eg, `project.json`) on your local filesystem (see example below for core/nginx):

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

1. Issue the following command:

    ```
    http POST http://localhost:9636/v1/projects Authorization:Bearer:${HAB_AUTH_TOKEN} < project.json
    ```

The response should be something like this:

    ```
    HTTP/1.1 201 Created

    {
        "id": "730634033288978462",
        "name": "core/nginx",
        "origin_id": "730632763933204510",
        "origin_name": "core",
        "owner_id": "730632479777497215",
        "package_name": "nginx",
        "plan_path": "nginx/plan.sh",
        "vcs_data": "https://github.com/habitat-sh/core-plans.git",
        "vcs_type": "git"
    }
    ```

Repeat the above steps for any other projects that you want to build.

## Upload any dependent packages to disk

During a build, the hab studio will look to download dependent packages from
your local machine. If the package metadata is present, but the on-disk
archive is not accessible, the builds will fail.

Make sure you do a `hab pkg upload` to your environment
```
export HAB_DEPOT_URL=http://localhost:9636/v1/depot
hab pkg upload <package>
```

You can get the packages from the production or acceptance environments
by pointing `HAB_DEPOT_URL` to https://app.habitat.sh or
http://app.acceptance.habitat.sh, and then doing a ```hab pkg install <package>```.

## Run a build

Issue the following command (replace `core/nginx` with your origin and package names):

```
http POST http://localhost:9636/v1/jobs Authorization:Bearer:${HAB_AUTH_TOKEN} project_id="core/nginx"
```

This should create a build job, and then dispatch it to the build worker.

You should see a response similar to the following:

```
HTTP/1.1 201 Created

{
    "created_at": "",
    "id": "0",
    "name": "nginx",
    "origin": "core",
    "state": "Pending"
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

1. If you get a build failing with a `401 Unauthorized`, make sure the builder worker is pointed to a valid Github token (via a config.toml in the Procfile)

1. If you get a `NOSPC` error when starting depot UI, see the following:
http://stackoverflow.com/questions/22475849/node-js-error-enospc

1. If you get a `404 Not Found` error during a build, make sure that you
have uploaded any dependent packages locally.

```
hab-studio: Destroying Studio at /tmp/739078532143013888/studio (unknown)
hab-studio: Creating Studio at /tmp/739078532143013888/studio (default)
hab-studio: Importing core secret origin key
» Importing origin key from standard input
★ Imported secret origin key core-20160810182414.
» Installing core/hab-backline
✗✗✗
✗✗✗ [404 Not Found]
✗✗✗
```

### Postgres troubleshooting:
1. If you are not able to connect at all, check the `pg_hba.conf` file in `/hab/svc/postgresql` or `/hab/svc/postgres/config`.
Add the following line if not present to see if it resolves your issue:
    ```
    local all all trust
    ```

1. If you are not seeing any persistent data in the DB, check the `user.toml` file in `/hab/svc/postgresql`. Make sure the search_path is not set to `pg_temp` (the DB tests create this setting explicitly in order to set the DB to be in ephemeral mode for each connection, so that it resets after every test).

1. If Postgres is not starting with `make bldr-run`, see if you can start it in a separate terminal windows with `hab start core/postgresql` or `sudo hab start core/postgresql`

1. In order to connect to Postgres (to examine the DB, etc), you can do the following from a separate terminal window (the version and release portions of the path may be different):
    ```
    su - hab
    /hab/pkgs/core/postgresql/9.6.1/20170215221136/bin/psql -h 127.0.0.1 <db_name>
    ```
1. If Postgres dies when you run `make bldr-run` with an error message that
   says `WARNING: out of shared memory`, edit the `postgresql.conf` file in
   `/hab/pkgs/core/postgresql/$VERSION/$RELEASE/config` and add
   `max_locks_per_transaction=128` to it.
