# Scaffolding

Scaffolding is a predictive implementation of the build phases and application lifecycle hooks for your application, based on the application type you are developing. The Habitat core team provides Scaffoldings for developers to leverage so if you are using a supported language you can focus on building your application instead of customizing your plans. Users can create their own customized Scaffolding to facilitate re-usability of common patterns in your organization for developing, building, and running your applications.

## Getting Started

To begin using Scaffolding, you will need to add the appropriate `pkg_scaffolding` setting to your plan.sh file.

    pkg_name=MY_APP
    pkg_origin=MY_ORIGIN
    pkg_version=MY_VERSION
    pkg_scaffolding=core/scaffolding-ruby

You can also use the cli command `hab plan init -s [SCAFFOLDING] [PKG_NAME]` to initialize a plan with a given scaffolding. Read more on the [CLI documentation page](/docs/reference/habitat-cli/#hab-plan-init) or see how to use it in the [Build a Sample App](/tutorials/sample-app/mac/) tutorial.

* Available scaffolding values: `ruby`, `node`


## Available Scaffolding

* [core/scaffolding-ruby](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-ruby/)
* [core/scaffolding-node](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-node)

### Coming Soon!

* core/scaffolding-go
* core/scaffolding-python
* core/scaffolding-java
* core/scaffolding-gradle 

## Variables

These are variables which each Scaffolding provides, allowing a Plan author to use a variable to override a particular behavior.  Please see the appropriate Scaffolding documentation for details.

## Scaffolding Internals

A language or framework Scaffolding is shipped as a Habitat package, which means that each Scaffolding runtime dependency becomes a build dependency for the application being built.

### lib/scaffolding.sh File

To create scaffolding, a package must contain a `lib/scaffolding.sh` file which gets sourced by the build program running Bash.

### scaffolding_load() Function

A optional function named `scaffolding_load()` may be created in `lib/scaffolding.sh` which will be called early in the build program which allows a Scaffolding author to control and augment the `pkg_deps` and `pkg_build_deps` arrays. At this point, no other build or run dependencies have been resolved so the code in this function can only rely on what the build program provides or software pulled in via the Scaffolding's Plan.

### Default Build Phases Implementations

The remainder of the `lib/scaffolding.sh` contains one or more default implementations for the build phases. These include, but are not limited to:

* `do_default_prepare()`
* `do_default_build()`
* `do_default_install()`
