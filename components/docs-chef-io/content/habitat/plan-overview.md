+++
title = "Plan Overview"
description = "Documentation for writing Chef Habitat Plan files including configuration templates, binds, and exporting"

[menu]
  [menu.habitat]
    title = "Plan Overview"
    identifier = "habitat/plans/plans-overview Introducing Chef Habitat Plans"
    parent = "habitat/plans"
    weight = 10

+++

Chef Habitat lets you automate the process of application delivery. The plan file is where you define how Chef Habitat builds, deploys, and manages your application.

## Application Delivery Lifecycle

* Write Plan
* Build Package
* Deploy Artifact
* Manage Application

## Writing Plans

The Chef Habitat directory, also called the Chef Habitat Manifest directory, is made up of shell scripts and optional configuration files that define how you download, configure, make, install, and manage your application.

### Types of Plans

* `plan.sh`: Linux or Linux kernel 2
* `plan.ps1`: Windows

On Windows, only a `plan.ps1` will be used and a `plan.sh` will only be used on Linux or Linux kernel 2. So i

## Habitat Directory Structure

The command `hab plan init` creates a `habitat` directory wherever you run it. In most cases you will run the command at the root of your application directory. The `app/habitat` directory has the structure:

```bash plan.sh
app/
└── habitat/
    └── config/
    └── hooks/
    default.toml
    plan.sh
    README.md
```

```powershell plan.ps1
app/
└── habitat/
    └── config/
    └── hooks/
    default.toml
    plan.ps1
    README.md
```

### Simplified Habitat Directory Structure

If your plan does not include hooks or configuration templates and just requires a plan file, you can use a simplified directory structure:

```bash
app/
└── habitat/
    |   plan.sh
```

```powershell
app/
└── habitat/
    |   plan.ps1
```

## Writing a Plans for Multiple Platform Targets

Chef Habitat can build applications for Linux, Linux kernel 2, and Windows operating systems. To write and build plans for multiple targets, create subdirectories for each target operating in either the root of your project or in a top level `habitat` folder. Then save the plan, hooks, and configuration templates specific to a single platform all inside of that target specific folder.

For example, this directory structure represents an application targeting Linux, Linux kernel 2, and Windows:

```bash habitat directory structure with multiple targets
app/
├── habitat/
    ├── x86_64-linux/
    |   |   plan.sh
    |   └── hooks/
    |           run
    ├── x86_64-linux-kernel2/
    |   |   plan.sh
    |   └── hooks/
    |           run
    └── x86_64-windows/
        |   plan.ps1
        └── hooks/
                run
```

```bash application directory structure with multiple targets
app/
├── x86_64-linux/
|   |   plan.sh
|   └── hooks/
|           run
├── x86_64-linux-kernel2/
|   |   plan.sh
|   └── hooks/
|           run
└── x86_64-windows/
    |   plan.ps1
    └── hooks/
            run
```

### Linux and Windows Targets

If you are creating an application that runs on Linux and Windows platform targets, you will need two plan files.

```bash Standard directory structure with multiple targets
app/
├── x86_64-linux/
|   |   plan.sh
|   └── hooks/
|           run
└── x86_64-windows/
    |   plan.ps1
    └── hooks/
            run
```

If your application plans are simple--if they consists of only plan files without additional hooks or configuration files--you can use the simple directory structure:

``` Simple directory with two target plans
app/
└── habitat/
    |   plan.sh
    |   plan.ps1
```

### Linux and Linux Kernel 2 Targets

To build packages and applications that run on both Linux and Linux Kernel 2 hardware, you will need to use target folders for each platform, even if your Chef Habitat manifest directory consists of two plan files without additional hooks or configuration files.

```bash habitat directory structure with multiple Linux targets
app/
├── habitat/
    ├── x86_64-linux/
    |   |   plan.sh
    ├── x86_64-linux-kernel2/
    |   |   plan.sh
```

## Build Plan Lookup

The `build` command looks look for the base of your plan in the following locations:

- `<app>/<target>/`
- `<app>/habitat/<target>/`
- `<app>/`
- `<app>/habitat/`

The best practice when you need plans for different targets is to put each plan in its own target folder. If the `build` command finds more than one plan file, where one plan is inside of a target directory and another is outside of it, it uses the target-specific plan, as long as the target was specified in the build command.

## Plan Anatomy

As a way to start to understand plans, let's look at an example `plan.sh` for [sqlite](http://www.sqlite.org/):

```bash plan.sh
# Metadata
pkg_name=sqlite
pkg_version=3130000
pkg_origin=core
pkg_license=('Public Domain')
pkg_maintainer="The Chef Habitat Maintainers <humans@habitat.sh>"
pkg_description="A software library that implements a self-contained, serverless, zero-configuration, transactional SQL database engine."
# Buildtime Metadata
pkg_upstream_url=https://www.sqlite.org/
pkg_source=https://www.sqlite.org/2016/${pkg_name}-autoconf-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-autoconf-${pkg_version}.tar.gz
pkg_dirname=${pkg_name}-autoconf-${pkg_version}
pkg_shasum=e2797026b3310c9d08bd472f6d430058c6dd139ff9d4e30289884ccd9744086b
pkg_deps=(core/glibc core/readline)
pkg_build_deps=(core/gcc core/make core/coreutils)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)
```

