+++
title = "Delivery CLI"
draft = false
robots = "noindex"


aliases = ["/delivery_cli.html", "/ctl_delivery.html"]

[menu]
  [menu.legacy]
    title = "Delivery CLI"
    identifier = "legacy/workflow/reference/delivery_cli.md Delivery CLI"
    parent = "legacy/workflow/reference"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_cli.md)

The Delivery CLI is the command-line interface for the workflow
capabilities in Chef Automate. It sits in-between a local git repository
and the Chef Automate server.

## Install Delivery CLI

{{% delivery_cli_install %}}

## Configure Delivery CLI

{{% delivery_cli_configure %}}

## Run Delivery CLI in FIPS Mode

{{% fips_intro %}}

See the main [FIPS documentation](/fips/) for more information on
what FIPS is and how to enable it.

{{% delivery_cli_fips %}}

## delivery api

Use the `api` subcommand to make an HTTP request to the Chef Automate
API using the `--data` option to specify the JSON that contains the data
in the request. This request must be authorized using a token generated
by the `token` subcommand.

### Syntax

This subcommand has the following syntax:

``` bash
delivery api METHOD PATH (options)
```

where:

-   `METHOD` is an HTTP method (`GET`, `POST`, `PUT`, `DELETE`) that is
    supported by the Chef Automate API
-   `PATH` is an endpoint in the Chef Automate API scoped to the
    specified Chef Automate enterprise

### Options

This subcommand has the following options:

`--api-port=PORT`

:   The HTTP port on which the Chef Automate API is listening.

`--config-path=PATH`

:   The path for the directory in which configuration is written.

`-d=DATA`, `--data=DATA`

:   The JSON data to submit to the Chef Automate API.

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`-o=ORGANIZATION`, `--org=ORGANIZATION`

:   An organization inside a Chef Automate enterprise.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running. This must be the FQDN
    for the Chef Automate server. For example: `delivery.example.com`.

`-u=USER`, `--user=USER`

:   A Chef Automate user name. This user must exist in the specified
    enterprise (`--ent`).

### Examples

A GitHub user name must be associated with Chef Automate in order for
changes piped to Chef Automate created by GitHub pull requests to be
associated with the corresponding Chef Automate user.

{{< note >}}

Two GitHub accounts may not be linked to a single Chef Automate user.
Two Chef Automate users may not share a single GitHub user name.

{{< /note >}}

{{< note >}}

You must have previously setup GitHub integration in order for this
command to work.

{{< /note >}}

**Link a GitHub enterprise user name**

``` bash
delivery api put users/$DELIVERY_NAME/set-oauth-alias --data='{"app":"github-enterprise","alias":"$GITHUB_NAME"}'
```

**Link a GitHub.com user name**

``` bash
delivery api put users/$DELIVERY_NAME/set-oauth-alias --data='{"app":"github","alias":"$GITHUB_NAME"}'
```

**Get list of blocked projects**

``` bash
delivery api get blocked_projects --ent ENTERPRISE --server URL
```

## delivery checkout

Use the `checkout` subcommand to check out an open change on an existing
project.

### Syntax

This subcommand has the following syntax:

``` bash
delivery checkout CHANGE (options)
```

where:

-   `CHANGE` is the name of a feature branch

### Options

This subcommand has the following options:

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`-P=NUMBER`, `--patchset=NUMBER`

:   The patchset number. Default value: `latest`.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

### Examples

None.

## delivery clone

Use the `clone` subcommand to clone a Chef Automate project.

{{< note >}}

There is a **clone** command in the Chef Automate web UI on the page for
an existing project.

{{< /note >}}

### Syntax

This subcommand has the following syntax:

``` bash
delivery clone PROJECT (options)
```

where:

-   `PROJECT` is the Chef Automate project to be cloned

### Options

This subcommand has the following options:

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`-g=URL`, `--git-url=URL`

:   The raw git URL for the specified project. This URL is used as the
    remote target for the local git checkout. If this option is used,
    the `--ent`, `--org`, `--server`, and `--user` options are ignored.

`-o=ORGANIZATION`, `--org=ORGANIZATION`

:   An organization inside a Chef Automate enterprise.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

`-u=USER`, `--user=USER`

:   A Chef Automate user name.

### Examples

None.

## delivery diff

Use the `diff` subcommand to perform a `git diff` between the change and
the pipeline.

### Syntax

This subcommand has the following syntax:

``` bash
delivery diff CHANGE (options)
```

