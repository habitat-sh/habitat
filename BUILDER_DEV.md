# Builder Services Development Environment

## Overview

This document captures the steps to start and run a Builder environment for development. The builder environment includes the builder services, as well as the depot web site.

There are several ways of creating a Builder dev environment - but supporting all operating systems and environments has proven to be untenable. This document includes one officially supported way of creating a Builder dev environment, and links to unsupported ways of creating the dev environment that you may use at your own risk.

Eventually, we will want to harness the power of the Habitat studio to automate a builder dev environment, but this serves as a stop gap for now.

## Officially Supported Dev Environment

This Dev Environment involves spinning up a virtual machine (most of the core contributors use VMWare, but Virtual Box should work as well) running Ubuntu 17.10 desktop.

### Pre-Reqs

* Ubuntu 17.10 Desktop Virtual Machine
* Access to the "core" origin in [Production Builder](https://bldr.habitat.sh). Ask a core maintainer (or in the [Habitat Slack](http://slack.habitat.sh/)) if you do not have access.
* Access to the Habitat 1Password account (NOTE - we are working on a solution to make this not required, but it is necessary for the time being)
* All of the following steps should be run within your Virtual Machine
* A [Github Personal Access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) with all repo and all user permissions.
* The [Habitat Builder App](https://github.com/apps/habitat-builder) installed on your Github account.
* Open your Sudoers file at /etc/sudoers and remove the "secure_path" section (this is required for the make scripts to run correctly)

```
$ sudo visudo -f /etc/sudoers
```

### Builder Setup
* Update your system
```
$ sudo apt-get update
```

* Install git
```
$ sudo apt-get install git
```

* Install curl
```
$ sudo apt-get install curl
```

* Clone the [Habitat repo](https://github.com/habitat-sh/habitat)
```
$ git clone https://github.com/habitat-sh/habitat.git
```

* Install the [Habitat CLI](https://www.habitat.sh/docs/get-habitat/)
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
$ cd path/to/your/habitat/repo
```

* Run the provision script

```
$ sudo ./support/linux/provision.sh
```

* After the script completes, source cargo/rust
```
$ source $HOME/.cargo/env
```

* Run the build-srv makefile (NOTE - if you receive an error about cargo not being found, make sure you have removed the "secure_path" section of /etc/sudoers)

```
$ sudo make build-srv
```

* Open up the builder-worker file

```
$ sudo vim /hab/svc/builder-worker
```

* Add your Github Auth Key:

**/hab/svc/builder-worker/config.toml**
```
auth_token = ""
```

Should become:

**/hab/svc/builder-worker/config.toml**
```
auth_token = "<your github token>"
```

* Run this command (it may take awhile) to start all the Builder services

```
$ sudo -E make bldr-run
```

### UI Setup

* Open a separate shell on your VM
* Install npm

```
$ sudo apt-get install npm
```

* Cd into the builder-web component

```
$ cd path/to/your/habitat/repo/components/builder-web
```

* Install all dependencies

```
$ npm install
```

* Create the habitat.conf.js file

```
$ cp habitat.conf.sample.js habitat.conf.js
```

* Start the UI

```
$ sudo npm start
```

* Open up a browser and navigate to http://localhost:3000/#/pkgs - you should see the Habitat UI running locally.

### Creating an Origin

* Navigate to http://localhost:3000/#/pkgs
* Log in
* Click on "Create New Origin"
* Fill out form (call the origin "core"), click "Save & Continue"
* Click on "Keys"
* Download the public and private key
* Copy the public key contents
* From a terminal, run

```
$ hab origin key import
```

* Paste the public key contents
* Hit Ctrl + D twice to close the stream
* Run this again:

```
$ hab origin key import
```

* Paste the private key contents
* Hit Ctrl + D twice to close the stream

### Install Dependencies in your local Builder Env

* on your VM, go to the production instance of Builder (the one at habitat.sh)
* Click on "My origins"
* Click on "core"
* Download public and private keys for the "core" origin (make sure the timestamp on the public key matches the one on the private key!) - we currently need the keys for the core production origin in order to upload some dependencies to our local Builder env)
* From a terminal, run

```
$ hab origin key import
```

* Paste the production core origin public key contents
* Hit Ctrl + D twice
* Run this again:

```
$ hab origin key import
```

* Paste the production core origin private key contents
* Hit Ctrl + D twice
* Run
```
$ hab install core/hab-backline
$ cd /hab/cache/artifacts
$ hab pkg upload -c stable core-hab-backline...hart
```
* You will need to follow this same process for any dependencies of anything you want to build locally - the sample app I will use below depends on [core/scaffolding-node](https://bldr.habitat.sh/#/pkgs/core/scaffolding-node/latest), [core/git](https://bldr.habitat.sh/#/pkgs/core/git/latest), [core/node](https://bldr.habitat.sh/#/pkgs/core/node/latest), [core/busybox-static](https://bldr.habitat.sh/#/pkgs/core/busybox-static/latest) - so I will follow the same procedure to download them from production builder, then upload them to my local builder.

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
    ````

## Windows build notes

The `-configure` switch will make sure you have all the necessary dependencies to build the `hab` CLI tool, including Rust, the Visual Studio Build Tools, and all the native dependencies.

Not all crates have been fully ported to Windows.

Currently the `hab` command will build (as well as the dependent crates).

Work is in progress on the Supervisor and other parts of the toolchain.

# Running all builder components

Run this command:
```
make bldr-run
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
