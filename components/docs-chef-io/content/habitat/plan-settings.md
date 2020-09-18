+++
title = "Plan Settings"
description = "Define basic metadata about your artifact with plan settings"

[menu]
  [menu.habitat]
    title = "Plan Settings"
    identifier = "habitat/reference/plan-settings"
    parent = "habitat/reference"
+++

Habitat reserves some names for internal use. You can set all of these values in your plan and use them as variables in your Habitat code.

For example:

```plan.sh
# Set the packag name
pkg_name=two-tier-app
# Call the package by name
${pkg_name}
```

```plan.ps1
# Set the packag name
pkg_name=two-tier-app
# Call the package by name
${"pkg_name"}
```

## General Settings

FORMAT:
setting
: Short definition. How to use. How habitat uses. If required other settings. Type: String, array, boolean,etc. Default: if any. Optional/Required

pkg_name
: Sets the name of the package. Can contain upper and lowercase letters, numbers, dashes, and underscores. By default, Chef Habitat uses `pkg_name`, `pkg_origin`, and `pkg_version` to create the fully-qualified package name. Type: string. _Required_.

```bash
pkg_name=zlib
```
```powershell
$pkg_name="zlib"
```

pkg_origin
: The name of the origin for this package. Can contain upper and lowercase letters, numbers, dashes, and underscores. The `HAB_ORIGIN` environment variable overrides the `pkg_origin` Type: string. _Required_.

```bash
pkg_origin=Habitat
```
```powershell
$pkg_origin="Habitat"
```

pkg_version
: Sets the version of the package By default, Chef Habitat uses `pkg_name`, `pkg_origin`, and `pkg_version` to create the fully-qualified package name. You can set the value through the `pkg_version()` function. Type: string. _Required_.

```bash
pkg_version=1.2.8
```
```powershell
$pkg_version="1.2.8"
```

pkg_maintainer
: The name and email address of the package maintainer. Type: string._Optional_.

```bash
pkg_maintainer="Your Name <someone@example.com>"
```
```powershell
$pkg_maintainer="Your Name <someone@example.com>"
```

pkg_license
: One or more [valid software licenses](https://spdx.org/licenses/) that relate to this package. Type: array. _Optional_.

```bash
pkg_license=('Apache-2.0')
```
```powershell
$pkg_license=("'Apache-2.0'")
```

> Note: If your package has a custom license, use a string literal matching the title of the license. For example, you'll see `pkg_license=('Boost Software License')` for the `cmake` plan.

pkg_source
: A URL that specifies the location from which to download an external source. Any valid `wget` url will work. Typically, the relative path for the URL typically contains the `pkg_name` and `pkg_version` values. Type: URL. _Optional_.

```bash
pkg_source=http://downloads.sourceforge.net/project/libpng/$pkg_name/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
```
```powershell
$pkg_source="http://downloads.sourceforge.net/project/libpng/$pkg_name/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz"
```

pkg_filename
: The filename for the finished artifact. By default, Chef Habitat ] constructs this from `pkg_name` and `pkg_version`. Type: string. _Optional_.

```bash
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
```

```powershell
$pkg_filename="${pkg_name}-${pkg_version}.tar.gz"
```

pkg_shasum
: The sha-256 sum of the downloaded `pkg_source`. If you do not have the checksum, generate it by downloading the source and using the `sha256sum` or `gsha256sum` tools. Override with `do_verify()`. When the value is unset or incorrect and you do not override it with `do_verify()`, then the build output of your package will show the expected value. Type: varchar(64) or char(64). _Required_ when providing a valid URL is provided for `pkg_source`, but is otherwise _Optional_.

```bash
pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
```
```powershell
$pkg_shasum="36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d"
```

pkg_deps
: The dependencies that your packages needs at runtime. Refer to packages at three levels of specificity: origin/package, origin/package/version, or origin/package/version/release. Type: array. _Optional_.

```bash
pkg_deps=(core/glibc core/pcre core/openssl core/zlib)
```
```powershell
$pkg_deps="(core/glibc core/pcre core/openssl core/zlib)"
```

pkg_build_deps
: The dependencies your package requires at build time. Type: array. _Optional_.

```bash
pkg_build_deps=(core/gcc core/linux-headers)
```
```powershell
$pkg_build_deps="(core/gcc core/linux-headers)"
```

pkg_lib_dirs
: An array of paths, relative to the final install of the software, where libraries can be found. Used to populate `LD_FLAGS` and `LD_RUN_PATH` for software that depends on your package. _Optional_.

```bash
pkg_lib_dirs=(lib)
```

pkg_include_dirs
: An array of paths, relative to the final install of the software, where headers can be found. Used to populate `CFLAGS` for software that depends on your package. _Optional_.

```bash
pkg_include_dirs=(include)
```

