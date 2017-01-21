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

function Get-HabPackagePath {
<#
.SYNOPSIS
Returns the path for the desired build or runtime direct package dependency
on stdout from the resolved dependency set.

.PARAMETER Identity
The package identity of the path to retrieve.

.EXAMPLE
Get-HabPackagePath "acme/nginx"
# /hab/pkgs/acme/nginx/1.8.0/20150911120000

.EXAMPLE
Get-HabPackagePath "zlib"
# /hab/pkgs/acme/zlib/1.2.8/20151216221001

.EXAMPLE
Get-HabPackagePath "glibc/2.22"
# /hab/pkgs/acme/glibc/2.22/20151216221001
#>
    param($Identity)

    foreach($e in $pkg_all_deps_resolved) {
        if((Resolve-HabPkgPath $e).Contains("/$Identity/")) {
          return $e
        }
    }
    Write-Error "Get-HabPackagePath '$Identity' did not find a suitable installed package`nResolved package set: ${pkg_all_deps_resolved}"
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
  if ($env:NO_INSTALL_DEPS) {
    Write-BuildLine "`$env:NO_INSTALL_DEPS set: no package dependencies will be installed"
  }

  if ($env:HAB_BIN) { $script:HAB_BIN=$env:HAB_BIN }
  else { $script:HAB_BIN=$_hab_cmd }
  Write-BuildLine "Using HAB_BIN=$HAB_BIN for installs, signing, and hashing"
}

function _install-dependency($dependency) {
  if (!$env:NO_INSTALL_DEPS) {
    & $HAB_BIN install -u $env:HAB_DEPOT_URL $dependency
  }
}

# **Internal** Return the path to the latest release of a package on stdout.
#
# ```
# _latest_installed_package acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _latest_installed_package acme/nginx/1.8.0
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _latest_installed_package acme/nginx/1.8.0/20150911120000
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# ```
#
# Will return the package found on disk, and $false if a package cannot be found.
function _latest_installed_package($dependency) {
  if (!(Test-Path "$HAB_PKG_PATH/$dependency")) {
    Write-Warning "No installed packages of '$dependency' were found"
    return $false
  }

  # Count the number of slashes, and use it to make a choice
  # about what to return as the latest package.
  $latest_package_flags = $dependency.split("/").length - 1
  $depth = switch ($latest_package_flags) {
    3 { 1 }
    2 { 2 }
    1 { 3 }
  }

  $result = try { (Get-ChildItem "$HAB_PKG_PATH/$dependency" -Recurse -Include MANIFEST)[-1].FullName } catch { }
  if (!$result) {
    Write-Warning "Could not find a suitable installed package for '$dependency'"
    return $false
  }
  else {
    return Split-Path $result -Parent
  }
}

function _resolve-dependency($dependency) {
  if (!$dependency.Contains("/")) {
    Write-Warning "Origin required for '$dependency' in plan '$pkg_origin/$pkg_name' (example: acme/$dependency)"
    return $false
  }

  if ($dep_path = _latest_installed_package $dependency) {
    return $dep_path
  }
  else {
    return $false
  }
}

# **Internal** Returns (on stdout) the `TDEPS` file contents of another locally
# installed package which contain the set of all direct and transitive run
# dependencies. An empty set could be returned as whitespace and/or newlines.
# The lack of a `TDEPS` file in the desired package will be considered an
# unset, or empty set.
#
# ```
# _Get-TdepsFor /hab/pkgs/acme/a/4.2.2/20160113044458
# # /hab/pkgs/acme/dep-b/1.2.3/20160113033619
# # /hab/pkgs/acme/dep-c/5.0.1/20160113033507
# # /hab/pkgs/acme/dep-d/2.0.0/20160113033539
# # /hab/pkgs/acme/dep-e/10.0.1/20160113033453
# # /hab/pkgs/acme/dep-f/4.2.2/20160113033338
# # /hab/pkgs/acme/dep-g/4.2.2/20160113033319
# ```
#
# Will return the contents of `TDEPS` if the file exists, otherwise an empty array.
function _Get-TdepsFor($dependency) {
  if (Test-Path "$dependency/TDEPS") {
    Get-Content $dependency/TDEPS
  }
  else {
    # No file, meaning an empty set
    @()
  }
}

