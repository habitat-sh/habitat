---
title: Habitat plan syntax reference
---

# Plan syntax guide

When defining a plan, there are several different settings, variables, and functions that you can use to set up specific installation and configuration details. Because plans are simply script files, you have a lot of flexibility in how you build and define the configuration your application.

This syntax guide is divided into six parts:

- [Basic settings](#basic-settings)
- [Variables](#variables)
- [Callbacks](#callbacks)
- [Hooks](#hooks)
- [Runtime configuration settings](#runtime-configuration-settings)
- [Utility functions](#utility-functions)

## Basic settings
The following settings are defined at the beginning of your plan. They specify basic information about your plan such as name, version, and dependencies.

pkg_name
: Required. Sets the name of the package. This will be used in along with `pkg_origin`, and `pkg_version` to define the fully-qualified package name, which determines where the package is installed to on disk, how it is referred to in package metadata, and so on.

  ~~~
  pkg_name=zlib
  ~~~

pkg_origin
: Required unless overridden by the `HAB_ORIGIN` environment variable. The origin is used to denote a particular upstream of a package.

  ~~~
  pkg_origin=Habitat
  ~~~

pkg_version
: Required. Sets the version of the package.

  ~~~
  pkg_version=1.2.8
  ~~~

pkg_maintainer
: Optional. The name and email address of the package maintainer.

  ~~~
  pkg_maintainer="Your Name <someone@example.com>"
  ~~~

pkg_license
: Optional. An array of [valid software licenses](https://spdx.org/licenses/) that relate to this package.

  ~~~
  pkg_license=('Apache-2.0')
  ~~~

> Note: If your package has a custom license, use a string literal matching the title of the license. For example, you'll see `pkg_license=('Boost Software License')` for the `cmake` plan.

pkg_source
: Required. A URL that specifies where to download the source from. Any valid `wget` url will work. Typically, the relative path for the URL is partially constructed from the `pkg_name` and `pkg_version` values; however, this convention is not required.

  ~~~
  pkg_source=http://downloads.sourceforge.net/project/libpng/$pkg_name/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
  ~~~

> Note: If your package does not require downloading any source code, you must enter a non-empty string for the value and override callbacks for **do_download()** and **do_unpack()**. See [Plan callbacks](#plan-callbacks) for more information.

pkg_filename
: Optional. The resulting filename for the download, typically constructed from the `pkg_name` and `pkg_version` values.

  ~~~
  pkg_filename=${pkg_name}-${pkg_version}.tar.gz
  ~~~

pkg_shasum
: Required if a valid URL is provided for `pkg_source` or unless **do_verify()** is overridden. The value for `pkg_shasum` is a sha-256 sum of the downloaded `pkg_source`. If you do not have the checksum, you can easily generate it by downloading the source and using the `sha256sum` or `gsha256sum` tools. Also, if you do not have **do_verify()** overridden, and you do not have the correct sha-256 sum, then the expected value will be shown in the build output of your package.

  ~~~
  pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
  ~~~

pkg_deps
: Optional. An array of package dependencies needed at runtime. You can refer to packages at three levels of specificity: origin/package, origin/package/version, or origin/package/version/release.

  ~~~
  pkg_deps=(core/glibc core/pcre core/openssl core/zlib)
  ~~~

pkg_build_deps
: Optional. An array of the package dependencies needed only at build time.

  ~~~
  pkg_build_deps=(core/gcc core/linux-headers)
  ~~~

pkg_lib_dirs
: Optional. An array of paths, relative to the final install of the software,
where libraries can be found. Used to populate `LD_FLAGS` and
`LD_RUN_PATH` for software that depends on your package.

  ~~~
  pkg_lib_dirs=(lib)
  ~~~

pkg_include_dirs
: Optional. An array of paths, relative to the final install of the software,
where headers can be found. Used to populate `CFLAGS` for software
that depends on your package.

  ~~~
  pkg_include_dirs=(include)
  ~~~

pkg_bin_dirs
: Optional. An array of paths, relative to the final install of the software,
where binaries can be found. Used to populate `PATH` for software
that depends on your package.

  ~~~
  pkg_bin_dirs=(bin)
  ~~~

pkg_pconfig_dirs
: Optional. An array of paths, relative to the final install of the
software, where pkg-config metadata (.pc files) can be found. Used to
populate `PKG_CONFIG_PATH` for software that depends on your package.

  ~~~
  pkg_pconfig_dirs=(lib/pkgconfig)
  ~~~

pkg_svc_run
: Optional. The command for the supervisor to execute when starting a service. You can omit this setting if your package is not intended to be run directly by a supervisor.

  ~~~
  pkg_svc_run="bin/haproxy -f $pkg_svc_config_path/haproxy.conf"
  ~~~

> Note: You should use a [run hook](#plan-hooks) instead if you have complex start up behavior.

pkg_expose
: Optional. An array of ports this service exposes when you create a Docker image from your package.

  ~~~
  pkg_expose=(80 443)
  ~~~


pkg_interpreters
: Optional. An array of interpreters used in [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)) lines for scripts. Specify the subdirectory where the binary is relative to the package, for example, `bin/bash` or `libexec/neverland`, since binaries can be located in directories besides `bin`. This list of interpreters will be written to the metadata INTERPRETERS file, located inside a package, with their fully-qualified path.  Then these can be used with the fix_interpreter function. For more information on declaring shebangs in Habitat, see [Plan hooks](#plan-hooks), and for more information on the fix_interpreter function, see [Plan utility functions](#plan-utility-functions).

  ~~~
  pkg_interpreters=(bin/bash)
  ~~~

pkg_svc_user
: Optional. The user to run the service as. The default is `hab`.

  ~~~
  pkg_svc_user=hab
  ~~~

pkg_svc_group
: Optional. The group to run the service as. The default is `hab`.

  ~~~
  pkg_svc_group=$pkg_svc_user
  ~~~

pkg_description
: Optional. A short description of the package. It can be a simple string, or you can create a multi-line description using markdown to provide a rich description of your package. {::comment} This description will be displayed on the Web app when users search for or browse to your package. {:/comment}

  ~~~
  pkg_description=$(cat << EOF
    # My package description
    This is the package for the foo library. It's pretty awesome.
    EOF
    )
  ~~~

> Note: Any special characters other than `#` will have to be escaped; otherwise, they could be interpreted by the hab-plan-build script when the package is built.

pkg_upstream_url
: Optional. An upstream project homepage or website URL.

  ~~~
  pkg_upstream_url=https://github.com/myrepo
  ~~~

***

## Variables
The following variables can be used in your plans to help get binaries and libraries to build and install in the correct locations in your package.

$pkg_prefix
: This variable is the absolute path for your package.

$pkg_dirname
: Set to `${pkg_name}-${pkg_version}` by default. If a .tar file extracts to a directory that's different from the filename, then you would need to override this value to match the directory name created during extraction.

$pkg_svc_path
: Where the running service is located. `$HAB_ROOT_PATH/svc/$pkg_name`

$pkg_svc_data_path
: Where the running service data is located. `$pkg_svc_path/data`

$pkg_svc_files_path
: Where the gossiped configuration files are located. `$pkg_svc_path/files`

$pkg_svc_var_path
: Where the running service variable data is located. `$pkg_svc_path/var`

$pkg_svc_config_path
: Where the running service configuration is located. `$pkg_svc_path/config`

$pkg_svc_static_path
: Where the running service static data is located. `$pkg_svc_path/static`

$HAB_CACHE_SRC_PATH
: The default path where source archives are downloaded, extracted, & compiled.

$HAB_CACHE_ARTIFACT_PATH
: The default download root path for packages.

$HAB_PKG_PATH
: The root path containing all locally installed packages.

$PLAN_CONTEXT
: The location on your local dev machine for the files in your plan directory.

$CFLAGS
: C compiler options.

$LDFLAGS
: C linker options.

$PREFIX
: Where to install the software; same as `$pkg_prefix`

$LD_RUN_PATH
: Where to find the binaries at run time.

***

## Callbacks
When defining your plan, you have the flexibility to override the default behavior of Habitat in each part of the package building stage through a series of callbacks. To define a callback, simply create a shell function of the same name in your plan.sh file and then write your script. If you do not want to use the default callback behavior, you must override the callback and `return 0` in the function definition.

These callbacks are listed in the order that they are called by the package build script.

do_begin()
: There is no default implementation of this callback. You can use it to execute any arbitrary commands before anything else happens.

do_download()
: The default implementation is that the software specified in $pkg_source is downloaded, checksum-verified, and placed in *$HAB_CACHE_SRC_PATH/$pkg_filename*, which resolves to a path like `/hab/cache/src/filename.tar.gz`. You should override this behavior if you need to change how your binary source is downloaded, if you are not downloading any source code at all, or if your are cloning from git. If you do clone a repo from git, you must override **do_verify()** to return 0.

do_verify()
: The default implementation tries to verify the checksum specified in the plan against the computed checksum after downloading the source tarball to disk. If the specified checksum doesn't match the computed checksum, then an error and a message specifying the mismatch will be printed to stderr. You should not need to override this behavior unless your package does not download any files.

do_check()
: The default implementation runs nothing during post-compile.  An example of a command you might use in this callback is `make test`. To use this callback, two conditions must be true. A) `do_check()` function has been declared, B) `DO_CHECK` environment variable exists and set to true, `env DO_CHECK=true`.

do_clean()
: The default implementation removes the *HAB_CACHE_SRC_PATH/$pkg_dirname* folder in case there was a previously-built version of your package installed on disk. This ensures you start with a clean build environment.

do_unpack()
: The default implementation extracts your tarball source file into *HAB_CACHE_SRC_PATH*. The supported archives are: .tar, .tar.bz2, .tar.gz, .tar.xz, .rar, .zip, .Z, .7z. If the file archive could not be found or was not supported, then a message will be printed to stderr with additional information.

do_prepare()
: There is no default implementation of this callback. At this point in the build process, the tarball source has been downloaded, unpacked, and the build environment variables have been set, so you can use this callback to perform any actions before the package starts building, such as exporting variables, adding symlinks, and so on.

do_build()
: The default implementation is to update the prefix path for the configure script to use $pkg_prefix and then run `make` to compile the downloaded source. This means the script in the default implementation does `./configure --prefix=$pkg_prefix && make`. You should override this behavior if you have additional configuration changes to make or other software to build and install as part of building your package.

do_install()
: The default implementation is to run `make install` on the source files and place the compiled binaries or libraries in *HAB_CACHE_SRC_PATH/$pkg_dirname*, which resolves to a path like `/hab/cache/src/packagename-version/`. It uses this location because of **do_build()** using the `--prefix` option when calling the configure script. You should override this behavior if you need to perform custom installation steps, such as copying files from HAB_CACHE_SRC_PATH to specific directories in your package, or installing pre-built binaries into your package.

do_strip()
: The default implementation is to strip any binaries in $pkg_prefix of their debugging symbols. You should override this behavior if you want to change how the binaries are stripped, which additional binaries located in subdirectories might also need to be stripped, or whether you do not want the binaries stripped at all.

do_end()
: There is no default implementation of this callback. This is called after the package has been built and installed. You can use this callback to remove any temporary files or perform other post-install clean-up actions.


***


## Hooks
Each plan can have a `hooks` subdirectory that specifies any of the hooks or asynchronous callbacks described in this section. Each hook is a `bash` script with a [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)) defined at the top of the script to specify the interpreter to be used.

file_updated
: File location: `<plan>/hooks/file_updated`
: This hook is run whenever a configuration file that is not related to a user or about the state of the service instances is updated.


health_check
: File location: `<plan>/hooks/health_check`
: This hook is run when the Habitat HTTP API receives a request at `/health`.

  The `health_check` script must return a valid exit code from the list below.

  - **0**- ok
  - **1**- warning
  - **2**- critical
  - **3**- unknown
  - any other code - failed health check with additional output taken from `health_check` stdout.


  A `health_check` hook can use the following as a template:

  ~~~ bash
  #!/bin/sh

  # default return code is 0
  rc=0
  program_that_returns_a_status
  case $? in
    0)
      rc=1 ;;
    3)
      rc=0 ;;
    4)
      rc=2 ;;
    *)
      rc=3 ;;
  esac

  exit $rc
  ~~~

init
: File location: `<plan>/hooks/init`

  This hook is run when a Habitat topology starts.

reconfigure
: File location: `<plan>/hooks/reconfigure`

  This hook is run when service configuration information has changed through a set of Habitat services that are peers with each other.

run
: File location: `<plan>/hooks/run`

  This hook is run when one of the following conditions occur:

  - The main topology starts, after the `init` hook has been called.
  - When a package is updated, after the `init` hook has been called.
  - When the package config changes, after the `init` hook has been called, but before a `reconfigure` hook is called.

***

## Runtime configuration settings
The following configuration settings can be used during a Habitat service's lifecycle. This means that you can use these settings in any of the plan hooks, such as init, or run, and also in any templatized configuration file for your application or service.

These configuration settings are referenced using the [Handlebars.js](https://github.com/wycats/handlebars.js/)) version of  [mustache-style](https://mustache.github.io/mustache.5.html) tags. For an example on how these settings are used in plan hooks, see [Add hooks to your plan](/tutorials/getting-started-add-hooks) in the getting started tutorial.


### sys
These are service settings specified by Habitat and correspond to the network information of the running Habitat service.

ip
: The IP address of the running service.

hostname
: The hostname of the running service.


### pkg
These are package settings specified by Habitat and correspond to the the settings of the package when it was built and installed.

origin
: Denotes a particular upstream of a package. This value is pulled from the pkg_origin setting in a plan.

name
: The name of the package. This value is pulled from the pkg_name setting in a plan.

version
: The version of a package. This value is pulled from the pkg_version setting in a plan.

release
: The UTC datetime stamp when the package was built. This value is specified in _YYYYMMDDhhmmss_ format.

ident
: The fully-qualified identifier of a package that consists of origin/name/version/release.

deps
: An array of runtime dependencies for your package based on the pkg_deps setting in a plan.

exposes
: The port(s) to expose for an application or service. This value is pulled from the pkg_expose setting in a plan.

path
: The location where the fully-qualified package is installed.

svc_path
: The root location of the source files for the Habitat service.

svc_config_path
: The location of any configuration files for the Habitat service.

svc_data_path
: The location of any data files for the Habitat service.

svc_files_path
: The location of any gossiped configuration files for the Habitat service.

svc_static_path
: The location of any static content for the Habitat service.

svc_var_path
: The location of any variable state data for the Habitat service.

svc_user
: The value of pkg_svc_user specified in a plan.

svc_group
: The value of pkg_svc_group specified in a plan.

svc_user_default
: The default user determined by the Habitat supervisor. `svc_user_default` will contain one of the following values, tested in order:
- `svc_user` if specified in the plan
- `hab` if the user exists
- the current user id

svc_group_default
: The default group determined by the Habitat supervisor. `svc_group_default` will contain one of the following values, tested in order:
- `svc_group` if specified in the plan
- `hab` if the group exists
- the effective group id

### cfg
These are settings defined in your templatized configuration file. The values for those settings are pulled from the `default.toml` file included in your package. 

***

## Utility functions
The following helper functions can be useful in your plan to help you build your package correctly.

attach()
: Attaches your script to an interactive debugging session, which lets you check the state of variables, call arbitrary functions, and turn on higher levels of logging by using the `set -x` command and switch.

  To use attach, add `attach` to any callback or part of your plan.sh file and the debugging session with start up when hab-plan-build comes to that part in the file.

pkg_path_for()
: Returns the path for a build or runtime package dependency on stdout from the list of dependencies referenced in pkg_deps or pkg_build_deps. This is useful if you need to install or reference specific dependencies from within a callback, such as `do_build()` or `do_install()`.

  Here's an example of how to use this function to retrieve the path to the perl binary in the core/perl package:

  ~~~
  _perl_path="$(pkg_path_for core/perl)/bin/perl"
  ~~~

fix_interpreter()
: Edits the `#!` shebang of the target file in-place. This is useful for changing hardcoded paths defined by your source files to the equivalent path in a Habitat package. You must include the required package that provides the expected path for the shebang in pkg_deps. This function performs a greedy match against the specified interpreter in the target file(s).

  To use this function in your plan, you must specify the following arguments:
    1. The target file or files
    2. The name of the package that contains the interpreter
    3. The relative directory and binary path to the interpreter

  For example, to replace all the files in `node_modules/.bin` that have `#!/usr/bin/env` with the coreutils path to `bin/env` (/hab/pkgs/core/coreutils/8.24/20160219013458/bin/env), you must quote the wildcard target as shown below.

  ~~~
  fix_interpreter "node_modules/.bin/*" core/coreutils bin/env
  ~~~

  For a single target, reference the file directly:

  ~~~
  fix_interpreter node_modules/.bin/concurrent core/coreutils bin/env
  ~~~

pkg_interpreter_for()
: Returns the path for the given package and interpreter by reading it from the INTERPRETERS metadata in the package. The directory of the interpreter needs to be specified, as an interpreter binary might live in `bin`, `sbin`, or `libexec`, depending on the software.

  The following shows how to call pkg_interpreter_for with the package and interpreter arguments specified.

  ~~~
  pkg_interpreter_for core/coreutils bin/env
  ~~~

  This function will return 0 if the specified package and interpreter were found, and 1 if the package could not be found or the interpreter is not specified for that package.

abspath()
: Return the absolute path for a path, which might be absolute or relative.

exists()
: Checks that the command exists. Returns 0 if it does, 1 if it does not.

build_line()
: Print a line of build output. Takes a string as its only argument.

~~~
build_line "Checksum verified - ${pkg_shasum}"
~~~

warn()
: Print a warning line on stderr. Takes a string as its only argument.

~~~
warn "Checksum failed"
~~~

debug()
: Prints a line only if the `$DEBUG` environment value is set to 1. The `debug` function takes a string as its only argument.

~~~
DEBUG=1
debug "Only if things are set"
~~~

exit_with()
: Exits the program with an error message and a status code.

~~~
exit_with "Something bad happened" 55
~~~

trim()
: Trims leading and trailing whitespace characters from a bash variable.