pkg_bin_dirs
: An array of paths, relative to the final install of the software, where binaries can be found. Used to populate `PATH` for software that depends on your package. _Optional_.

```bash
pkg_bin_dirs=(bin)
```

pkg_pconfig_dirs
: An array of paths, relative to the final install of the software, where pkg-config metadata (.pc files) can be found. Used to populate `PKG_CONFIG_PATH` for software that depends on your package. _Optional_.

```bash
pkg_pconfig_dirs=(lib/pkgconfig)
```

pkg_svc_run
: The command for the Supervisor to execute when starting a service. This setting requires `pkg_bin_dirs`  to place package binaries in the path. If your package hs complex start-up behaviors, use a [run hook](#hooks) instead. Omit this setting for packages that are designed for consumption by other packages instead of being run directly by a Supervisor.  _Optional_.

```bash
pkg_svc_run="haproxy -f $pkg_svc_config_path/haproxy.conf"
```

pkg_exports
: Configuration data that will be passed between peers. The keys in this array are used with `pkg_exposes` and for any consuming services that set `pkg_binds` or `pkg_binds_optional`. An [associative array](http://www.linuxjournal.com/content/bash-associative-arrays) in Bash or a `hashtable` in Powershell.  Type: array. _Optional_.

```bash
pkg_exports=(
  [port]=server.port
  [host]=server.host
  [ssl-port]=ssl.port
)
```

In this example, the corresponding `default.toml` file would have the following key/value pairs defined:

```toml default.toml
    [server]
    port = 80
    host = "www.example.com"

    [ssl]
    port = 443
```

pkg_exposes
: An array of `pkg_exports` keys containing default values for the ports that this package exposes. These values are used as sensible defaults for other tools, such as when exporting a package to a container format. _Optional_.

```bash
pkg_exposes=(port ssl-port)
```

  > Note: In addition to specifying the keys you defined in `pkg_exports`, you must have a default.toml file indicating the port values to expose.

pkg_binds
: An associative array (or `hashtable` in Powershell) representing services which you depend on and the configuration keys that you expect the service to export (by their `pkg_exports`). These binds *must* be set for the Supervisor to load the service. The loaded service will wait to run until its bind becomes available. If the bind does not contain the expected keys, the service will not start successfully. _Optional_.

```bash
pkg_binds=(
  [database]="port host"
)
```

pkg_binds_optional
: Same as `pkg_binds` but these represent optional services to connect to. _Optional_.

```bash
pkg_binds_optional=(
  [storage]="port host"
)
```

pkg_interpreters
: An array of interpreters used in [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)) lines for scripts. Specify the subdirectory where the binary is relative to the package, for example, `bin/bash` or `libexec/neverland`, since binaries can be located in directories besides `bin`. This list of interpreters will be written to the metadata INTERPRETERS file, located inside a package, with their fully-qualified path. Then these can be used with the fix_interpreter function. For more information on declaring shebangs in Chef Habitat, see [Plan hooks](#hooks), and for more information on the fix_interpreter function, see [Plan utility functions](#plan-helpers). _Optional_.

```bash
pkg_interpreters=(bin/bash)
```

pkg_svc_user
: The user to run the service as. Default: `hab`. On Windows, if the `hab` user does not exist then the service will run under the same account as the Supervisor. _Optional_.

```bash
pkg_svc_user=hab
```

pkg_svc_group
: Assigned service group for the package. **Not used in a `plan.ps1`.** Type: string. Default: `hab`. _Optional_.

```bash
pkg_svc_group=$pkg_svc_user
```

pkg_shutdown_signal
: The signal to send the service to shutdown. **Not used in a `plan.ps1`.** Default: `TERM`. _Optional_.

```bash
pkg_shutdown_signal=HUP
```

pkg_shutdown_timeout_sec
: The number of seconds to wait for a service to shutdown. After this interval the service will forcibly be killed. **Not used in a `plan.ps1`.** Default: `8`. _Optional_.

```bash
pkg_shutdown_timeout_sec=$pkg_shutdown_timeout_sec
```

pkg_description
: A short description of the package. It can be a simple string, or you can create a multi-line description using markdown to provide a rich description of your package. This description will be displayed on the Web app when users search for or browse to your package. Type: Text._Required_ for [core](https://github.com/habitat-sh/core-plans) plans, but otherwise _Optional_.

```bash
pkg_description=$(cat << EOF
  # My package description
  This is the package for the foo library. It's pretty awesome.
  EOF
  )
```

> Note: Escape all special characters other than `#`. The hab-plan-build script interprets unescaped characters as code during the package build.

pkg_upstream_url
: An upstream project homepage or website URL. _Optional_.

```bash
pkg_upstream_url=https://github.com/myrepo
```
