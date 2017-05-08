# Scaffolding

Scaffolding is default implementations of the build phases and runtime hooks for your application. Each scaffolding understands how your application was built, which allows it to can create the appropriate hooks and add in the correct runtime dependnencies when creating the package for your application. Scaffoldings also provide some default health check hooks where appropriate to ensure your application is functioning reliably. Customized Scaffolding can be created to facilitate re-usability of common patterns in your organization for developing, building, and running your applications.

> Note: Currently only the Ruby scaffolding automatically adds in the appropriate dependencies and configuration settings based on how your application is currently being built. This behavior will be pushed into the other available scaffoldings soon.

## Getting Started

To begin using Scaffolding, you will need to add the appropriate `pkg_scaffolding` setting to your plan.sh file.

    pkg_name=MY_APP
    pkg_origin=MY_ORIGIN
    pkg_version=MY_VERSION
    pkg_scaffolding=core/scaffolding-ruby

## Embedded Plans (Coming Soon!)
Each scaffolding defines a default value for `pkg_source` in [URI syntax](https://tools.ietf.org/html/rfc3986). You can override this value within your plan should you application source live in a non-standard location.

## Available Scaffolding

* [core/scaffolding-go](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-go)
* [core/scaffolding-go17](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-go17)
* [core/scaffolding-python](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-python)
* [core/scaffolding-python2](https://github.com/habitat-sh/core-plans/tree/master/scaffolding-python2)
* [core/scaffolding-ruby](https://github.com/habitat-sh/core-plans/blob/master/scaffolding-ruby/doc/reference.md)

### Coming Soon!

* core/scaffolding-python-gunicorn
* core/scaffolding-python2-gunicorn
* core/scaffolding-python-uwsgi
* core/scaffolding-python2-uwsgi
* core/scaffolding-ruby23
* core/scaffolding-ruby-rails-unicorn
* core/scaffolding-ruby-rails41-unicorn
* core/scaffolding-ruby-rails40-unicorn
* core/scaffolding-ruby23-rails-unicorn
* core/scaffolding-ruby23-rails41-unicorn
* core/scaffolding-ruby32-rails40-unicorn
* core/scaffolding-java-maven
* core/scaffolding-java-maven-tomcat
* core/scaffolding-java-maven-jboss
* core/scaffolding-java-gradle
* core/scaffolding-java-gradle-tomcat
* core/scaffolding-java-gradle-jboss
* core/scaffolding-java7-ant

## Callbacks
Each scaffolding defines a set of callbacks which are unique to the scaffolding type. Please see the documentation for the appropriate scaffolding for details on each scaffolding's callbacks. Depending on the language, some default callbacks are defined as overrides since they are are often not used for building packages in that language.

### Internal Callbacks

#### `scaffolding_load`

The default_begin phase is executed prior to loading the scaffolding. This internal callback allow the scaffolding to run anything we need to execute before the download and build.

## Variables

These are variables which each scaffolding honors, allowing the plan author to consume the value to override a particular behavior.  Please see the documentation for the appropriate scaffolding for details on each scaffolding's callbacks.

## Run Hooks (Coming Soon!)

#### Health Checks (Coming Soon!)

Each scaffolding comes with a default health check for your application.

## Scaffolding Internals

Each scaffolding defines it's own `pkg_build_deps` and `pkg_deps` arrays with any required packages and is merged into the TDEPS (transitive dependencies) as it would any other package.

### `scaffolding.sh`

To create scaffolding, a package must contain a `lib/scaffolding.sh` bash script. If the file exists, and a plan contains a valid `pkg_scaffolding` variable it will be sourced into the plan-build.