```powershell plan.ps1
$pkg_name="sqlite"
$pkg_version="3130000"
$pkg_origin="core"
$pkg_license=@("Public Domain")
$pkg_maintainer="The Chef Habitat Maintainers <humans@habitat.sh>"
$pkg_description="A software library that implements a self-contained, serverless, zero-configuration, transactional SQL database engine."
$pkg_upstream_url="https://www.sqlite.org/"
$pkg_source="https://www.sqlite.org/2016/${pkg_name}-autoconf-${pkg_version}.tar.gz"
$pkg_filename=${pkg_name}-autoconf-${pkg_version}.tar.gz
$pkg_dirname=${pkg_name}-autoconf-${pkg_version}
$pkg_shasum="e2797026b3310c9d08bd472f6d430058c6dd139ff9d4e30289884ccd9744086b"
$pkg_deps=@("core/glibc", "core/readline")
$pkg_build_deps=@("core/gcc", "core/make", "core/coreutils")
$pkg_lib_dirs=@("lib")
$pkg_include_dirs=@("include")
$pkg_bin_dirs=@("bin")
```

It has the name of the software, the version, where to download it, a checksum to verify the contents are what we expect, run dependencies on `core/glibc` and `core/readline`, build dependencies on `core/coreutils`, `core/make`, `core/gcc`, libraries files in `lib`, header files in `include`, and a binary file in `bin`. Also, because it's a core plan, it has a description and upstream URL for the source project included.

## Buildtime Workflow

For buildtime installation and configuration, workflow steps need to be included in the plan file to define how you will install your application source files into a package. Before writing your plan, you should know and understand how your application binaries are currently built, installed, what their dependencies are, and where your application or software library expects to find those dependencies.

The main steps in the buildtime workflow are:

1. Create your fully-qualified package identifier.
2. Add licensing and contact information.
3. Download and unpack your source files.
4. Define your dependencies.
5. (Optional) Override any default build phases you need to using callbacks.

The following sections describe each of these steps in more detail.

### Create your Package Identifier