where:

-   `CHANGE` is the name of the feature branch associated with the
    change

### Options

This subcommand has the following options:

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`-l`, `--local`

:   Run a diff against the local branch `HEAD`.

`-P=NUMBER`, `--patchset=NUMBER`

:   The patchset number. Default value: `latest`.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

### Examples

None.

## delivery init

Use the `init` subcommand to initialize a Chef Automate project. This
will set up a local repository in the Chef Automate server, set up a
pipeline, and commit a build cookbook specific to the project.
Subsequent changes to this repo should be done using the `review`
subcommand.

### Syntax

This subcommand has the following syntax:

``` bash
delivery init (options)
```

### Options

This subcommand has the following options:

`--bitbucket=PROJECT_KEY`

:   The Bitbucket repository to use for code review with the associated
    project key.

`--config-path=PATH`

:   The path for the directory in which configuration is written.

`-c=PATH`, `--config-json=PATH`

:   The path to a custom `config.json` file.

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`--generator=GENERATOR`

:   The path to a local git repo or the URL to a custom `build-cookbook`
    generated by ChefDK. See <https://github.com/chef-cookbooks/pcb> for
    more information about using the `chef generate` commands in ChefDK
    to generate a `build-cookbook`.

`--github=ORGANIZATION`

:   The GitHub repository to use for code review with the associated
    organization. See `--no-verify-ssl`.

`-l`, `--local`

:   Run locally without the Chef Automate server.

`-n`, `--no-open`

:   Prevent opening a browser that shows the pipeline in Chef Automate
    web UI.

`--no-verify-ssl`

:   Specifies that SSL verification is not used with a GitHub
    repository. See `--github`.

`-o=ORGANIZATION`, `--org=ORGANIZATION`

:   An organization inside a Chef Automate enterprise.

`-p=PROJECT`, `--project=PROJECT`

:   A project inside a Chef Automate organization.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

`-r=REPO_NAME`, `--repo-name=REPO_NAME`

:   The name of the repository. This will vary, depending on whether
    it's located in git, GitHub, or Bitbucket.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

`--skip-build-cookbook`

:   Skip the creation of a `build-cookbook` when initializing a project.

`<type>`

:   The type of project. Default value: `cookbook`.

`-u=USER`, `--user=USER`

:   A Chef Automate user name.

### Examples

**Initialize project with Bitbucket repository**

{{% delivery_cli_init_bitbucket_project %}}

**Initialize project with GitHub repository**

To initialize a project using a GitHub repository, run a command similar
to:

``` bash
delivery init --github ORG_NAME -r REPO_NAME
```

where `ORG_NAME` is the name of the GitHub organization and `REPO_NAME`
is the name of the repository in GitHub. For example to initialize the
`seapower` repository in GitHub with the `chef-cookbooks` organization:

``` bash
delivery init --github chef-cookbooks -r seapower
```

and returns output similar to:

``` bash
Chef Delivery
Loading configuration from /Users/albertatom/chef/delivery/organizations/sandbox/seapower
Is /Users/albertatom/chef/delivery/organizations/sandbox/seapower a git repo?  yes
Project seapower already exists.
Creating and checking out add-delivery-config feature branch: done
Generating build cookbook skeleton
Using cached copy of build-cookbook generator "/Users/albertatom/.delivery/cache/generator-cookbooks/pcb"
Build-cookbook generated: "chef" "generate" "cookbook" ".delivery/build-cookbook" "-g" "/Users/albertatom/.delivery/cache/generator-cookbooks/pcb"
Adding and committing build-cookbook: done
Writing configuration to /Users/albertatom/chef/delivery/organizations/sandbox/seapower/.delivery/config.json
New delivery configuration
--------------------------
{
  "version": "2",
  "build_cookbook": {
    "path": ".delivery/build-cookbook",
    "name": "build-cookbook"
  },
  "skip_phases": [],
  "build_nodes": {},
  "dependencies": []
}
Git add and commit delivery config: done
Push add-delivery-config branch and create Pull Request
```

**Add build-cookbook from private Supermarket**

The following example shows how to add a build cookbook after the
initialization process

``` bash
delivery init -skip-build-cookbook
```

and then update the `config.json` file for the `delivery-truck` cookbook
and the path to the cookbook in a private Chef Supermarket:

``` javascript
{
  "version": "2",
  "build_cookbook": {
    "name": "delivery-truck",
    "supermarket": "true",
    "site": "https://private-supermarket.example.com"
  },
  ...
}
```

