---
title: Package contents
---

# Contents of a Habitat package
During the build process, the hab-plan-build script creates several files that specify dependency, build, and configuration information. When packages are unpacked (extracted) and installed during the initialization phase of a Habitat service, these files define what those packages need to run.

Packages are installed in the `/hab/pkgs/` directory, and then further organized in subdirectories corresponding to fully-qualified package identifiers: `origin/name/version/release`. For more information on package identifiers, see [Packages](/docs/concepts-packages).

## BUILD_DEPS
Fully-qualified package identifiers of any build dependencies that your package depends on. These are listed in the root plan.sh file of your plan directory.

## BUILD_TDEPS
Fully-qualified package identifiers of any runtime dependencies that the build dependencies for your project depend on. This is essentially a flattened tree of dependencies all the way up to the root dependency (`linux-headers` in most cases).

## CFLAGS
Additional switches to be passed to the compiler when this package is used as a build dependency.

## DEPS
Runtime dependencies for your package. These dependencies are processed by Habitat and their corresponding environment variables (such as `PATH` and `LD_LIBRARY_PATH`) are added to the current environment.

## FILES
List of all files in this package along with their blake2b checksums. The FILES file itself is signed using `hab pkg sign` to provide an assurance that its contents haven't been tampered with.

## IDENT
The fully-qualified identifier for the package. The format is `origin/name/version/release`.

## INTERPRETERS
If `pkg_interpreters` is specified in your plan.sh, then this file will be generated and contain a list of absolute paths to any interpreters that a package can provide. Code in a `plan.sh` may use the `fix_interpreter` function to replace hardcoded instances of interpreters, such as `/bin/env`. The location of interpreters in Habitat will be nested under `/hab/pkgs/`. For more information on interpreters, see the fix_interpreter description in [Utility functions](/docs/reference/plan-syntax#utility-functions).

## LDFLAGS
Additional switches to be passed to the compiler when this package is used as a build dependency.

## LD_RUN_PATH
Additional switches to be passed to the compiler when this package is used as a build dependency.

## MANIFEST
A file containing package information, such as checksum, maintainer, build variables, and other metadata specified in plan.sh as well as the contents of the plan.sh itself.

## PATH
An absolute path to the `bin` folder for the package. A fully-qualified package identifier is used, so version and release information is included in the path.

## TARGET
The CPU architecture and platform for the package. The format is `architecture-platform`. For example, x86_64-linux.

## TDEPS
Fully-qualified package identifiers of any runtime dependencies that the runtime dependencies for your project depend on. This is essentially a flattened tree of dependencies all the way up to the root dependency (`linux-headers` in most cases).

## SVC_GROUP
The value of `pkg_svc_group` from a plan. The Habitat supervisor will try to start a service with this group if it exists.

## SVC_USER
The value of `pkg_svc_user` from a plan. The Habitat supervisor will try to start a service with this user if it exists.

## default.toml
If you have defined a default.toml file in the root of your plan, then it will be included in the same relative location within the installed package directory. For more information on configuration and the default.toml file, see [Add configuration to plans](/docs/create-packages-configure/).

## config directory
If you have defined a `config` subdirectory with a templatized configuration file in your plan, then they will be included in the same relative location within the installed package directory. For more information on templatized configuration files, see [Add configuration to plans](/docs/create-packages-configure/).

## hooks directory
If you have defined a `hooks` subdirectory with hook scripts in your plan, then they will be included in the same relative location within the installed package directory.