# **Internal** Appends an entry to the given array only if the entry is not
# already present and returns the resulting array back on stdout. In so doing,
# this function mimics a set when adding new entries. Note that any array can
# be passed in, including ones that already contain duplicate entries.
#
# ```
# arr=(a b c)
# arr=($(_return_or_append_to_set "b" "${arr[@]}"))
# echo ${arr[@]}
# # a b c
# arr=($(_return_or_append_to_set "z" "${arr[@]}"))
# echo ${arr[@]}
# # a b c z
# ```
#
# Will return 0 in any case.
function _return_or_append_to_set($dependency, $depArray) {
  foreach($e in $depArray) {
    if ($e -eq $dependency) {
      return $depArray
    }
  }
  $depArray + $dependency
}

function Resolve-HabPkgPath($unresolved) {
    $unresolved.Replace("$(Resolve-Path $HAB_PKG_PATH)\", "").Replace("\", "/")
}

# **Internal** Prints a dependency graph in a format to the `tree(1)` command.
# This is used in concert with `_validate_deps` for the purpose of output to an
# end user.  It accepts a standard in stream as input where each line is a
# direct dependency package identifier of some package. The first function
# parameter is the leading padding depth when printing the dependency line.
# Finally, a global internal variable, `$_dupes_qualified`, is used to display
# which dependency entries have the duplicate versions present. An example
# should help to clarify:
#
# ```
# _dupes_qualified=$(cat <<EOF
# acme/glibc/2.22/20160309153915
# acme/glibc/2.22/20160308150809
# acme/linux-headers/4.3/20160309153535
# acme/linux-headers/4.3/20160308150438
# EOF
# )
#
# echo "acme/less/481/20160309165238"
#
# cat <<EOF | _print_recursive_deps 1
# acme/glibc/2.22/20160309153915
# acme/ncurses/6.0/20160308165339
# acme/pcre/8.38/20160308165506
# EOF
# ```
#
# And the corresponding output, in this case showing the problematic
# dependencies:
#
# ```
# acme/less/481/20160309165238
#     acme/glibc/2.22/20160309153915 (*)
#         acme/linux-headers/4.3/20160309153535 (*)
#     acme/ncurses/6.0/20160308165339
#         acme/glibc/2.22/20160308150809 (*)
#             acme/linux-headers/4.3/20160308150438 (*)
#         acme/gcc-libs/5.2.0/20160308165030
#             acme/glibc/2.22/20160308150809 (*)
#                 acme/linux-headers/4.3/20160308150438 (*)
#     acme/pcre/8.38/20160308165506
#         acme/glibc/2.22/20160308150809 (*)
#             acme/linux-headers/4.3/20160308150438 (*)
#         acme/gcc-libs/5.2.0/20160308165030
#             acme/glibc/2.22/20160308150809 (*)
#                 acme/linux-headers/4.3/20160308150438 (*)
# ```
function _print_recursive_deps($dependencies, $qualified, $level) {
  # Compute the amount of leading whitespace when display this line and any
  # child dependencies.
  $padn=" " * ($level * 4)
  foreach($dep in $dependencies) {
    $dep = Resolve-HabPkgPath $dep
    # If this dependency is a member of the duplicated set, then add an
    # asterisk at the end of the line, otherwise print the dependency.
    if ($qualified.Contains($dep)) {
      Write-Host "$padn$dep (*)"
    }
    else {
      Write-Host "$padn$dep"
    }
    # If this dependency itself has direct dependencies, then recursively print
    # them.
    if (Test-Path "$HAB_PKG_PATH/$dep/DEPS") {
      _print_recursive_deps (Get-Content "$HAB_PKG_PATH/$dep/DEPS") $qualified ($level + 1)
    }
  }
}

# **Internal** Validates that the computed dependencies are reasonable and that
# the full runtime set is unique--that is, there are no duplicate entries of
# the same `ORIGIN/NAME` tokens. An example would be a Plan which has a
# dependency on `acme/glibc` and a dependency on `acme/pcre` which uses an
# older version of `acme/glibc`. This leads to a package which would have 2
# version of `acme/glibc` in the shared library `RUNPATH` (`RPATH`). Rather
# than building a package which is destined to fail at runtime, this function
# will fast-fail with dependency information which an end user can use to
# resolve the situation before continuing.
function _Assert-Deps {
  # Build the list of full runtime deps (one per line) without the
  # `$HAB_PKG_PATH` prefix.
  $tdeps = $pkg_tdeps_resolved | % {
    $_.Substring((Resolve-Path $HAB_PKG_PATH).Path.Length+1).Replace("\", "/")
  }

  $pkgNames = $tdeps | % {
    [String]::Join("/", $_.Split("/")[0..1])
  }

  if($pkgNames -eq $null) { $pkgNames = @() }
  # Build the list of any runtime deps that appear more than once. That is,
  # `ORIGIN/NAME` token duplicates.
  $uniques = $pkgNames | Select -Unique
  if($uniques -eq $null) { $uniques = @() }
  $dupes = Compare-object -referenceobject $uniques -differenceobject $pkgNames | Select -Unique

  if($dupes) {
    # Build a list of all fully qualified package identifiers that are members
    # of the duplicated `ORIGIN/NAME` tokens. This will be used to star the
    # problematic dependencies in the graph.
    $_dupes_qualified=$dupes | % {
        $candidate = $_.InputObject
        $tdeps | ? {
            $_.StartsWith($candidate)
        }
    }

    Write-Warning ""
    Write-Warning "The following runtime dependencies have more than one version"
    Write-Warning "release in the full dependency chain:"
    Write-Warning ""
    foreach($dupe in $dupes) {
      Write-Warning "  * $($dupe.InputObject) ( $($tdeps | ? { $_.StartsWith($dupe.InputObject) } ))"
    }
    Write-Warning ""
    Write-Warning 'The current situation usually arises when a Plan has a direct '
    Write-Warning 'dependency on one version of a package (`acme/A/7.0/20160101200001`)'
    Write-Warning 'and has a direct dependency on another package which itself depends'
    Write-Warning 'on another version of the same package (`acme/A/2.0/20151201060001`).'
    Write-Warning 'If this package (`acme/A`) contains shared libraries which are'
    Write-Warning 'loaded at runtime by the current Plan, then both versions of'
    Write-Warning '`acme/A` could be loaded into the same process in a potentially'
    Write-Warning 'surprising order. Worse, if both versions of `acme/A` are'
    Write-Warning 'ABI-incompatible, runtime segmentation faults are more than likely.'
    Write-Warning ""
    Write-Warning 'In order to preserve reliability at runtime the duplicate dependency'
    Write-Warning 'entries will need to be resolved before this Plan can be built.'
    Write-Warning 'Below is an expanded graph of all `$pkg_deps` and their dependencies'
    Write-Warning 'with the problematic lines noted.'
    Write-Warning ""
    Write-Warning "Computed dependency graph (Lines with '*' denote a problematic entry):"
    Write-Host "`n$pkg_origin/$pkg_name/$pkg_version/$pkg_release"
    _print_recursive_deps $pkg_deps_resolved $_dupes_qualified 1
    Write-Host ""
    _Exit-With "Computed runtime dependency check failed, aborting" 31
  }
}

function _Complete-DependencyResolution {
  Write-BuildLine "Resolving dependencies"

  # Build `${pkg_build_deps_resolved[@]}` containing all resolved direct build
  # dependencies.
  $script:pkg_build_deps_resolved=@()
  foreach($dep in $pkg_build_deps) {
    _install-dependency $dep
    if($resolved=(_resolve-dependency $dep)) {
      Write-BuildLine "Resolved build dependency '$dep' to $resolved"
      $script:pkg_build_deps_resolved+=($resolved)
    }
    else {
      _Exit-With "Resolving '$dep' failed, should this be built first?" 1
    }
  }

  # Build `${pkg_deps_resolved[@]}` containing all resolved direct run
  # dependencies.
  $script:pkg_deps_resolved=@()
  foreach($dep in $pkg_deps) {
    _install-dependency $dep
    if ($resolved=(_resolve-dependency $dep)) {
      Write-BuildLine "Resolved dependency '$dep' to $resolved"
      $script:pkg_deps_resolved+=$resolved
    }
    else {
      _Exit-With "Resolving '$dep' failed, should this be built first?" 1
    }
  }

  # Build `${pkg_build_tdeps_resolved[@]}` containing all the direct build
  # dependencies, and the run dependencies for each direct build dependency.

  # Copy all direct build dependencies into a new array
  $script:pkg_build_tdeps_resolved=$pkg_build_deps_resolved
  # Append all non-direct (transitive) run dependencies for each direct build
  # dependency. That's right, not a typo ;) This is how a `acme/gcc` build
  # dependency could pull in `acme/binutils` for us, as an example. Any
  # duplicate entries are dropped to produce a proper set.
  foreach($dep in $pkg_build_deps_resolved) {
    $tdeps=_Get-TdepsFor $dep
    foreach($tdep in $tdeps) {
      $tdep=(Resolve-Path "$HAB_PKG_PATH/$tdep").Path
      $script:pkg_build_tdeps_resolved=(_return_or_append_to_set $tdep $pkg_build_tdeps_resolved)
    }
  }

  # Build `${pkg_tdeps_resolved[@]}` containing all the direct run
  # dependencies, and the run dependencies for each direct run dependency.

  # Copy all direct dependencies into a new array
  $script:pkg_tdeps_resolved=$pkg_deps_resolved
  # Append all non-direct (transitive) run dependencies for each direct run
  # dependency. Any duplicate entries are dropped to produce a proper set.
  foreach($dep in $pkg_deps_resolved) {
    $tdeps=_Get-TdepsFor $dep
    foreach($tdep in $tdeps) {
      $tdep=(Resolve-Path "$HAB_PKG_PATH/$tdep").Path
      $script:pkg_tdeps_resolved=_return_or_append_to_set $tdep $pkg_tdeps_resolved
    }
  }

  # Build `${pkg_all_deps_resolved[@]}` containing all direct build and run
  # dependencies. The build dependencies appear before the run dependencies.
  $script:pkg_all_deps_resolved = $pkg_deps_resolved + $pkg_build_deps_resolved

  # Build an ordered set of all build and run dependencies (direct and
  # transitive). The order is important as this gets used when setting the
  # `$PATH` ordering in the build environment. To give priority to direct
  # dependencies over transitive ones the order of packages is the following:
  #
  # 1. All direct build dependencies
  # 1. All direct run dependencies
  # 1. All unique transitive build dependencies that aren't already added
  # 1. All unique transitive run dependencies that aren't already added
  $script:pkg_all_tdeps_resolved = $pkg_deps_resolved + $pkg_build_deps_resolved
  foreach($dep in ($pkg_tdeps_resolved + $pkg_build_tdeps_resolved)) {
    $script:pkg_all_tdeps_resolved = _return_or_append_to_set $tdep $pkg_all_tdeps_resolved
  }

  _Assert-Deps
}

function _Set-Path {
  $path_part = $null
  foreach($path in $pkg_bin_dirs) {
    if (!$path_part) {
      $path_part="$pkg_prefix/$path"
    }
    else {
      $path_part += ";$pkg_prefix/$path"
    }
  }
  foreach($dep_path in $pkg_all_tdeps_resolved) {
    if (Test-Path "$dep_path/PATH") {
      $data = Get-Content "$dep_path/PATH"
      if (!$path_part) {
        $path_part = $data.Trim()
      }
      else {
        $path_part += ";$($data.Trim())"
      }
    }
  }
  # Insert all the package PATH fragments before the default PATH to ensure
  # package binaries are used before any userland/operating system binaries
  if ($path_part) {
    $env:PATH="$path_part;$INITIAL_PATH"
  }

  Write-BuildLine "Setting PATH=$env:PATH"
}

# TODO: When we switch to powershell core, we must use
# System.Security.Cryptography.SHA256 which is not an IDisposable
function _Get-Sha256($src) {
  $converter = New-Object -TypeName Security.Cryptography.SHA256Managed
  try {
    $bytes = $converter.ComputeHash(($in = (Get-Item $src).OpenRead()))
    return ([System.BitConverter]::ToString($bytes)).Replace("-", "").ToLower()
  }
  finally {
    $converter.Dispose()
    if ($in -ne $null) { $in.Dispose() }
  }
}

# Verifies that a file on disk matches the given shasum. If the given shasum
# doesn't match the file's shasum then a warning is printed with the expected
# and computed shasum values.
#
# ```sh
# _verify_file file.tar.gz abc123...
# ```
#
# Will return 0 if the shasums match, and 1 if they do not match. A message
# will be printed to stderr with the expected and computed shasum values.
function _verify_file($dst, $sha) {
  Write-BuildLine "Verifying $dst"
  $checksum=($(_Get-Sha256 "$HAB_CACHE_SRC_PATH/$dst"))
  if ($sha -eq $checksum) {
    Write-BuildLine "Checksum verified for $dst"
  }
  else {
    Write-Warning "Checksum invalid for ${dst}:"
    Write-Warning "   Expected: $sha"
    Write-Warning "   Computed: $checksum"
    return $false
  }

  return $true
}

function _download_file($url, $dst, $sha) {
  Push-Location $HAB_CACHE_SRC_PATH
  try {
      if ((Test-Path $dst) -and $sha) {
        Write-BuildLine "Found previous file '$dst', attempting to re-use"
        if (_verify_file $dst $sha) {
          Write-BuildLine "Using cached and verified '$dst'"
          return
        }
        else {
          Write-BuildLine "Clearing previous '$dst' file and re-attempting download"
          Remove-Item $dst -Force
        }
      }

      Write-BuildLine "Downloading '$url' to '$dst'"
      Invoke-WebRequest $url -OutFile $dst
      Write-BuildLine "Downloaded '$dst'"
  }
  finally {
    Pop-Location
  }
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
    _download_file $pkg_source $pkg_filename $pkg_shasum
}

# Verify that the package we have in `$HAB_CACHE_SRC_PATH\$pkg_filename` has
# the `$pkg_shasum` we expect. Delegates most of the implementation to the
# `Invoke-DefaultVerify` function.
function Invoke-Verify {
    Invoke-DefaultVerify
}

# Default implementation for the `Invoke-Verify` phase.
function Invoke-DefaultVerify {
    if( !(_verify_file $pkg_filename $pkg_shasum)) {
        Write-Error "Verification Failed!"
    }
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
    try { Invoke-Prepare } finally { Pop-Location }
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
    try { Invoke-Build } finally { Pop-Location }
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
      try { Invoke-Check } finally { Pop-Location }
    }
}

# Identical to the `Invoke-BuildWrapper` function above; simply makes sure the
# working directory for the install_files step is correct.
function Invoke-InstallWrapper {
    Write-BuildLine "Installing"
    New-Item "$pkg_prefix" -ItemType Directory -Force | Out-Null
    foreach($dir in $pkg_lib_dirs) {
      New-Item "$pkg_prefix\$dir" -ItemType Directory -Force | Out-Null
    }
    foreach($dir in $pkg_bin_dirs) {
      New-Item "$pkg_prefix\$dir" -ItemType Directory -Force | Out-Null
    }
    foreach($dir in $pkg_include_dirs) {
      New-Item "$pkg_prefix\$dir" -ItemType Directory -Force | Out-Null
    }
    foreach($dir in $pkg_pconfig_dirs) {
      New-Item "$pkg_prefix\$dir" -ItemType Directory -Force | Out-Null
    }
    Push-Location "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    try { Invoke-Install } finally { Pop-Location }
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
        if ($pkg_svc_run -ne "" -and (Test-Path $pkg_svc_run)) {
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

    $pkg_build_deps_resolved | % {
        Resolve-HabPkgPath $_ | Out-File $pkg_prefix\BUILD_DEPS -Encoding ascii -Append
    }
    $pkg_build_tdeps_resolved | % {
        Resolve-HabPkgPath $_ | Out-File $pkg_prefix\BUILD_TDEPS -Encoding ascii -Append
    }
    $pkg_deps_resolved | % {
        Resolve-HabPkgPath $_ | Out-File $pkg_prefix\DEPS -Encoding ascii -Append
    }
    $pkg_tdeps_resolved | % {
        Resolve-HabPkgPath $_ | Out-File $pkg_prefix\TDEPS -Encoding ascii -Append
    }

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

    # The 7z tar archiving via -spf does not produce an expandable
    # archive that is readable by hab pkg install. By replicating the
    # packages directory structure in a temp directory and taring that
    # entire tree, hab pkg install is able to successfully install the
    # generated hart file.
    $tempBase = Join-Path $env:temp "hab"
    $tempPkg = "$tempBase\pkgs\$pkg_origin\$pkg_name\$pkg_version"
    if (Test-Path $tempBase) { Remove-Item $tempBase -Recurse -Force }
    New-Item $tempPkg -ItemType Directory -Force | Out-Null
    Copy-Item $pkg_prefix $tempPkg -Recurse

    & "$_7z_cmd" a -ttar "$tarf" $tempBase
    & "$_7z_cmd" a -txz "$xzf" "$tarf"
    & $HAB_BIN pkg sign --origin "$pkg_origin" "$xzf" "$pkg_artifact"
    Remove-Item "$tarf", "$xzf" -Force
    Remove-Item $tempBase -Recurse -Force
}

# **Internal** Copy the final package artifact to the `$pkg_output_path`
# directory as well as prepare a `last_build.env` report.
function _Copy-BuildOutputs {
    New-Item "$pkg_output_path" -ItemType Directory -Force | Out-Null
    Copy-Item "$pkg_artifact" "$pkg_output_path"

    $_pkg_sha256sum = (Get-FileHash "$pkg_artifact" -Algorithm SHA256).Hash.ToLower()
    $_pkg_blake2bsum = $(& $HAB_BIN pkg hash "$pkg_artifact")

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

# We want to fail the build for both termionating and non terminating errors
$ErrorActionPreference = "Stop"

# Change into the `$PLAN_CONTEXT` directory for proper resolution of relative
# paths in `plan.ps1`
Push-Location "$PLAN_CONTEXT"

try {
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
    $script:pkg_output_path = "$(Get-Location)\results"

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
}
finally {
    Pop-Location
}

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
