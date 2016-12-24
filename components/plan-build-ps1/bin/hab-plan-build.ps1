param (
    [string]
    $Context = ".",

    [string]
    $DepotUrl
)

# # License and Copyright
# ```
# Copyright: Copyright (c) 2015 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ```
#
#

# ## Default variables

# The short version of the program name which is used in logging output
$program = $MyInvocation.MyCommand
# The current version of this program
$HAB_PLAN_BUILD = "@VERSION@"
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
if (Test-Path Env:\HAB_ROOT_PATH) {
    $script:HAB_ROOT_PATH = "$env:HAB_ROOT_PATH"
} else {
    $script:HAB_ROOT_PATH = "\hab"
}
# The default path where source artifacts are downloaded, extracted, & compiled
$HAB_CACHE_SRC_PATH = "${HAB_ROOT_PATH}\cache\src"
# The default download root path for package artifacts, used on package
# installation
$HAB_CACHE_ARTIFACT_PATH = "${HAB_ROOT_PATH}\cache\artifacts"
# The default path where cryptographic keys are stored. If the
# `$HAB_CACHE_KEY_PATH` environment variable is set, this value is overridden,
# otherwise it is set to its default.
if (Test-Path Env:\HAB_CACHE_KEY_PATH) {
    $script:HAB_CACHE_KEY_PATH = "$env:HAB_CACHE_KEY_PATH"
} else {
    $script:HAB_CACHE_KEY_PATH = "${HAB_ROOT_PATH}\cache\keys"
}
# Export the key path for other programs and subshells to use
$env:HAB_CACHE_KEY_PATH = "$script:HAB_CACHE_KEY_PATH"
# The root path containing all locally installed packages
$script:HAB_PKG_PATH = "${HAB_ROOT_PATH}\pkgs"
# The first argument to the script is a Plan context directory, containing a
# `plan.sh` file
$script:PLAN_CONTEXT = "$Context"
# The default Habitat Depot from where to download dependencies. If the URL was
# provided as an option use that, if not use any set `HAB_DEPOT_URL`
# environment variable, and otherwise use the default provided.
if ($DepotUrl) {
    $script:HAB_DEPOT_URL = "$DepotUrl"
} elseif (Test-Path Env:\HAB_DEPOT_URL) {
    $script:HAB_DEPOT_URL = "$env:HAB_DEPOT_URL"
} else {
    $script:HAB_DEPOT_URL = "https://willem.habitat.sh/v1/depot"
}
# Export the Depot URL so all other programs and subshells use this same one
$env:HAB_DEPOT_URL = "$script:HAB_DEPOT_URL"
# The value of `$env:Path` on initial start of this program
$script:INITIAL_PATH = "$env:Path"
# The value of `pwd` on initial start of this program
$script:INITIAL_PWD = "$(Get-Location)"
# The target architecture this plan will be built for
$script:pkg_arch = "x86_64"
# The target system (i.e. operating system variant) this plan will be built for
$script:pkg_sys = "windows"
# The full target tuple this plan will be built for
$script:pkg_target = "${pkg_arch}-${pkg_sys}"
# The package's origin (i.e. acme)
$script:pkg_origin = ""
# The package's name (i.e. myapp)
$script:pkg_name = ""
# The package's version (i.e. 1.2.3)
$script:pkg_version = ""
# Each release is a timestamp - `YYYYMMDDhhmmss`
$script:pkg_release = "$(Get-Date -UFormat "%Y%m%d%H%M%S")"
# The default build deps setting - an empty array
$script:pkg_build_deps = @()
# The default runtime deps setting - an empty array
$script:pkg_deps = @()
# The path inside a package that contains libraries - used in `LD_RUN_PATH` and
# `LD_FLAGS`.
$script:pkg_lib_dirs = @()
# The path inside a package that contains binary programs - used in `PATH`
$script:pkg_bin_dirs = @()
# The path inside a package that contains header files - used in `CFLAGS`
$script:pkg_include_dirs = @()
# The path(s) inside a package that contain pkg-config (.pc) files
$script:pkg_pconfig_dirs = @()
# The command to run the service - must not fork or return
$script:pkg_svc_run = ''
# An array of ports to expose.
$script:pkg_expose = @()
# The user to run the service as
$script:pkg_svc_user = "hab"
# The group to run the service as
$script:pkg_svc_group = "$pkg_svc_user"

