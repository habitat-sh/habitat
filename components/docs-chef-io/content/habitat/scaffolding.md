+++
title = "Scaffolding"
description = "Scaffolding"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Scaffolding"
    identifier = "habitat/plans/scaffolding"
    parent = "habitat/plans"
    weight = 95
+++

Chef Habitat scaffoldings are standardized plans for automated building and running your application. Each scaffolding is tuned to the way your application was built, which allows it to create the appropriate [application lifecycle hooks]({{< relref "application_lifecycle_hooks" >}}) and add in the correct runtime dependencies when building the package for your application. Scaffoldings also provide some default health check hooks where appropriate to ensure your application is functioning reliably. Customized Scaffolding can be created to facilitate re-usability of common patterns in your organization for developing, building, and running your applications.

## Automated Scaffolding

While we are targeting many platforms for automated scaffolding we currently support Ruby, Node.js and Gradle.

* [core/scaffolding-ruby](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-ruby/doc/reference.md)
* [core/scaffolding-node](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-node)
* [core/scaffolding-gradle](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-gradle)

## Variables

Scaffolding provides certain customizable variables for language-specific behavior. Please see the appropriate scaffolding documentation for details.

### Overriding Scaffolding Callbacks

If you want to override phases of a scaffold's build in your plans, make sure to override the main `do_xxx` phase, not the callback directly. ex override `do_install()` instead of `do_default_install` or `do_node_install`.

### Scaffolding Internals

A language or framework scaffolding is shipped as a Chef Habitat package, which means that each scaffolding runtime dependency becomes a build dependency for the application being built.

## lib/scaffolding.sh File

To create scaffolding, a package must contain a `lib/scaffolding.sh` file which gets sourced by the build program running Bash.

## scaffolding_load() Function

A optional function named `scaffolding_load()` may be created in `lib/scaffolding.sh` which will be called early in the build program which allows a Scaffolding author to control and augment the `pkg_deps` and `pkg_build_deps` arrays. At this point, no other build or run dependencies have been resolved so the code in this function can only rely on what the build program provides or software pulled in via the Scaffolding's Plan.

## Default Build Phases Implementations

The remainder of the `lib/scaffolding.sh` contains one or more default implementations for the build phases. These include, but are not limited to:

* `do_default_prepare()`
* `do_default_build()`
* `do_default_install()`