**Initialize project with custom pipeline**

To initialize a project using a GitHub repository, run a command similar
to:

``` bash
delivery init --generator PATH_TO_COOKBOOK -c PATH_TO_CONFIG -f PIPELINE
```

where `PATH_TO_COOKBOOK` is path to the cookbook generator,
`PATH_TO_CONFIG` is the path to a `config.json` file, and `PIPELINE` is
the name of a pipeline in Chef Automate. For example to initialize a
pipeline using the `bc-generator` cookbook generator and the `trunk`
pipeline:

``` bash
delivery init --generator https://github.com/albertatom/bc-generator.git -c /Users/albertatom/chef/delivery/.delivery/config.json -f trunk
```

returns output similar to:

``` bash
Chef Delivery
Loading configuration from /Users/albertatom/chef/delivery/organizations/sandbox/seapower
Is /Users/albertatom/chef/delivery/organizations/sandbox/seapower a git repo?  yes
Creating delivery project: seapower  created
adding remote delivery: ssh://albertatom@Chef@delivery.chef.co:8989/Chef/sandbox/seapower
Remote 'delivery' added to git config!
Checking for content on the git remote delivery: No upstream content
Pushing local content to server:
To ssh://albertatom@Chef@delivery.chef.co:8989/Chef/sandbox/seapower
*   refs/heads/master:refs/heads/master [new branch]
Branch master set up to track remote branch master from delivery.
Done

Creating trunk  pipeline for project: seapower:  created
Creating and checking out add-delivery-config feature branch: done
Generating build cookbook skeleton
Downloading build-cookbook generator from "https://github.com/albertatom/bc-generator.git"
Build-cookbook generated: "chef" "generate" "cookbook" ".delivery/build-cookbook" "-g" "/Users/albertatom/.delivery/cache/generator-cookbooks/bc-generator"
Adding and committing build-cookbook: done
Copying configuration to /Users/albertatom/chef/delivery/organizations/sandbox/seapower/.delivery/config.json
New delivery configuration
--------------------------
{
  "version": "2",
  "build_cookbook": {
    "path": ".delivery/build-cookbook",
    "name": "build-cookbook"
  },
  "skip_phases": [
    "smoke",
    "security",
    "syntax",
    "lint",
    "quality"
  ],
  "build_nodes": {},
  "delivery-truck": {
    "publish": {
      "chef_server": true
    }
  },
  "dependencies": []
}

Git add and commit delivery config: done
Chef Delivery
Loading configuration from /Users/albertatom/chef/delivery/organizations/sandbox/seapower
Review for change add-delivery-config targeted for pipeline trunk
Created new patchset
https://delivery.chef.co/e/Chef/#/organizations/sandbox/projects/seapower/changes/9e5b6c36-8deb-4c5c-822c-52e2863b8bb6
  seapower git:(add-delivery-config)
```

## delivery job

Use the `job` subcommand to execute a Chef Automate phase. This command
starts two Chef Infra Client runs: the first is based on the default
recipe in a build cookbook and the second is based on the specified Chef
Automate phase.

### Syntax

This subcommand has the following syntax:

``` bash
delivery job STAGE PHASE (options)
```

where:

-   `STAGE` is a stage in the Chef Automate pipeline: Verify, Build,
    Acceptance, Union, Rehearsal, Delivered
-   `PHASE` is a phase, which runs recipes, in a Chef Automate stage

### Options

This subcommand has the following options:

`-b=BRANCH`, `--branch=BRANCH`

:   A branch name for a Chef Automate change.

`-C=CHANGE`, `--change=CHANGE`

:   A branch name for a Chef Automate change.

`--change-id=ID`

:   The unique identifier for the specified Chef Automate change.

`--docker=IMAGE`

:   The Docker image in which the job is run.

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`-g=URL`, `--git-url=URL`

:   The raw git URL for the specified project. This URL is used as the
    remote target for the local git checkout when the job is run. If
    this option is used, the `--ent`, `--org`, `--server`, and `--user`
    options are ignored.

`-j=PATH`, `--job-root=PATH`

:   The path to the job root.

`-l`, `--local`

:   Run locally without the Chef Automate server.

`-n`, `--no-spinner`

:   Disable the spinner.

`-o=ORGANIZATION`, `--org=ORGANIZATION`

:   An organization inside a Chef Automate enterprise.

`-p=PROJECT`, `--project=PROJECT`

