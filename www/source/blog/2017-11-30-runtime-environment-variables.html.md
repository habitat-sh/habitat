---
title: Introducing Runtime Environment Variables
date: 2017-11-30
author: Christopher Maier
tags: blog, supervisor, packaging
category: supervisor, packaging
classes: body-article
---

Today, we'd like to introduce a new feature for Habitat packages that should make it easier to create packages that work the way you want automatically without a lot of extra work from plan authors. Lots of users have asked for this for a while, and we're happy to say that you can now specify runtime environment variables in your Linux Habitat plans. Let's see what this looks like, and what it offers plan authors.

## Motivation

When the Habitat Supervisor starts your application, it controls the environment in which the process runs. Among other variables, it will automatically generate a `PATH` variable that includes the paths to all the binaries from your application, as well as the binaries from your application's dependencies. This is great, because this is something that a) you'll want every time anyway, and b) is irritating to do manually in all your hook scripts. All the information exists in the plan metadata and can be constructed deterministically, so it makes sense for Habitat to take care of this work for you.

While `PATH` is perhaps the most obvious example of an environment variable that we'd like to have automatically managed, it is by no means the only one. Java developers would like to have `JAVA_HOME` set according to the Java runtime dependency they're using, and `CLASSPATH` to include `*.jar` files from their application (as well as any dependencies that might have needed libraries). Ruby developers would like `GEM_HOME` and `GEM_PATH` set in similar ways, just as Python developers need to manage `PYTHONHOME` and `PYTHONPATH`. I'm sure you can think of more examples from your own applications; just take a look at your own Habitat hook scripts and see if you have line upon line of boiler plate `export` calls to create an appropriate runtime environment based on your application's dependencies.

It is now possible for Habitat packages to set values for environment variables and propagate those values to applications that depend on them. This means that a given variable can be set by the package most directly responsible for it (as with our `JAVA_HOME` example), and more things should "Just Work", without requiring a lot of extra ceremony and boilerplate.

Let's take a look at how we can use this new feature.

## How it Works

Since Habitat will be managing these environment variables for us, we won't simply add `export` calls directly into our Habitat plan files. Instead, we have a new callback function for this purpose: `do_setup_environment`. This callback runs after all dependencies have been retrieved, but before the build itself starts. This allows us to compose an environment from the dependencies, then layer on any changes for the current package, thus preserving the proper order of operations. Once we've assembled the environment, we also apply it to the Studio's environment, thus making the values available at build-time as well.

There are a few additional functions we've added that you can run in the `do_setup_environment` hook that give you control over how environment variables are handled. We'll look at them in turn.

* set_runtime_env [-f] VARIABLE_NAME VALUE

This function, as the name suggests, allows you to _set_ an environment variable's value. If one of your dependencies has already declared a value for this, it will result in a build failure, protecting you from inadvertently breaking anything. If you really do want to replace the value, however, you can supply the `-f` flag (for "force").

* push_runtime_env VARIABLE_NAME VALUE

For multi-valued variables like `PATH`, `CLASSPATH`, `GEM_PATH`, etc., (which we refer to as "aggregate" variables), you often want to push a new value onto any that exists currently, rather than resetting the value altogether. For this, we use `push_runtime_env`; the value of the variable is first assembled from any dependencies, and then the value from the current package is prepended. Duplicates are automatically removed.

We process all the package's runtime dependencies in order to assemble these runtime environment variables. We also perform a similar process on the package's build dependencies as well, generating a build-time environment. Plan authors can use the `set_buildtime_env` and `push_buildtime_env` helper functions in the `do_setup_environment` callback to modify these variables as desired. As they are purely build-time values, they will _not_ be present when your service is being run by the Habitat Supervisor.

As an example, this is how we might use this feature in a Java runtime package to automatically set `JAVA_HOME` for any package that uses it as a dependency:

```sh
# ...
do_setup_environment() {
  set_runtime_env JAVA_HOME "${pkg_prefix}"
}
# ...
```

Here is how we might enable a Ruby project to add its gems to the `GEM_PATH` of any project that uses it:

```sh
#...
do_setup_environment() {
   push_runtime_env GEM_PATH "${pkg_prefix}"
}
#...
```

## Escape Hatches

You might be wondering how Habitat knows what separator to use for variables like `PATH`, or how it knows the difference between "aggregate" variables and non-aggregate, or "primitive", variables. While the build process knows about some common environment variables and how to properly process them, it doesn't know about everything. In the case that Habitat does not know how to properly process your variables, we have added a hinting system that lets you control how variables are treated.

By default, Habitat treats all variables as "primitive" variables. If you are working with a value that is actually an "aggregate" type, though, you can set a special environment variable. For example if you have a variable named `FOO` that is an aggregate type, you can add `export HAB_ENV_FOO_TYPE=aggregate` somewhere in the top level of your plan.

Similarly, Habitat defaults to using the colon (`:`) as a separator for aggregate variables. If our hypothetical `FOO` variable uses a semicolon (`;`) as a separator instead, we can add `export HAB_ENV_FOO_SEPARATOR=;` at the top level of the plan.

In all cases, when Habitat is assuming a default strategy, it will emit log messages to notify you of that fact, along with these instructions on how to change the behavior.

If you discover common environment variables that Habitat doesn't currently treat appropriately, feel free to request an addition to the codebase, or even to submit a pull request yourself.

## Metadata

As you may already know, Habitat generates a number of metadata files as part of the package build process, which can be found within the artifacts themselves. With this new feature, several additional files are generated, which both help to drive the operation of the feature, as well as help users reason about and troubleshoot how it works.

The `RUNTIME_ENVIRONMENT` file contains the result of the layering operation of the current package's runtime environment variables on top of those of its dependencies. This is what the build process consults when it processes dependencies, and this is what the Supervisor consults when generating the runtime environment for a supervised process.

Alongside this is a `RUNTIME_ENVIRONMENT_PROVENANCE` file, which provides information on which specific dependencies have influenced the final value of a given variable in the `RUNTIME_ENVIRONMENT` file. This file is not currently consumed by any other software in the Habitat ecosystem, but is provided to help humans troubleshoot their builds.

A `BUILDTIME_ENVIRONMENT` file is also provided, which provides similar information as the `RUNTIME_ENVIRONMENT` file, but drawn from a package's build-time dependencies instead of its runtime dependencies. A `BUILDTIME_ENVIRONMENT_PROVENANCE` file is also provided. Neither of these drive any Habitat features, but are provided for troubleshooting and informative purposes only.

## Conclusion

We're excited about the uses of this feature, and hope that it makes it even easier to run your applications using Habitat.  We're interested in your feedback on this feature; give it a try and [let us know what you think!](http://slack.habitat.sh/)

Special thanks to [George Marshall](https://github.com/georgemarshall), who did a lot of preliminary work on this feature.
