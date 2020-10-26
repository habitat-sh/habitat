+++
title = "Plan Tips"
description = "Best Practices for Plans"

[menu]
  [menu.habitat]
    title = "Plan Tips"
    identifier = "habitat/plans/plan-tips"
    parent = "habitat/plans"
    weight = 30

+++

## Package Name Conventions

Each package is identified by a unique string containing four sub-strings separated
by a forward slash (`/`) called a [PackageIdent](/glossary/#glossary-artifacts).

    `origin`/`name`/`version`/`release`

The `origin`, `name`, and `version` values of this identifier are user defined by
setting their corresponding variable in your `plan.sh` or `plan.ps1` file while the value of
`release` is generated at build-time.

The value of `name` should exactly match the name of the project it represents and the plan file should be located within a directory of the same name in this repository.

> Example: The plan for the [bison project](https://www.gnu.org/software/bison/) project contains setting `pkg_name=bison` and resides in `$root/bison/plan.sh`.

## Managing Major Versions

There is one exception to this rule: Additional plans may be defined for projects for their past major versions by appending the major version number to its name. The plan file for this new package should be located within a directory of the same name.

> Example: the [bison project](https://www.gnu.org/software/bison/) maintains the 2.x line along with their current major version (at time of writing: 3.x). A second plan is created as `bison2` and placed within a directory of the same name in this repository.

Packages meeting this exception will always have their latest major version found in the package sharing the exact name of their project. A new package will be created for the previous major version following these conventions.

> Example: the [bison project](https://www.gnu.org/software/bison/) releases the 4.x line and is continuing to support Bison 3.x. The `bison` package is copied to `bison3` and the `bison` package is updated to build Bison 4.x.

## Plan Basic Settings

You can read more about [basic plan settings](/plan-overview/#write-plans) here. The minimum requirements for a core plan are:

- pkg_name is set
- pkg_origin is set
- pkg_shasum is set
- pkg_description is set

## Callbacks

You can read more about [callbacks](/reference/#reference-callbacks) here. The minimum requirement for a core plan are:

### Callback Do's

- `do_prepare()` (`Invoke-Prepare` in a `plan.ps1`) is a good place to set environment variables and set the table to build the software. Think of it as a good place to do patches.
- If you clone a repo from git, you must override `do_verify()` to `return 0` in a `plan.sh` or if you are authoring a `plan.ps1` then override `Invoke-Verify` with an empty implementation.

### Callback Don't's

- Don't call `exit` within a build phase. In a `plan.sh`, you should instead return an exit code such as `return 1` for failure, and `return 0` for success. In a `plan.ps1` you should call `Write-Exception` or `throw` an exception upon failure.
- Don't use `pkg_source` unless you are downloading something as a third party.
- Don't shell out to `hab` from inside of a callback. If you think you want to, you should use a [utility function](/reference/#utility-functions) instead.
- Don't call any functions or helper sthat begin with an underscore, for example `_dont_call_this_function()`. Those are internal for internal builder functions and are not supported for external use. They will break your plan if you call them.
- Don't run any code or run anything outside of a build phase or a function.

## Application Lifecycle Hooks

The Supervisor dynamically invokes hooks at run-time, triggered by an application lifecycle event. You can read more about [hooks](/plans/application-lifestyle-hooks) here.

### Lifecycle Hook Do's

- Do redirect `stderr` to `stdout` (e.g. with `exec 2>&1` at the start of the hook)
- Do call the command to execute with `exec <command> <options>` rather than running the command directly in a Linux targeted hook. This ensures the command is executed in the same process and that the service will restart correctly on configuration changes.
- You can write to the `/var/`, `/static/`, and `/data/` directories. Access these with your `runtime configuration setting` variable.

### Lifecycle Hook Don't's

- Don't call `hab` or `sleep` in a hook that is not the `run` hook. You can only block the thread in a hook if it is in the `run` hook.
- Don't shell out to `hab` from within a hook. If you think you want to, you should use a [runtime configuration setting](/reference/#template-data) instead. If none of those will solve your problem, open an issue and tell the core team why.
- Don't use `exec` if you're running something with a pipe. It won't work.
- Don't execute commands as a `root` user or try to `sudo hab pkg install`.
- Don't edit any of the Supervisor rendered templates.
- Don't edit anything in `/hab/` directly.
- Don't write to anything in `/hab/` directly.

## README.md

All plans need a `README.md`. Items to strongly consider including:

- Your name as maintainer and supporter of this plan.
- What Chef Habitat topology it uses (and the plan should have the correct topology for the technology).
- Step-by-step instructions for how to use the package.
- Describe the best update strategy for different deployments.
- Describe the configuration updates a user can make and if a full rebuild is required.
- Document how to scale the service.
- Instructions for monitoring the health of the service at the application layer.
- Describe how to call the package as an application dependency in an application.
- Describe how to integrate package into an application.

## A repo of plans

The best practice is to place all plan files within a `habitat` parent directory, which allows for a clean separation between your application source code and habitat specific files. However, if you maintain a separate repository solely for the purpose of storing habitat plans, then the use of a `habitat` folder may not be necessary.