:   A project inside a Chef Automate organization.

`-P=NUMBER`, `--patchset=NUMBER`

:   The patchset number. Default value: `latest`.

`<phase>`

:   The name of a Chef Automate phase.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

`-S=GIT_SHA`, `--shasum=GIT_SHA`

:   The git SHA associated with a patchset.

`--skip-default`

:   Skip the `default.rb` recipe in the `build-cookbook`.

`-u=USER`, `--user=USER`

:   A Chef Automate user name.

### Examples

**Verify a job**

To run your unit tests on your local machine the same way they'd be run
on Chef Automate, run the following command:

``` bash
delivery job verify unit --local
```

which will return output similar to:

``` bash
Chef Delivery
Loading configuration from /Users/adam/src/opscode/delivery/opscode/delivery-cli
Starting job for verify unit
Creating workspace
Cloning repository, and merging adam/job to master
Configuring the job
Running the job
Starting Chef Client, version 11.18.0.rc.1
resolving cookbooks for run list: ["delivery_rust::unit"]
Synchronizing Cookbooks:
  - delivery_rust
  - build-essential
Compiling Cookbooks...
Converging 2 resources
Recipe: delivery_rust::unit
  * execute[cargo clean] action run
    - execute cargo clean
  * execute[cargo test] action run
    - execute cargo test

Running handlers:
Running handlers complete
Chef Client finished, 2/2 resources updated in 32.770955 seconds
```

## delivery local

Use the `local` subcommand to run Delivery phase or stage on your local
Chef Workstation installation, based on settings in the `project.toml`
file located in the project's `.delivery` directory.

### Syntax

This subcommand has the following syntax:

``` bash
delivery local PHASE|STAGE
```

where `PHASE` is one of the following:

-   lint
-   syntax
-   unit
-   provision
-   deploy
-   smoke
-   functional
-   cleanup

and `STAGE` will execute a series of phases in the following order: \*
verify: \[lint, syntax, unit\] \* acceptance: \[provision, deploy,
smoke, functional, cleanup\] \* all: \[lint, syntax, unit, provision,
deploy, smoke, functional, cleanup\]

### Configuration

**Phases**

Phases are defined in the `project.toml` file in the following format:

``` ruby
[local_phases]
name_of_phase = "command to execute locally"
```

Example configuration for commands to run locally:

``` ruby
[local_phases]
unit = "rspec spec/"
lint = "cookstyle"
syntax = "echo skipping syntax phase"
```

**Remote project.toml**

You can use a `project.toml` file located in a remote location by
specifying a URI in the following format:

``` ruby
remote_file = "https://url-for-my-project.toml"
```

This is useful for teams that wish to centrally manage the behavior of
the `delivery local` command across many different projects.
Alternatively, you can provide the URI via the `-r` flag:

``` bash
delivery local syntax -r https://url-for-my-project.toml
```

Providing the URI through this manner will take precedence over anything
configured in the local `project.toml`.

### Examples

**Run Cookstyle**

If the `project.toml` file contains:

``` ruby
unit = "rspec spec/"
lint = "cookstyle --only ChefCorrectness"
syntax = "echo skipping syntax phase"
provision = "kitchen create"
deploy = "kitchen converge"
smoke = "kitchen verify"
cleanup = "kitchen destroy"
```

the command

``` bash
delivery local lint
```

will run Cookstyle and execute the following command locally:

``` bash
cookstyle --only ChefCorrectness
```

**Run Verify Stage**

If the `project.toml` file contains:

``` ruby
unit = "rspec spec/"
lint = "cookstyle --only ChefCorrectness"
syntax = "echo skipping syntax phase"
provision = "kitchen create"
deploy = "kitchen converge"
smoke = "kitchen verify"
cleanup = "kitchen destroy"
```

the command

``` bash
delivery local lint
```

will run lint, syntax and unit phases in that order:

``` bash
Chef Delivery
Running Lint Phase
Inspecting 45 files
.............................................

45 files inspected, no offenses detected
```

## delivery review

Use the `review` subcommand to submit a feature branch for review as a
new patchset. This either creates a new change associated with the
feature branch, or adds a new patchset on an existing change in the
pipeline. When the new patchset has been created, the Verify stage for
the associated change is automatically triggered and runs the unit, lint
and syntax phases. By default, this action opens a browser window to
show the pipeline in Chef Automate.

### Syntax

This subcommand has the following syntax:

``` bash
delivery review (options)
```

