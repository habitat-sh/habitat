## Bulk Project Creation Tool

This is a development tool that can be used to bulk create projects for builder
services. Project creation is a key pre-requisite that needs to be done for the
builder to recognize and be able to kick off automated builds.

This tool is targeted to creating ```core``` origin projects. It takes as
input parameters a local path to the core-plans repo, an API endpoint URL,
an installation id (for an installed Habitat Builder app), and a Github auth
token.

### Usage

You need to have a recent Ruby installed.

To run:
```
ruby project_create.rb <core-plans-dir> <projects-url> <installation-id> [<auth-token>]
```

The projects-url should be in this form (replace the URL appropriately):
https://bldr.acceptance.habitat.sh

For a development environment, the URL will be:
http://localhost:9636

If `<auth-token>` is not specified, the script will default to looking for
the `HAB_AUTH_TOKEN` environment variable.

### Extras

This directory also contains some helper tools for querying installations
and repositories from Github using Github App authentication. These
require nodejs to be installed (any recent version should be fine).

The tools are:
* app.js - gets basic app info (for sanity validation)
* installations.js - get a list of installation ids and names
* repos.js - gets a list of repo ids and repo names

In order to use these tools, you need to know the Github app id, and have
the PEM file available on the machine. For reference, the Habitat Builder
Dev app id is 5629.

Example usage (assumes the pem file is in the current folder):
```
node app.js 5629 builder-github-app.pem
node installations.js 5629 builder-github-app.pem
node repos.js 5629 56940 builder-github-app.pem
```