# Initially set $pkg_svc_* variables. This happens before the Plan is sourced,
# meaning that `$pkg_name` is not yet set. However, `$pkg_svc_run` wants
# to use these variables, so what to do? We'll set up these svc variables
# with the `$pkg_svc_run` variable as the customer-in-mind and pass over
# it once the Plan has been loaded. For good meaure, all of these variables
# will need to be set again.
$script:pkg_svc_path="$HAB_ROOT_PATH\svc\@__pkg_name__@"
$script:pkg_svc_data_path="$pkg_svc_path\data"
$script:pkg_svc_files_path="$pkg_svc_path\files"
$script:pkg_svc_var_path="$pkg_svc_path\var"
$script:pkg_svc_config_path="$pkg_svc_path\config"
$script:pkg_svc_static_path="$pkg_svc_path\static"


# ## Private/Internal helper functions
#
# These functions are part of the private/internal API of this program and
# should **not** be used externally by Plan authors or other programs. Their
# behaviors and names can change with little to no warning and no direct
# support can be provided as a result. Thank you for your
# understanding--maintaining a tiny but robust public interface is not an easy
# task.

<#
function _On-Exit {

}

trap { _On-Exit }
#>

function _Assert-OriginKeyPresent {
    $cache = "$HAB_CACHE_KEY_PATH"
    if (-Not (Test-Path "$cache")) {
        _Exit-With "Cache key path $cache does not exist, aborting" 35
    }

    $candidate_keys = (Get-ChildItem "$cache" `
        -Recurse -Force -Include "${pkg_origin}-*.sig.key")
    if ($candidate_keys.Length -eq 0) {
        _Exit-With "Signing origin key '$pkg_origin' not found in $cache, aborting" 35
    }
    Write-Debug "At least one signing key for $pkg_origin found in $cache"
}

function _Check-Command {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$True)]
        [string]
        $CmdletName
    )

    process {
        return [bool](Get-Command -Name $CmdletName -ErrorAction SilentlyContinue)
    }
}

function _Exit-With {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory=$True)]
        [string]
        $Message,

        [Parameter(Mandatory=$True)]
        [int32]
        $ExitCode
    )

    process {
        Write-Error "$Message"
        exit $ExitCode
    }
}

# **Internal** Ensures that the correct versions of key system commands are
# able to be used by this program. If we cannot find suitable versions, we will
# abort early.
#
# The following variables are set which contain an absolute path to the desired
# command:
#
# * `$_hab_cmd` (hab cli for signing, hashing, and possibly installing)
# * `$_7z_cmd` (7z program for creating tarballs and compressing with xz)
#
# Note that all of the commands noted above are considered internal
# implementation details and are subject to change with little to no notice,
# which means the variables such as `$_hab_cmd` should **not** be used directly
# by Plan authors. The leading underscore denotes an internal/private variable
# or function.
#
# If the commands are not found, `exit_with` is called and the program is
# terminated.
function _Get-SystemCommands {
    if (_Check-Command hab) {
        $script:_hab_cmd = (Get-Command hab).Source
    } else {
        _Exit-With "We require hab to sign artifacts; aborting" 1
    }
    Write-Debug "Setting _hab_cmd=$_hab_cmd"

    if (_Check-Command 7z) {
        $script:_7z_cmd = (Get-Command 7z).Source
    } else {
        _Exit-With "We require 7z to create tar files & compress; aborting" 1
    }
    Write-Debug "Setting _7z_cmd=$_7z_cmd"
}




function Write-BuildLine {
    <#
    .SYNOPSIS
    Print a line of build output
    .DESCRIPTION
    Takes a string as its only argument.
    #>
    [CmdletBinding()]
    param(
        # A message to display
        [string]
        $Message
    )

    process {
        Write-Host "   ${pkg_name}: " -ForegroundColor Cyan -NoNewline
        Write-Host "$Message" -ForegroundColor White
    }
}

# ## Build Phases
#
# Stub build phases, in the order they are executed. These can be overridden by
# the `plan.ps1` if needed.


# Used to execute arbitrary commands before anything else happens. Delegates
# most implementation to the `Invoke-DefaultBegin` function.
function Invoke-Begin {
  Invoke-DefaultBegin
}

# Default implementation for the `Invoke-Begin` phase.
function Invoke-DefaultBegin {
}

function _Set-HabBin {
}

function _Complete-DependencyResolution {
}

function _Set-Path {
}

# Download the software from `$pkg_source` and place it in
# `$HAB_CACHE_SRC_PATH\${$pkg_filename}`. If the source already exists in the
# cache, verify that the checksum is what we expect, and skip the download.
# Delegates most of the implementation to the `Invoke-DefaultDownload` function.
function Invoke-Download {
    Invoke-DefaultDownload
}

# Default implementation for the `Invoke-Download` phase.
function Invoke-DefaultDownload {
}

# Verify that the package we have in `$HAB_CACHE_SRC_PATH\$pkg_filename` has
# the `$pkg_shasum` we expect. Delegates most of the implementation to the
# `Invoke-DefaultVerify` function.
function Invoke-Verify {
    Invoke-DefaultVerify
}

# Default implementation for the `Invoke-Verify` phase.
function Invoke-DefaultVerify {
}

# Clean up the remnants of any previous build job, ensuring it can't pollute
# out new output. Delegates most of the implementation to the
# `Invoke-DefaultClean` function.
function Invoke-Clean {
    Invoke-DefaultClean
}

# Default implementation for the `Invoke-Clean` phase.
function Invoke-DefaultClean {
    Write-BuildLine "Clean the cache"
    $src = "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    if (Test-Path "$src") {
        Remove-Item "$src" -Recurse -Force
    }
}

# Takes the `$HAB_CACHE_SRC_PATH\$pkg_filename` from the download step, and
# unpacks it, as long as the method of extraction can be determined.
#
# This takes place in the `$HAB_CACHE_SRC_PATH` directory.
#
# Delegates most of the implementation to the `Invoke-DefaultUnpack` function.
function Invoke-Unpack {
    Invoke-DefaultUnpack
}

# Default implementation for the `Invoke-Unpack` phase.
function Invoke-DefaultUnpack {
}

function _Set-Environment {
    # Create a working directory if it doesn't already exist from `Invoke-Unpack`
    New-Item "$HAB_CACHE_SRC_PATH\$pkg_dirname" -ItemType Directory -Force |
        Out-Null
}

# This function simply makes sure that the working directory for the prepare
# step is correct, that is inside the extracted source directory.
function Invoke-PrepareWrapper {
    Write-BuildLine "Preparing to build"
    Push-Location "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    Invoke-Prepare
    Pop-Location
}

# A step that exists to be overriden. We have the software downloaded,
# unpacked, and the build environment variables set. Do what you need to do
# before we actually run the build steps. Delegates most of the implementation
# to the `Invoke-DefaultPrepare` function.
function Invoke-Prepare {
    Invoke-DefaultPrepare
}

# Default implementation of the `Invoke-Prepare` phase.
function Invoke-DefaultPrepare {
}

# Since `build` is one of the most overriden functions, this wrapper makes sure
# that no matter how it is changed, our working directory is
# `$HAB_CACHE_SRC_PATH\$pkg_dirname`.
function Invoke-BuildWrapper {
    Write-BuildLine "Building"
    Push-Location "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    Invoke-Build
    Pop-Location
}

# Build the software. Delegates most of the implementation to the
# `Invoke-DefaultBuild` function.
function Invoke-Build {
    Invoke-DefaultBuild
}

# Default implementation for the `Invoke-Build` phase.
function Invoke-DefaultBuild {
}

# Will run post-compile tests and checks, provided 2 conditions are true:
#
# 1. An `Invoke-Check` function has been declared. By default, no such function
#    exists, so Plan author must add one explicitly--there is no reasonably
#    good deault here.
# 1. A `$env:DO_CHECK` environment variable is set to some non-empty value. As
#    tests can dramatically inflate the build time of a Plan, this has been
#    left as an opt-in option.
#
# Here's an example example of a vanilla Plan such as Sed:
#
# ```ps1
# $pkg_name = "sed""
# # other Plan metadata...
#
# function Invoke-Check {
#   make check
# }
# ```
function Invoke-CheckWrapper {
    if ((_Check-Command Invoke-Check) -and (Test-Path Env:\DO_CHECK)) {
        Write-BuildLine "Running post-compile tests"
        Push-Location "$HAB_CACHE_SRC_PATH\$pkg_dirname"
        Invoke-Check
        Pop-Location
    }
}

# Identical to the `Invoke-BuildWrapper` function above; simply makes sure the
# working directory for the install_files step is correct.
function Invoke-InstallWrapper {
    Write-BuildLine "Installing"
    New-Item "$pkg_prefix" -ItemType Directory -Force | Out-Null
    Push-Location "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    Invoke-Install
    Pop-Location
}

# Install the software. Delegates most of the implementation to the
# `Invoke-DefaultInstall` function.
function Invoke-Install {
    Invoke-DefaultInstall
}

# Default implementation for the `Invoke-Install` phase.
function Invoke-DefaultInstall {
}

# Copy the `./config` directory, relative to the Plan, to `$pkg_prefix\config`.
# Do the same with `default.toml`. Delegates most of the implementation to the
# `Invoke-DefaultBuildConfig` function.
function Invoke-BuildConfig {
    Invoke-DefaultBuildConfig
}

# Default implementation for the `Invoke-BuildConfig` phase.
function Invoke-DefaultBuildConfig {
    Write-BuildLine "Writing configuration"
    if(test-path "$PLAN_CONTEXT/config") {
        if (!$HAB_CONFIG_EXCLUDE) {
          # HAB_CONFIG_EXCLUDE not set, use defaults
          $config_exclude_exts=@("*.sw?", "*~", "*.bak")
        }
        else {
          $config_exclude_exts = $HAB_CONFIG_EXCLUDE -split " "
        }
        Get-ChildItem "$PLAN_CONTEXT/config" -Exclude $config_exclude_exts | foreach {
          if (!(Test-Path "$pkg_prefix/config" )) {
            mkdir "$pkg_prefix/config"
          }
          if($_.PSIsContainer) {
            mkdir (Join-Path $pkg_prefix $_.FullName.Substring($PLAN_CONTEXT.Length))
          }
          else {
            cp $_ (Join-Path $pkg_prefix $_.FullName.Substring($PLAN_CONTEXT.Length))
          }
        }
    }

    if (Test-Path "$PLAN_CONTEXT/hooks") {
        cp "$PLAN_CONTEXT/hooks" $pkg_prefix -Recurse
    }
    if (Test-Path "$PLAN_CONTEXT/default.toml") {
        cp "$PLAN_CONTEXT/default.toml" $pkg_prefix
    }
}

# Write out the `$pkg_prefix\run` file. If a file named `hooks\run`
# exists, we skip this step. Otherwise, we look for `$pkg_svc_run`,
# and use that. We assume that the binary used in the `$pkg_svc_run`
# command is set in the $env:Path.
#
# This will write a `run` script...
# @TODO fin - what does the run script do by default? is it a powershell script?
#
# Delegates most of the implementation to the `Invoke-DefaultBuildService`
# function.
function Invoke-BuildService {
    Invoke-DefaultBuildService
}

# Default implementation of the `Invoke-BuildService` phase.
function Invoke-DefaultBuildService {
    Write-BuildLine "Writing service management scripts"
    if (Test-Path "${PLAN_CONTEXT}/hooks/run") {
        Write-BuildLine "Using run hook $PLAN_CONTEXT/hooks/run"
    }
    else {
        if (Test-Path $pkg_svc_run) {
          Write-BuildLine "Writing $pkg_prefix/run script to run $pkg_svc_run"
          Set-Content -Path "$pkg_prefix/run" -Value @"
cd "$pkg_svc_path"

& "$pkg_svc_run" 2>&1
exit `$LASTEXITCODE
"@
        }
    }
}

# **Internal** Write the `$pkg_prefix\MANIFEST`.
function _Write-Manifest {
    Write-BuildLine "Creating manifest"
    if ([string]::IsNullOrEmpty($pkg_upstream_url)) {
        $upstream_url_string = "upstream project's website or home page is not defined"
    } else {
        $upstream_url_string = "[$pkg_upstream_url]($pkg_upstream_url)"
    }

    if ([string]::IsNullOrEmpty($pkg_shasum)) {
        $shasum_string = "SHA256 checksum not provided or required"
    } else {
        $shasum_string = "``$pkg_shasum``"
    }

    if ($pkg_build_deps.Length -eq 0) {
        $build_deps_string = "no build dependencies or undefined"
    } else {
        $build_deps_string = $pkg_build_deps -join ' '
    }

    if ($pkg_deps.Length -eq 0) {
        $deps_string = "no runtime dependencies or undefined"
    } else {
        $deps_string = $pkg_deps -join ' '
    }

    if ($pkg_interpreters.Length -eq 0) {
        $interpreters_string = "no interpreters or undefined"
    } else {
        $interpreters_string = $pkg_interpreters -join ' '
    }

    "# $pkg_origin / $pkg_name
$pkg_description

* __Maintainer__: $pkg_maintainer
* __Version__: $pkg_version
* __Release__: $pkg_release
* __Architecture__: $pkg_arch
* __System__: $pkg_sys
* __Target__: $pkg_target
* __Upstream URL__: $upstream_url_string
* __License__: $($pkg_license -join ' ')
* __Source__: [$pkg_source]($pkg_source)
* __SHA__: $shasum_string
* __Path__: ``$pkg_prefix``
* __Build Dependencies__: $build_deps_string
* __Dependencies__: $deps_string
* __Interpreters__: $interpreters_string

# Plan

## Plan Source

``````ps1
$(Get-Content "$PLAN_CONTEXT\plan.ps1" -Raw)
``````
" | Out-File "$pkg_prefix\MANIFEST" -Encoding ascii
}

# **Internal** Write out the package data to files:
#
# * `$pkg_prefix/BUILD_DEPS` - Any dependencies we need build the package
# * `$pkg_prefix/CFLAGS` - Any CFLAGS for things that link against us
# * `$pkg_prefix/PKG_CONFIG_PATH` - Any PKG_CONFIG_PATH entries for things that depend on us
# * `$pkg_prefix/DEPS` - Any dependencies we need to use the package at runtime
# * `$pkg_prefix/EXPOSES` - Any ports we expose
# * `$pkg_prefix/FILES` - blake2b checksums of all files in the package
# * `$pkg_prefix/LDFLAGS` - Any LDFLAGS for things that link against us
# * `$pkg_prefix/LD_RUN_PATH` - The LD_RUN_PATH for things that link against us
# * `$pkg_prefix/PATH` - Any PATH entries for things that link against us
function _Write-Metadata {
    Write-BuildLine "Building pacakge metadata"

    # @TODO fin - LD_RUN_PATH
    # @TODO fin - LDFLAGS
    # @TODO fin - CFLAGS
    # @TODO fin - CPPFLAGS
    # @TODO fin - CXXFLAGS
    # @TODO fin - PKG_CONFIG_PATH

    if ($pkg_bin_dirs.Length -gt 0) {
        $($pkg_bin_dirs | % { "${pkg_prefix}\$_" }) -join ':' |
            Out-File "$pkg_prefix\PATH" -Encoding ascii
    }

    if ($pkg_expose.Length -gt 0) {
        "$($pkg_expose -join ' ')" |
            Out-File "$pkg_prefix\EXPOSES" -Encoding ascii
    }

   # @TODO fin - INTERPRETERS
   # @TODO fin - BUILD_DEPS
   # @TODO fin - BUILD_TDEPS
   # @TODO fin - DEPS
   # @TODO fin - TDEPS

   "$pkg_target" | Out-File "$pkg_prefix\TARGET" -Encoding ascii

   "$pkg_origin/$pkg_name/$pkg_version/$pkg_release" |
        Out-File "$pkg_prefix\IDENT" -Encoding ascii

    if (-Not ([string]::IsNullOrEmpty($pkg_svc_user))) {
        "$pkg_svc_user" |
            Out-File "$pkg_prefix\SVC_USER" -Encoding ascii
    }
    if (-Not ([string]::IsNullOrEmpty($pkg_svc_group))) {
        "$pkg_svc_group" |
            Out-File "$pkg_prefix\SVC_GROUP" -Encoding ascii
    }

    # Generate the blake2b hashes of all the files in the package. This
    # is not in the resulting MANIFEST because MANIFEST is included!
    # @TODO fin - FILES
}

# **Internal** Create the package artifact with `tar`/`hab pkg sign`
function _Save-Artifact {
    Write-BuildLine "Generating package artifact"
    $dir = Split-Path $pkg_artifact -Parent
    $file = ".$([io.path]::GetFileNameWithoutExtension($pkg_artifact))"
    $tarf = "$dir\${file}.tar"
    $xzf = "${tarf}.xz"

    New-Item "$dir" -ItemType Directory -Force | Out-Null
    Remove-Item "$tarf" -Force -ErrorAction SilentlyContinue
    Remove-Item "$xzf" -Force -ErrorAction SilentlyContinue

    & "$_7z_cmd" a -ttar -spf "$tarf" "$pkg_prefix"
    & "$_7z_cmd" a -txz "$xzf" "$tarf"
    & "$_hab_cmd" pkg sign --origin "$pkg_origin" "$xzf" "$pkg_artifact"
    Remove-Item "$tarf", "$xzf" -Force

    <#
        ConvertFrom-StringData -StringData (gc -raw .\results\last_build.env)
        $t = ConvertFrom-StringData -StringData (gc -raw .\results\last_build.env)

        get-content -Encoding Byte .\results\fnichol-names-0.11.0-20161128001352-x86_64-windows.hart |
            Select-Object -Skip 5 |
            Set-Content -Encoding Byte -Path .\results\test.tar.xz
        7z x -txz .\results\test.tar.xz .\results\test.tar
        7z x -ttar .\results\test.tar .\results\hab
    #>
}

# **Internal** Copy the final package artifact to the `$pkg_output_path`
# directory as well as prepare a `last_build.env` report.
function _Copy-BuildOutputs {
    New-Item "$pkg_output_path" -ItemType Directory -Force | Out-Null
    Copy-Item "$pkg_artifact" "$pkg_output_path"

    $_pkg_sha256sum = (Get-FileHash "$pkg_artifact" -Algorithm SHA256).Hash.ToLower()
    $_pkg_blake2bsum = $(& "$_hab_cmd" pkg hash "$pkg_artifact")

    @"
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
pkg_artifact=$(Split-Path $pkg_artifact -Leaf)
pkg_sha256sum=$_pkg_sha256sum
pkg_blake2bsum=$_pkg_blake2bsum
"@ | Out-File "$pkg_output_path\last_build.env" -Encoding ascii
}

# A function for cleaning up after yourself. Delegates most of the
# implementation to the `Invoke-DefaultEnd` function.
function Invoke-End {
  Invoke-DefaultEnd
}

# Default implementation for the `Invoke-End` phase.
function Invoke-DefaultEnd {
}


# # Main Flow

# Expand the context path to an absolute path
if (-Not (Test-Path "$Context")) {
    _Exit-With "Context must be an existing directory" 10
}
$script:PLAN_CONTEXT = (Get-Item $Context).FullName

# Now to ensure a `plan.ps1` exists where we expect it. There are 2 possible
# candidate locations relative to the `$PLAN_CONTEXT` directory: a `./plan.ps1`
# or a `./habitat/plan.ps1`. Only one or the other location is allowed so that
# a Plan author isn't confused if they edit one to have this program read
# the other.

# We'll make sure that both are not present, and if so abort.
if ((Test-Path "$PLAN_CONTEXT\plan.ps1") -and (Test-Path "$PLAN_CONTEXT\habitat\plan.ps1")) {
    $places = "$PLAN_CONTEXT\plan.sh and $PLAN_CONTEXT\habitat/plan.sh"
    _Exit-With "A Plan file was found at $places. Only one is allowed at a time" 42
}
# We check if the provided path has a `plan.sh` in it in either location. If
# not, we'll quickly bail.
if (-Not (Test-Path "$PLAN_CONTEXT\plan.ps1")) {
    if (Test-Path "$PLAN_CONTEXT\habitat\plan.ps1") {
        $PLAN_CONTEXT = "$PLAN_CONTEXT\habitat"
    } else {
        $places = "$PLAN_CONTEXT\plan.sh or $PLAN_CONTEXT\habitat/plan.sh"
        _Exit-With "Plan file not found at $places" 42
    }
}

# Change into the `$PLAN_CONTEXT` directory for proper resolution of relative
# paths in `plan.ps1`
Set-Location "$PLAN_CONTEXT"
# @TODO fin - is there a way to change directory only in this program without
# affecting the calling powershell session?

# Load the Plan
Write-BuildLine "Loading $PLAN_CONTEXT\plan.ps1"
. "$PLAN_CONTEXT\plan.ps1"
Write-BuildLine "Plan loaded"
# @TODO fin - what to do when dot souring fails? can it?

# If the `HAB_ORIGIN` environment variable is set, override the value of
# `$pkg_origin`.
if (Test-Path Env:\HAB_ORIGIN) {
    $script:pkg_origin = "$env:HAB_ORIGIN"
}

# Validate metadata
Write-BuildLine "Validating plan metadata"

foreach ($var in @("pkg_origin", "pkg_name", "pkg_version", "pkg_source")) {
    if (-Not (Test-Path variable:script:$var)) {
        _Exit-With "Failed to build. '$var' must be set." 1
    } elseif ((Get-Variable $var -Scope script).Value -eq "") {
        _Exit-With "Failed to build. '$var' must be set and non-empty." 1
    }
}

# Test to ensure package name contains only valid characters
if (-Not ("$pkg_name" -match '^[A-Za-z0-9_-]+$')) {
    _Exit-With "Failed to build. Package name '$pkg_name' contains invalid characters." 1
}

# Pass over `$pkg_svc_run` to replace any `$pkg_name` placeholder tokens
# from prior pkg_svc_* variables that were set before the Plan was loaded.
if ("$pkg_svc_run" -ne "") {
    $pkg_svc_run = "$pkg_svc_run".Replace("@__pkg_name__@", "$pkg_name")
}

# Set `$pkg_filename` to the basename of `$pkg_source`, if it is not already
# set by the `plan.ps1`.
if ("$pkg_filename" -eq "") {
    $script:pkg_filename = "$(Split-Path $pkg_source -Leaf)"
}

# Set `$pkg_dirname` to the `$pkg_name` and `$pkg_version`, if it is not
# already set by the `plan.ps1`.
if ("$pkg_dirname" -eq "") {
    $script:pkg_dirname = "${pkg_name}-${pkg_version}"
}

# Set `$pkg_prefix` if not already set by the `plan.ps1`.
if ("$pkg_prefix" -eq "") {
    $script:pkg_prefix = "$HAB_PKG_PATH\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"
}

# Determine the final output path for the package artifact
$script:pkg_output_path = "$INITIAL_PWD\results"

# Set $pkg_svc variables a second time, now that the Plan has been sourced and
# we have access to `$pkg_name`.
$script:pkg_svc_path="$HAB_ROOT_PATH\svc\$pkg_name"
$script:pkg_svc_data_path="$pkg_svc_path\data"
$script:pkg_svc_files_path="$pkg_svc_path\files"
$script:pkg_svc_var_path="$pkg_svc_path\var"
$script:pkg_svc_config_path="$pkg_svc_path\config"
$script:pkg_svc_static_path="$pkg_svc_path\static"

# Set the package artifact name
$_artifact_ext="hart"
$script:pkg_artifact="$HAB_CACHE_ARTIFACT_PATH\${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"

# Run `do_begin`
Write-BuildLine "$program setup"
Invoke-Begin

# Determine if we have all the commands we need to work
_Get-SystemCommands

# Enure that the origin key is available for package signing
_Assert-OriginKeyPresent

_Set-HabBin

# Download and resolve the depdencies
_Complete-DependencyResolution

# Set the complete `Path` environment.
_Set-Path

# Download the source
New-Item "$HAB_CACHE_SRC_PATH" -ItemType Directory -Force | Out-Null
Invoke-Download

# Verify the source
Invoke-Verify

# Clean the cache
Invoke-Clean

# Unpack the source
Invoke-Unpack

# Set up the build environment
_Set-Environment

# Prepare the source
Invoke-PrepareWrapper

# Build the source
Invoke-BuildWrapper

# Check the source
Invoke-CheckWrapper

# Install the source
Invoke-InstallWrapper

# Copy the configuration
Invoke-BuildConfig

# Copy the service management scripts
Invoke-BuildService

# Write the manifest
_Write-Manifest

# Render the linking and dependency files
_Write-Metadata

# Generate the artifact and write to artifact cache
_Save-Artifact

# Copy produced artifact to a local relative directory
_Copy-BuildOutputs

# Cleanup
Write-BuildLine "$program cleanup"
Invoke-End

# Print the results
Write-BuildLine
Write-BuildLine "Source Cache: $HAB_CACHE_SRC_PATH\$pkg_dirname"
Write-BuildLine "Installed Path: $pkg_prefix"
Write-BuildLine "Artifact: $pkg_output_path\$(Split-Path $pkg_artifact -Leaf)"
Write-BuildLine "Build Report: $pkg_output_path\last_build.env"
Write-BuildLine "SHA256 Checksum: $_pkg_sha256sum"
Write-BuildLine "Blake2b Checksum: $_pkg_blake2bsum"

# Exit cleanly
Write-BuildLine
Write-BuildLine "I love it when a plan.ps1 comes together."
Write-BuildLine

# Return to the original directory
Set-Location "$INITIAL_PWD"