### Options

This subcommand has the following options:

`-a`, `--auto-bump`

:   Bumps the cookbook metadata version number automatically when
    `delivery review` is run.

`--edit`

:   Edit the title and description for the change.

`--fips`

:   Runs command in FIPS mode. This proxies all git traffic through
    Stunnel FIPS encryption.

`--fips-git-port=PORT`

:   The port Stunnel listens locally on when proxying git traffic.

`--fips-custom-cert-filename=PATH_TO_PEM`

:   The path to a pem file that contains a self-signed certificate or
    certificate chain. Use this setting only when you have a custom
    certificate authority or a self-signed certificate.

`--no-open`

:   Prevent opening a browser that shows the pipeline in Chef Automate
    web UI.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

### Examples

**Bump version metadata automatically**

``` bash
delivery review --auto-bump
```

will return something similar to:

``` none
Chef Delivery
Loading configuration from /Users/albertatom/delivery/organizations/sandbox/coffee
Project coffee is a cookbook
Validating version in metadata
The version hasn't been updated (0.1.0)
Bumping version to: 0.1.1
Review for change black targeted for pipeline master
Created new patchset
https://delivery.chef.co/e/URL_FOR_CHANGE
```

## delivery setup

Use the `setup` subcommand to set up the Chef Automate project. This
will set up the configuration needed for a project to communicate with
the Chef Automate server. Use the `token` subcommand to get an API token
that allows authorized requests to be made to the server.

### Syntax

This subcommand has the following syntax:

``` bash
delivery setup (options)
```

### Options

This subcommand has the following options:

`--config-path=DIRECTORY`

:   The path for the directory in which configuration is written.

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`-o=ORGANIZATION`, `--org=ORGANIZATION`

:   An organization inside a Chef Automate enterprise.

`--pipeline=PIPELINE`

:   The name of a Chef Automate pipeline.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

`-u=USER`, `--user=USER`

:   A Chef Automate user name.

### Examples

None.

## delivery status

Get status information about the Chef Automate server's `_status`
endpoint, API response time, and additional information depending on the
server's configuration.

### Syntax

This subcommand has the following syntax:

``` bash
delivery status (options)
```

### Options

This subcommand has the following options:

`--api-port=PORT`

:   The HTTP port on which the Chef Automate API is listening.

`--json`

:   Output in JSON format instead of human readable.

`--no-color`

:   Prevent color output.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

### Examples

``` shell
delivery status

Status information for Automate server automate.example.com...

Status: up (request took 75 ms)
Configuration Mode: standalone
FIPS Mode: enabled
Upstreams:
Lsyncd:
  status: not_running
PostgreSQL:
  status: up
RabbitMQ:
  status: up
  node_health:
    status: up
  vhost_aliveness:
    status: up

Your Automate Server is configured in FIPS mode.
Please add the following to your cli.toml to enable Automate FIPS mode on your machine:

fips = true
fips_git_port = "OPEN_PORT"

Replace OPEN_PORT with any port that is free on your machine.
```

## delivery token

Use the `token` subcommand to manage a Chef Automate API token.

{{< note >}}

If you're running this command on Windows in Git Bash with MinTTY you
must include `winpty` before `delivery token` to avoid errors.

{{< /note >}}

### Syntax

This subcommand has the following syntax:

``` bash
delivery token (options)
```

{{< note >}}

You can also pass in your Chef Automate password through an environment
variable to the <span class="title-ref">delivery token</span>
subcommand. If this variable is set, you will not be asked to input your
password.

``` bash
AUTOMATE_PASSWORD=secret delivery token -s automate.example.com -e myent -u token
```

{{< /note >}}

### Options

This subcommand has the following options:

`--api-port=PORT`

:   The HTTP port on which the Chef Automate API is listening.

`-e=ENTERPRISE`, `--ent=ENTERPRISE`

:   A configured Chef Automate enterprise.

`--raw`

:   Print the raw token.

`-s=SERVER`, `--server=SERVER`

:   The server on which Chef Automate is running.

`-u=USER`, `--user=USER`

:   A Chef Automate user name.

`--verify`

:   Verify if a token is a valid token.

### Examples

**Verify a token**

``` bash
delivery token --verify
```

returns something similar to:

``` none
Chef Delivery
Loading configuration from /Users/dennisteck/chef/delivery
token: GmTtD0t7asgy5KZyw//r/6etpXYfw8dfgQccjdeU=
Verifying Token: valid
```