The origin is a place for you to set default privacy rules, store your packages, and collaborate with teammates. For example, the "core" origin is where the core maintainers of Chef Habitat share packages that are foundational to building other packages. If you would like to browse them, they are located in the [core-plans repo](https://github.com/habitat-sh/core-plans), and on [Chef Habitat Builder's Core Origin](https://bldr.habitat.sh/#/pkgs/core).

Creating artifacts for a specific origin requires that you have access to the that origin's private key. The private origin key will be used to sign the artifact when it is built by the `hab plan build` command. Origin keys are kept in `$HOME/.hab/cache/keys` on the host machine when running `hab` as a non-root user and `/hab/cache/keys` when running as root (including in the studio). For more information on origin keys, see [Keys](/docs/glossary/#glossary-keys).

The next important part of your package identifier is the name of the package. Standard naming convention is to base the name of the package off of the name of the source or project you download and install into the package.

### Add Licensing and Contact Information

You should enter your contact information in your plan.

Most importantly, you should update the `pkg_license` value to indicate the type of license (or licenses) that your source files are licensed under. Valid license types can be found at [https://spdx.org/licenses/](https://spdx.org/licenses/). You can include multiple licenses as an array.

> Note: Because all arrays in the pkg_* settings are shell arrays, they are whitespace delimited.

#### Download and Unpack Your Source Files

Add in the `pkg_source` value that points to where your source files are located at. Any `wget` url will work; however, unless you're downloading a tarball from a public endpoint, you may need to modify how you download your source files and where in your plan.sh you perform the download operation.

Chef Habitat supports retrieving source files from [GitHub](https://github.com). When cloning from GitHub, it is recommended to use https URIs because they are proxy friendly, whereas `git@github` or `git://` are not. To download the source from a GitHub repository, implement `do_download()` in your plan.sh (or `Invoke-Download` in a plan.ps1) and add a reference the `core/git` package as a build dependency. Because Chef Habitat does not contain a system-wide CA cert bundle, you must use the `core/cacerts` package and export the `GIT_SSL_CAINFO` environment variable to point the `core/cacerts` package on Linux. Here's an example of how to do this in the `do_download()` callback.

```bash
do_download() {
  export GIT_SSL_CAINFO="$(pkg_path_for core/cacerts)/ssl/certs/cacert.pem"
  git clone https://github.com/chef/chef
  pushd chef
  git checkout $pkg_version
  popd
  tar -cjvf $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}.tar.bz2 \
      --transform "s,^\./chef,chef-${pkg_version}," ./chef \
      --exclude chef/.git --exclude chef/spec
  pkg_shasum=$(trim $(sha256sum $HAB_CACHE_SRC_PATH/${pkg_filename} | cut -d " " -f 1))
}
```

The plan.ps1 equivalent would be:

```powershell
Function Invoke-Download {
  git clone https://github.com/chef/chef
  pushd chef
  git checkout $pkg_version
  popd
  Compress-Archive -Path chef/* -DestinationPath $HAB_CACHE_SRC_PATH/$pkg_name-$pkg_version.zip -Force
  $script:pkg_shasum = (Get-FileHash -path $HAB_CACHE_SRC_PATH/$pkg_name-$pkg_version.zip -Algorithm SHA256).Hash.ToLower()
}
```

After you have either specified your source in `pkg_source`, or overridden the **do_download()** or **Invoke-Download** callback, create a sha256 checksum for your source archive and enter it as the `pkg_shasum` value. The build script will verify this after it has downloaded the archive.

> Note: If your computed value does not match the value calculated by the `hab-plan-build` script, an error with the expected value will be returned when you execute your plan.

If your package does not download any application or service source files, then you will need to override the **do_download()**, **do_verify()**, and **do_unpack()** callbacks. See [Callbacks](/docs/reference#reference-callbacks) for more details.

#### Define Your Dependencies

Applications have two types of dependencies: buildtime and runtime.

Declare any build dependencies in `pkg_build_deps` and any run dependencies in `pkg_deps`. You can include version and release information when declaring dependencies if your application is bound to a particular version.

The package `core/glibc` is typically listed as a run dependency and `core/coreutils` as a build dependency, however, you should not take any inference from this. There are no standard dependencies that every package must have. For example, the mytutorialapp package only includes the `core/node` as a run dependency. You should include dependencies that would natively be part of the build or runtime dependencies your application or service would normally depend on.

There is a third type of dependencies, transitive dependencies, that are the run dependencies of either the build or run dependencies listed in your plan. You do not need to explicitly declare transitive dependencies, but they are included in the list of files when your package is built. See [Package contents](/docs/reference/#package-contents) for more information.

#### Override Build Phase Defaults with Callbacks

As shown in an example above, there are occasions when you want to override the default behavior of the hab-plan-build script. The Plan syntax guide lists the default implementations for [build phase callbacks](/docs/reference/#reference-callbacks), but if you need to reference specific packages in the process of building your applications or services, then you need to override the default implementations as in the example below.

```bash
pkg_name=httpd
pkg_origin=core
pkg_version=2.4.18
pkg_maintainer="The Chef Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apache')
pkg_source=http://www.apache.org/dist/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=1c39b55108223ba197cae2d0bb81c180e4db19e23d177fba5910785de1ac5527
pkg_deps=(core/glibc core/expat core/libiconv core/apr core/apr-util core/pcre core/zlib core/openssl)
pkg_build_deps=(core/patch core/make core/gcc)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_exports=(
  [port]=serverport
)
pkg_svc_run="httpd -DFOREGROUND -f $pkg_svc_config_path/httpd.conf"
pkg_svc_user="root"

do_build() {
  ./configure --prefix=$pkg_prefix \
              --with-expat=$(pkg_path_for expat) \
              --with-iconv=$(pkg_path_for libiconv) \
              --with-pcre=$(pkg_path_for pcre) \
              --with-apr=$(pkg_path_for apr) \
              --with-apr-util=$(pkg_path_for apr-util) \
              --with-z=$(pkg_path_for zlib) \
              --enable-ssl --with-ssl=$(pkg_path_for openssl) \
              --enable-modules=most --enable-mods-shared=most
  make
}
```

In this example, the `core/httpd` plan references several other core packages through the use of the `pkg_path_for` function before `make` is called. You can use a similar pattern if you need reference a binary or library when building your source files.

Or consider this override from a plan.ps1:

```powershell
function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release --verbose
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    }
    finally { Pop-Location }
}
```

Here the plan is building an application written in Rust. So it overrides `Invoke-Build` and uses the `cargo` utility included in its buildtime dependency on `core/rust`.

> Note: Powershell plan function names differ from their Bash counterparts in that they use the `Invoke` `verb` instead of the `do_` prefix.

When overriding any callbacks, you may use any of the variables, settings, or functions in the [Plan syntax guide](/docs/reference/), except for the runtime template data. Those can only be used in Application Lifecycle hooks once a Chef Habitat service is running.

### Runtime Workflow

Similar to defining the setup and installation experience at buildtime, behavior for your application or service needs to be defined for the Supervisor. This is done at runtime through Application lifecycle hooks. See [Application Lifecycle hooks](/docs/reference/reference-hooks) for more information and examples.

If you only need to start the application or service when the Chef Habitat service starts, you can instead use the `pkg_svc_run` setting and specify the command as a string. When your package is created, a basic run hook will be created by Chef Habitat.

You can use any of the [runtime configuration settings](/docs/reference/template-data), either defined by you in your config file, or defined by Chef Habitat.

Once you are done writing your plan, use the studio to [build your package](/docs/studio/studio-builds).

### Related Resources

- [Binary-only packages](/docs/best-practices/#binary-wrapper): Learn how to create packages from software that comes only in binary form, like off-the-shelf or legacy programs.

You may also find the [plan syntax guide](/docs/reference/) useful. It lists the settings, variables, and functions that you can use when creating your plan.
