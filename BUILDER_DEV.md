# Builder Services Development Environment

## NOTE

This is currently only meant for the Habitat internal team. There is work in progress to support builder dev environments that do not require Habitat internal team secrets - stay tuned!

## Overview

This document captures the steps to start and run a Builder environment for development. The builder environment includes the builder services, as well as the depot web site.

There are several ways of creating a Builder dev environment - but supporting all operating systems and environments has proven to be untenable. This document includes one officially supported way of creating a Builder dev environment, and links to unsupported ways of creating the dev environment that you may use at your own risk.

Eventually, we will want to harness the power of the Habitat studio to automate a builder dev environment, but this serves as a stop gap for now.

## Officially Supported Dev Environment

This Dev Environment involves spinning up a virtual machine (most of the core contributors use VMWare, but Virtual Box should work as well) running Ubuntu 17.10 desktop OR Ubuntu 17.10 Server.

### Pre-Reqs

* Ubuntu 17.10 Desktop or Ubuntu 17.10 Server Virtual Machine
* Access to the "core" origin in [Production Builder](https://bldr.habitat.sh). Ask a core maintainer (or in the [Habitat Slack](http://slack.habitat.sh/)) if you do not have access.
* Access to the Habitat 1Password account (NOTE - we are working on a solution to make this not required, but it is necessary for the time being)
* All of the following steps should be run within your Virtual Machine
* A [Github Personal Access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) with all repo and all user permissions.
* The [Habitat Builder App](https://github.com/apps/habitat-builder) installed on your Github account.
* Remove the "secure_path" section from your sudoers file (this is required for the make scripts to run correctly). The following command will comment it out:
```
$ sudo sed -i.bak '/secure_path/s/^/#/' /etc/sudoers
```
* Some of the sample commands below use the 'httpie' tool. Install it if not present on your system (https://github.com/jkbrzt/httpie).


### Builder Setup
* Update your system
```
$ sudo apt-get update
```

* NOTE - if you are running Ubuntu 17.10 Server, the above command may fail with "E: The repository 'cdrom://Ubuntu-Server 17.10 _Artful Aardvark_ - Release amd64 (20171017.1) artful Release' does not have a Release file."  To fix this, run:

```
$ sudo sed -i.bak '/deb cdrom/s/^/#/g' /etc/apt/sources.list
```

* Install git
```
$ sudo apt-get install git
```

* Clone the [Habitat repo](https://github.com/habitat-sh/habitat)
```
$ git clone https://github.com/habitat-sh/habitat.git
```

* Copy habitat-builder-dev.2017-10-02.private-key.pem from 1Password into path/to/habitat/repo/.secrets/builder-github-app.pem (NOTE - this is required as of 12/11/17 - we are working on a solution to make access to the Habitat 1Password vault not required for dev environment setup).
* Set up some environmental variables (I put these in my .bashrc)

```
export HAB_AUTH_TOKEN="<your github token>"
export HAB_BLDR_URL="http://localhost:9636"
export HAB_ORIGIN="core"
```

* symlink /src to path/to/your/habitat/repo

```
$ sudo ln -s path/to/your/habitat/repo /src
```

* Cd into your Habitat repo

```
$ cd /src
```

* Run the provision script

```
$ ./support/linux/provision.sh
```

* Source cargo

```
$ source $HOME/.cargo/env
```

* Run the build-srv makefile (NOTE - if you receive an error about cargo not being found, make sure you have removed the "secure_path" section of /etc/sudoers)

```
$ make build-srv
```

* Run this command (it may take awhile) to start all the Builder services

```
$ sudo -E make bldr-run-no-build
```

* Wait for all services to start - they are started when you see worker heartbeat debug entries in the log. If services do not start because some were already running from a previous attempt, run `$ make bldr-kill`

### UI Setup

* Follow the instructions in the [Web UI README](https://github.com/habitat-sh/habitat/blob/master/components/builder-web/README.md) to get the Web UI running locally.
* Open up a browser and navigate to http://localhost:3000/#/pkgs - you should see the Habitat UI running locally.

### Creating an Origin

### Option A: Create an origin using the web UI

* Navigate to http://localhost:3000/#/pkgs
* Log in
* You may need to accept the Habitat Builder Dev Githhub app in your Github account.
* Click on "Create New Origin"
* Fill out form (call the origin "core"), click "Save & Continue"

### Option B: Create an origin using the command line

1. Create an origin in the DB by issuing the following command:

```
$ http POST http://localhost:9636/v1/depot/origins Content-Type:application/json Authorization:Bearer:${HAB_AUTH_TOKEN} name=${HAB_ORIGIN}
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
### Install Dependencies in your local Builder Env

* On your VM, go to the production instance of Builder (the one at habitat.sh)
* Click on "My origins"
* Click on "core"
* Download the public key for the "core" origin
* Next, install the public key for the core origin on the Production builder (the one at habitat.sh). You need this in order to upload some packages to your local builder.

```
$ hab origin key import <<EOT
SIG-PUB-1
core-20160810182414

vQqVVhUTW9ABKzoi9W+LP14GL2MrYRmL8FGETjwNANQ=
EOT
```
(This core public key is current as of 12/18/2017 and will continue to work unless the private key is changed.)

* Run
```
$ sudo hab install core/hab-backline --url https://bldr.habitat.sh
$ cd /hab/cache/artifacts
$ hab pkg upload -c stable core-hab-backline...hart
```

* NOTE - if you receive this error: "No auth token specified" - make sure you have set HAB_AUTH_TOKEN="<your github token>"  in your environment
* NOTE - if you receive this error: "No such file or directory (os error 2)" It means that the public key from bldr.habitat.sh (that corresponds to the private key the hart file was signed with) wasn't installed.
* NOTE - if you receive this error: "403 Forbidden" when running `hab pkg upload`, it may mean the core origin hasn't been successfully created.
* You will need to follow this same process for any dependencies of anything you want to build locally

## Create the project(s) you want to build
In order to build a package, there needs to be a project created in the database.
If the DB has been newly created, there will initially not be any projects available.
There are two ways that a project can be created - either via the web UI, or via the command line - both options are documented below:

### Option A: Create a project using the web UI

1. Go the web UI that you used in the last step
2. Go to the origins page, and select your origin
3. Click on the 'Connect a plan file' button
4. Click on 'Install Github App' button to install the Builder Dev app on your github account
5. Go back to the Packages page (from Step 3), and follow the instructions to link the plan you want to build

### Option B: Create a project using the command line
1. Create a project file (eg, `project.json`) on your local filesystem (see example below for core/nginx):

    ```
    {
        "origin": "core",
        "plan_path": "nginx/plan.sh",
        "installation_id": 56940,
        "repo_id": 46349776
    }
    ```

Note: the `installation_id` above is for the Habitat Builder Dev app, and the
`repo_id` is for the 'core-plans' repo.

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

## Run a build

### Option A: From the Web UI
* Navigate to http://localhost:3000/#/pkgs
* If you are not already logged in, log in.
* Click on "My origins"
* Click on "core"
* Click on the package you wish to build
* Click on "Latest"
* Click on "Build latest version"
* Click on "Build Jobs" and "View the output" to see the job in progress
* The job should complete successfully! Congrats, you have a working build!



### Option B: From the Command Line

Issue the following command (replace `core/nginx` with your origin and package names):

```
http POST http://localhost:9636/v1/depot/pkgs/schedule/core/nginx Authorization:Bearer:${HAB_AUTH_TOKEN}
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

Additionally, you could also run

```
$ hab bldr job start core/nginx
```

This command will have output similar to:
```
Ω Creating build job for core/nginx
✓ Created build job. The id is 876066265100378112
```

You can view the build progress in the web UI or by viewing `/hab/svc/builder-worker/data/876066265100378112/log_pipe-876066265100378112.log`. Replace `876066265100378112` with the group ID output by the `start` command.

Note: you will need to upload additional packages to the core origin for the `core/nginx` build to succeed. Follow the same procedure as for `core/hab-backline`. Currently `core/gcc` and `core/libedit` are required.

## Unsupported Dev Environments

Maintainers have historically been able to use alternative development environment setups. If you would like to explore using other OS's or Docker or Vagrant - please check out these links. **Do remember that these are NOT officially supported by the Habitat maintainers - use at your own risk!**

* [Other Operating Systems](BUILDING.md)
* [Container Dev Environment](BUILDER_CONTAINER.md)

## General build notes

- Once make has finished, executables will exist in `/src/target/debug/foo`,
  where `foo` is the name of an executable (`hab`, `hab-sup`, `hab-depot`,
  etc).
- Executable names are specified in each components `Cargo.toml` file in a TOML
  table like this:
    ```
	  [[bin]]
	  name = "hab-depot"
    ```

## Windows build notes

The `-configure` switch will make sure you have all the necessary dependencies to build the `hab` CLI tool, including Rust, the Visual Studio Build Tools, and all the native dependencies.

Not all crates have been fully ported to Windows.

Currently the `hab` command will build (as well as the dependent crates).

Work is in progress on the Supervisor and other parts of the toolchain.

# Running all builder components

Run this command:
```
sudo -E make bldr-run
```

# Building and running individual components

When you are working on an individual component in the /components directory, you may wish to build, install, then use that individual component.

Let's say you want to do this with the Supervisor (which lives in the components/sup directory).

## Building

Change directories into the component you want to build

```
cd components/sup
```

Then run

```
cargo build
```

Once it is finished compiling, you can find the new build in root hab_repo/target/debug

Head back to the root of the Habitat repo

```
cd ../..
```

And you will find your build in target/debug

If you built the sup component, this is where you would find the new build

```
target/debug/hab-sup
```

## Running

You can now run this newly built component with

```
./target/debug/hab-sup
```
