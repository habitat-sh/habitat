## Bulk Project Creation Tool

This is a development tool that can be used to bulk create projects for builder
services. Project creation is a key pre-requisite that needs to be done for the
builder to recognize and be able to kick off automated builds.

This tool is targeted to creating ```core``` origin projects. It takes as
input parameters a local path to the core-plans repo, an API endpoint URL,
and a Github auth token.

### Usage

You need to have a recent Ruby installed.

To run:
```
ruby project_create.rb <core-plans-dir> <projects-url> [<auth-token>]
```

The projects-url should be in this form (replace the URL appropriately):
http://app.acceptance.habitat.sh/v1/projects

If `<auth-token>` is not specified, the script will default to looking for
the `HAB_AUTH_TOKEN` environment variable.
