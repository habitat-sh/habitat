param (
    [string]
    $Context = ".",

    [string]
    $BldrUrl
)

# # License and Copyright
# ```
# Copyright: Copyright (c) 2017 Chef Software, Inc.
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

$script:env = @{
    RunTime = @{}
    BuildTime = @{}
}
$script:provenance = @{
    RunTime = @{}
    BuildTime = @{}
}

$script:originalPath = (Get-Location).Path
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
if (Test-Path Env:\HAB_ROOT_PATH) {
    $script:HAB_ROOT_PATH = "$env:HAB_ROOT_PATH"
} else {
    $script:HAB_ROOT_PATH = "\hab"
}
$resolvedRoot = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($HAB_ROOT_PATH)
# The default path where source artifacts are downloaded, extracted, & compiled
$HAB_CACHE_SRC_PATH = "$resolvedRoot\cache\src"
# The default download root path for package artifacts, used on package
# installation
$HAB_CACHE_ARTIFACT_PATH = "$resolvedRoot\cache\artifacts"
# The default path where cryptographic keys are stored. If the
# `$HAB_CACHE_KEY_PATH` environment variable is set, this value is overridden,
# otherwise it is set to its default.
if (Test-Path Env:\HAB_CACHE_KEY_PATH) {
    $script:HAB_CACHE_KEY_PATH = "$env:HAB_CACHE_KEY_PATH"
} else {
    $script:HAB_CACHE_KEY_PATH = "$resolvedRoot\cache\keys"
}
# Export the key path for other programs and subshells to use
$env:HAB_CACHE_KEY_PATH = "$script:HAB_CACHE_KEY_PATH"
# The root path containing all locally installed packages
$script:HAB_PKG_PATH = "$resolvedRoot\pkgs"
# The first argument to the script is a Plan context directory, containing a
# `plan.ps1` file
$script:PLAN_CONTEXT = "$Context"
# The default Habitat Depot from where to download dependencies. If the URL was
# provided as an option use that, if not use any set `HAB_BLDR_URL`
# environment variable, and otherwise use the default provided.
if ($BldrUrl) {
    $script:HAB_BLDR_URL = "$BldrUrl"
} elseif (Test-Path Env:\HAB_BLDR_URL) {
    $script:HAB_BLDR_URL = "$env:HAB_BLDR_URL"
} else {
    $script:HAB_BLDR_URL = "https://bldr.habitat.sh"
}
# Export the Builder URL so all other programs and subshells use this same one
$env:HAB_BLDR_URL = "$script:HAB_BLDR_URL"
if (!(Test-Path Env:\HAB_BLDR_CHANNEL)) {
    $env:HAB_BLDR_CHANNEL = "stable"
}
$script:FALLBACK_CHANNEL = "stable"
# The value of `$env:Path` on initial start of this program
$script:INITIAL_PATH = "$env:Path"
# The full target tuple this plan will be built for
$script:pkg_target = "@@pkg_target@@"
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
# An associative array representing configuration data which should be gossiped to peers.
$script:pkg_exports = @{}
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
        $script:_hab_cmd = (Get-Command hab.exe).Source
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

# **Internal**  Build and export `$PATH` containing each path in our own
# `${pkg_bin_dirs[@]}` array, and then any dependency's `PATH` entry (direct or
# transitive) if one exists. The ordering of the path is specific to
# `${pkg_all_tdeps_resolved[@]}` which is further explained in the
# `_resolve_dependencies()` function.
#
# Reference implementation:
# https://github.com/habitat-sh/habitat/blob/3d63753468ace168bbbe4c52e600d408c4981b03/components/plan-build/bin/hab-plan-build.sh#L1584-L1638
function _Set-BuildPath {
  $prefixDrive = (Resolve-Path $originalPath).Drive.Root

  $paths=@()

  # Add element for each entry in `$pkg_bin_dirs[@]` first
  foreach($dir in $pkg_bin_dirs) {
    $paths += "$pkg_prefix\$dir"
  }

  # Iterate through all build and run dependencies in the order present in
  # `${pkg_all_tdeps_resolved[@]}` and for each, append each path entry onto
  # the result, assuming it hasn't already been added. Additionally, any path
  # entries that don't relate to the dependency in question are filtered out to
  # deal with a vintage of packages which included more data in `PATH` and have
  # since been addressed.
  foreach($dep_prefix in $pkg_all_tdeps_resolved) {
    if (Test-Path (Join-Path $dep_prefix "PATH")) {
      $data = (Get-Content (Join-Path $dep_prefix "PATH") | Out-String).Trim()
      foreach($entry in $data.split(";")) {
        $paths = @(_return_or_append_to_set (_Resolve-Path $entry) $paths)
      }
    } elseif (Test-Path (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT")) {
      # Backwards Compatibility: If `PATH` can't be found, then attempt to fall
      # back to looking in an existing `RUNTIME_ENVIRONMENT` metadata file for
      # a `PATH` entry. This is necessary for packages created using a release
      # of Habitat between 0.53.0 (released 2018-02-05) and up to including
      # 0.55.0 (released 2018-03-20).
      $strippedPrefix = $dep_prefix.Substring($prefixDrive.length)
      if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

      foreach ($line in (Get-Content (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT"))) {
          $varval = $line.split("=")
          if ($varval[0] -eq "PATH") {
              foreach($entry in $varval[1].split(";")) {
                # Filter out entries that are not related to the `$dep_prefix`
                if ("$entry" -Like "$strippedPrefix\*") {
                  $paths = @(_return_or_append_to_set (_Resolve-Path $entry) $paths)
                }
              }
              break;
          }
      }
    }
  }

  $paths += $INITIAL_PATH

  $env:PATH = $paths -join ';'
  Write-BuildLine "Setting env:PATH=$env:PATH"
}

# At this phase of the build, all dependencies are downloaded, the build
# environment is set, but this is just before any source downloading would
# occur (if `$pkg_source` is set). This could be a suitable phase in which to
# compute a dynamic version of a pacakge given the state of a Git repository,
# fire an API call, start timing something, etc.
function Invoke-Before {
  Invoke-DefaultBefore
}

# Default implementation for the `Invoke-Before` phase.
function Invoke-DefaultBefore {
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
    $cmd = "$HAB_BIN install -u $env:HAB_BLDR_URL --channel $env:HAB_BLDR_CHANNEL $dependency"
    if($env:HAB_FEAT_IGNORE_LOCAL -eq "true") { $cmd += " --ignore-local" }
    Invoke-Expression $cmd
    if ($LASTEXITCODE -ne 0 -and ($env:HAB_BLDR_URL -ne $FALLBACK_CHANNEL)) {
      Write-BuildLine "Trying to install '$dependency' from '$FALLBACK_CHANNEL'"
      $cmd = "$HAB_BIN install -u $env:HAB_BLDR_URL --channel $FALLBACK_CHANNEL $dependency"
      if($env:HAB_FEAT_IGNORE_LOCAL -eq "true") { $cmd += " --ignore-local" }
      Invoke-Expression $cmd
    }
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

# **Internal**  Build a `PATH` string suitable for entering into this package's
# `RUNTIME_PATH` metadata file. The ordering of this path is important as this
# value will ultimately be consumed by other programs such as the Supervisor
# when constructing the `PATH` environment variable before spawning a process.
#
# The path is constructed by taking all `PATH` metadata file entries from this
# package (in for the form of `$pkg_bin_dirs[@]`), followed by entries from the
# *direct* dependencies first (in declared order), and then from any remaining
# transitive dependencies last (in lexically sorted order). All entries are
# present only once in the order of their first appearance.
function _Assemble-RuntimePath() {
  # Contents of `pkg_xxx_dirs` are relative to the plan root;
  # prepend the full path to this release so everything resolves
  # properly once the package is installed.
  $prefixDrive = (Resolve-Path $originalPath).Drive.Root
  $strippedPrefix = $pkg_prefix.Substring($prefixDrive.length)
  if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

  $paths = @()

  # Add element for each entry in `$pkg_bin_dirs[@]` first
  foreach($dir in $pkg_bin_dirs) {
    $paths += "$strippedPrefix\$dir"
  }

  # Iterate through all direct direct run dependencies following by all
  # remaining transitive run dependencies and for each, append each path entry
  # onto the result, assuming it hasn't already been added. In this way, all
  # direct dependencies will match first and any programs that are used by a
  # direct dependency will also be present on PATH, albeit at the very end of
  # the PATH. Additionally, any path entries that don't relate to the
  # dependency in question are filtered out to deal with a vintage of packages
  # which included more data in `PATH` and have since been addressed.
  foreach($dep_prefix in ($pkg_deps_resolved + $pkg_tdeps_resolved)) {
    if (Test-Path (Join-Path $dep_prefix "PATH")) {
      $data = (Get-Content (Join-Path $dep_prefix "PATH") | Out-String).Trim()
      foreach($entry in $data.split(";")) {
        $paths = @(_return_or_append_to_set $entry $paths)
      }
    } elseif (Test-Path (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT")) {
      # Backwards Compatibility: If `PATH` can't be found, then attempt to fall
      # back to looking in an existing `RUNTIME_ENVIRONMENT` metadata file for
      # a `PATH` entry. This is necessary for packages created using a release
      # of Habitat between 0.53.0 (released 2018-02-05) and up to including
      # 0.55.0 (released 2018-03-20).
      $strippedPrefix = $dep_prefix.Substring($prefixDrive.length)
      if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

      foreach ($line in (Get-Content (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT"))) {
          $varval = $line.split("=")
          if ($varval[0] -eq "PATH") {
              foreach($entry in $varval[1].split(";")) {
                # Filter out entries that are not related to the `$dep_prefix`
                if ("$entry" -Like "$strippedPrefix\*") {
                  $paths = @(_return_or_append_to_set (_Resolve-Path $entry) $paths)
                }
              }
              break;
          }
      }
    }
  }

  # Return the elements of the result, joined with a colon
  $paths -join ';'
}

# **Internal** Returns (on stdout) the `DEPS` file contents of another locally
# installed package which contain the set of all direct run dependencies. An
# empty set could be returned as whitespace and/or newlines.  The lack of a
# `DEPS` file in the desired package will be considered an unset, or empty set.
#
# ```
# _Get-DepsFor /hab/pkgs/acme/a/4.2.2/20160113044458
# # /hab/pkgs/acme/dep-b/1.2.3/20160113033619
# # /hab/pkgs/acme/dep-c/5.0.1/20160113033507
# # /hab/pkgs/acme/dep-d/2.0.0/20160113033539
# # /hab/pkgs/acme/dep-e/10.0.1/20160113033453
# # /hab/pkgs/acme/dep-f/4.2.2/20160113033338
# # /hab/pkgs/acme/dep-g/4.2.2/20160113033319
# ```
#
# Will return 0 in any case and the contents of `DEPS` if the file exists.
function _Get-DepsFor($dependency) {
  if (Test-Path "$dependency/DEPS") {
    Get-Content $dependency/DEPS
  }
  else {
    # No file, meaning an empty set
    @()
  }
}

function _Get-BuildDepsFor($dependency) {
  if (Test-Path "$dependency/BUILD_DEPS") {
    Get-Content $dependency/BUILD_DEPS
  }
  else {
    # No file, meaning an empty set
    @()
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

# **Internal** Verifies that any lazily-computed, required variables have been
# set, otherwise it fails the build.
function _Assert-Vars {
    if("$pkg_version" -eq "__pkg__version__unset__") {
      $e="Plan did not set 'pkg_version' and did not call 'Set-PkgVersion'"
      $e="$e before the 'Invoke-Prepare' build phase."
      _exit-with $e 2
    }
  
    $script:_verify_vars=$true
  }
  
# **Internal** Create initial package-related arrays.
function _init-Dependencies {
  # Create `${pkg_build_deps_resolved[@]}` containing all resolved direct build
  # dependencies.
  $script:pkg_build_deps_resolved=@()

  # Create `${pkg_build_tdeps_resolved[@]}` containing all the direct build
  # dependencies, and the run dependencies for each direct build dependency.
  $script:pkg_build_tdeps_resolved=@()

  # Create `${pkg_deps_resolved[@]}` containing all resolved direct run
  # dependencies.
  $script:pkg_deps_resolved=@()

  # Create `${pkg_tdeps_resolved[@]}` containing all the direct run
  # dependencies, and the run dependencies for each direct run dependency.
  $script:pkg_tdeps_resolved=@()
}

function _Resolve-ScaffoldingDependencies {
  Write-BuildLine "Resolving scaffolding dependencies"
  $scaff_build_deps = @()
  $scaff_build_deps_resolved = @()
  foreach($dep in $pkg_scaffolding) {
    _install-dependency $dep
    # Add scaffolding package to the list of scaffolding build deps
    $scaff_build_deps += $dep
    if($resolved=(_resolve-dependency $dep)) {
      Write-BuildLine "Resolved scaffolding dependency '$dep' to $resolved"
      $scaff_build_deps_resolved+=($resolved)
      $sdeps=(@(_Get-DepsFor $resolved) + @(_Get-BuildDepsFor $resolved))
      foreach($sdep in $sdeps) {
          $scaff_build_deps += $sdep
          $scaff_build_deps_resolved+=(Resolve-Path "$HAB_PKG_PATH/$sdep").Path
      }
    }
    else {
      _Exit-With "Resolving '$dep' failed, should this be built first?" 1
    }
  }

  # Add all of the ordered scaffolding dependencies to the start of
  # `${pkg_build_deps[@]}` to make sure they could be overridden by a Plan
  # author if required.
  $script:pkg_build_deps=$scaff_build_deps + $pkg_build_deps
  Write-debug "Updating pkg_build_deps=$pkg_build_deps from Scaffolding deps"

  # Set `pkg_build_deps_resolved[@]}` to all resolved scaffolding dependencies.
  # This will be used for early scaffolding package loading to mimic the state
  # where all dependencies are known for helpers such as `pkg_path_for` and
  # will be re-set later when the full build dependency set is known.
  $script:pkg_build_deps_resolved=$scaff_build_deps_resolved
  # Set `${pkg_build_tdeps_resolved[@]}` to all the direct scaffolding
  # dependencies, and the run dependencies for each direct scaffolding
  # dependency. As above, this will be re-set later when the full dependency
  # set is known.
  _Set-BuildTdepsResolved
}

function _Set_DependencyArrays {
  # Build `${pkg_all_deps_resolved[@]}` containing all direct build and run
  # dependencies. The build dependencies appear before the run dependencies.
  $script:pkg_all_deps_resolved = $pkg_deps_resolved + $pkg_build_deps_resolved
  # Build an ordered set of all build and run dependencies (direct and
  # transitive). The order is important as this gets used when setting the
  # `$PATH` ordering in the build environment. To give priority to direct
  # dependencies over transitive ones the order of packages is the following:
  #
  # 1. All direct run dependencies
  # 2. All direct build dependencies
  # 3. All unique transitive run dependencies that aren't already added
  # 4. All unique transitive build dependencies that aren't already added
  $script:pkg_all_tdeps_resolved = $pkg_deps_resolved + $pkg_build_deps_resolved
  foreach($dep in ($pkg_tdeps_resolved + $pkg_build_tdeps_resolved)) {
    $script:pkg_all_tdeps_resolved = @(_return_or_append_to_set $dep $pkg_all_tdeps_resolved)
  }
}

# **Internal** Sets the value of `${pkg_build_tdeps_resolved[@]}`. This
# function completely re-sets the value of `${pkg_build_tdeps_resolved[@]}`
# using the current value of `${pkg_build_deps_resolved[@]}`.
 function _Set-BuildTdepsResolved {
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
      $script:pkg_build_tdeps_resolved=@(_return_or_append_to_set $tdep $pkg_build_tdeps_resolved)
    }
  }
 }

 function _Resolve-BuildDependencies {
  # Build `${pkg_build_deps_resolved[@]}` containing all resolved direct build
  # dependencies.
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

  _Set-BuildTdepsResolved
 }

 function _Resolve-RunDependencies {
  # Build `${pkg_deps_resolved[@]}` containing all resolved direct run
  # dependencies.
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

  # Copy all direct dependencies into a new array
  $script:pkg_tdeps_resolved=$pkg_deps_resolved
  # Append all non-direct (transitive) run dependencies for each direct run
  # dependency. Any duplicate entries are dropped to produce a proper set.
  foreach($dep in $pkg_deps_resolved) {
    $tdeps=_Get-TdepsFor $dep
    foreach($tdep in $tdeps) {
      $tdep=(Resolve-Path "$HAB_PKG_PATH/$tdep").Path
      $script:pkg_tdeps_resolved=@(_return_or_append_to_set $tdep $pkg_tdeps_resolved)
    }
  }
 }

 function _Resolve-Paths($paths) {
    $path_part = $null
    Push-Location $originalPath
    try {
        foreach($path in $paths.split(";")) {
            $data = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($path)
            if (!$path_part) {
            $path_part = $data
            }
            else {
            $path_part += ";$data"
            }
        }
    }
    finally { Pop-Location }
    $path_part
  }

function _Resolve-Path($path) {
  $result = $null
  Push-Location $originalPath
  try {
    $result = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($path)
  }
  finally { Pop-Location}
  $result
}

function _Write-Pre-Build-File {
    New-Item "$pkg_output_path" -ItemType Directory -Force | Out-Null
    $preBuild = "$pkg_output_path\pre_build.env"
    if (Test-Path $preBuild) { Remove-Item $preBuild -Force }

    @"
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
"@ | Out-File "$pkg_output_path\pre_build.env" -Encoding ascii
}

function _Get-SHA256Converter {
  if($PSVersionTable.PSEdition -eq 'Core') {
    [System.Security.Cryptography.SHA256]::Create()
  }
  else {
    New-Object -TypeName Security.Cryptography.SHA256Managed
  }
}

function _Get-Sha256($src) {
  $converter = _Get-SHA256Converter
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
      Invoke-WebRequest $url -OutFile $dst -UserAgent "Habitat"
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
    if($pkg_source -and $pkg_source -ne "") {
        _download_file $pkg_source $pkg_filename $pkg_shasum
    }
}

# Verify that the package we have in `$HAB_CACHE_SRC_PATH\$pkg_filename` has
# the `$pkg_shasum` we expect. Delegates most of the implementation to the
# `Invoke-DefaultVerify` function.
function Invoke-Verify {
    Invoke-DefaultVerify
}

# Default implementation for the `Invoke-Verify` phase.
function Invoke-DefaultVerify {
    if($pkg_filename -and $pkg_filename -ne "") {
        if( !(_verify_file $pkg_filename $pkg_shasum)) {
            Write-Error "Verification Failed!"
        }
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
    if($pkg_filename -ne $null) {
        Expand-Archive -Path "$HAB_CACHE_SRC_PATH/$pkg_filename" -DestinationPath "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    }
}

function _Set-Environment {
    $prefixDrive = (Resolve-Path $originalPath).Drive.Root

    $libs = @()
    $includes = @()

    # Add element for each entry in `$pkg_lib_dirs[@]` first
    foreach ($dir in $pkg_lib_dirs) {
        $libs += "$pkg_prefix\$dir"
    }

    # Add element for each entry in `$pkg_include_dirs[@]` first
    foreach ($dir in $pkg_include_dirs) {
        $includes += "$pkg_prefix\$dir"
    }

    foreach ($dep_prefix in $pkg_all_deps_resolved) {
        $strippedPrefix = $dep_prefix.Substring($prefixDrive.length)
        if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

        if (Test-Path (Join-Path $dep_prefix "LIB_DIRS")) {
            $data = (Get-Content (Join-Path $dep_prefix "LIB_DIRS") | Out-String).Trim()
            foreach ($entry in $data.split(";")) {
                $libs = @(_return_or_append_to_set (_Resolve-Path $entry) $libs)
            }
        } elseif (Test-Path (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT")) {
            # Backwards Compatibility: If `LIB_DIRS` can't be found, then
            # attempt to fall back to looking in an existing
            # `RUNTIME_ENVIRONMENT` metadata file for a `LIB` entry. This is
            # necessary for packages created using a release of Habitat between
            # 0.53.0 (released 2018-02-05) and up to including 0.55.0 (released
            # 2018-03-20).
            foreach ($line in (Get-Content (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT"))) {
                $varval = $line.split("=")
                if ($varval[0] -eq "LIB") {
                    foreach($entry in $varval[1].split(";")) {
                        # Filter out entries that are not related to the `$dep_prefix`
                        if ("$entry" -Like "$strippedPrefix\*") {
                            $libs = @(_return_or_append_to_set (_Resolve-Path $varval[1]) $libs)
                        }
                    }
                    break;
                }
            }
        }

        if (Test-Path (Join-Path $dep_prefix "INCLUDE_DIRS")) {
            $data = (Get-Content (Join-Path $dep_prefix "INCLUDE_DIRS") | Out-String).Trim()
            foreach($entry in $data.split(";")) {
                $includes = @(_return_or_append_to_set (_Resolve-Path $entry) $includes)
            }
        } elseif (Test-Path (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT")) {
            # Backwards Compatibility: If `INCLUDE_DIRS` can't be found, then
            # attempt to fall back to looking in an existing
            # `RUNTIME_ENVIRONMENT` metadata file for a `INCLUDE` entry. This
            # is necessary for packages created using a release of Habitat
            # between 0.53.0 (released 2018-02-05) and up to including 0.55.0
            # (released 2018-03-20).
            foreach ($line in (Get-Content (Join-Path $dep_prefix "RUNTIME_ENVIRONMENT"))) {
                $varval = $line.split("=")
                if ($varval[0] -eq "INCLUDE") {
                    foreach($entry in $varval[1].split(";")) {
                        # Filter out entries that are not related to the `$dep_prefix`
                        if ("$entry" -Like "$strippedPrefix\*") {
                            $includes = @(_return_or_append_to_set (_Resolve-Path $varval[1]) $includes)
                        }
                    }
                    break;
                }
            }
        }
    }

    $env:LIB = $libs -join ';'
    $env:INCLUDE = $includes -join ';'

    # Create a working directory if it doesn't already exist from `Invoke-Unpack`
    New-Item "$HAB_CACHE_SRC_PATH\$pkg_dirname" -ItemType Directory -Force |
        Out-Null

    Write-BuildLine "Setting env:LIB=$env:LIB"
    Write-BuildLine "Setting env:INCLUDE=$env:INCLUDE"
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

# (I'm sure there are many other common variables we could add here;
# PRs welcome!)
$__well_known_aggregate_env_vars=@{
    # Shell
    INCLUDE=";"
    LIB=";"
    PATH=";"
    PATHEXT=";"
    PSModulePath=";"

    # Go
    GOPATH=";"

    # Java
    CLASSPATH=";"

    # NodeJS
    NODE_PATH=";"

    # Python
    PYTHONPATH=";"

    # Ruby
    BUNDLE_PATH=";"
    BUNDLE_WITHOUT=";"
    GEM_PATH=";"
    RUBYLIB=";"
    RUBYPATH=";"
}

function Invoke-SetupEnvironment {
}

function Invoke-SetupEnvironmentWrapper {
    Write-BuildLine "Setting up environment"
    Write-BuildLine "Populating runtime environment from dependencies"
    __populate_environment_from_deps "RunTime"
    Write-BuildLine "Populating buildtime environment from dependencies"
    __populate_environment_from_deps "BuildTime"

    Invoke-SetupEnvironment

    Write-BuildLine "Layering runtime environment on top of system environment"
    # Export everything from our collected runtime environment into
    # the real environment, except for PATH; for that, push the
    # runtime path onto the front of the system path
    foreach($k in $env["Runtime"].keys) {
        if(@("PATH", "LIB", "INCLUDE") -contains $k) {
            $currentVal = ""
            if(Test-path env:\$k) {
                $currentVal = Get-Content env:\$k
            }
            New-Item -Name $k -Value (push_to_path (_Resolve-Paths $env["Runtime"][$k]) $currentVal) -ItemType Variable -Path Env: -Force | Out-Null
        } else {
            New-Item -Name $k -Value $env["Runtime"][$k] -ItemType Variable -Path Env: -Force | Out-Null
        }
        Write-BuildLine "Value of $k is $(Get-Content env:\$k)"
    }

    Write-BuildLine "Layering buildtime environment on top of system environment"
    # Layer buildtime environment values into the system environment,
    # which has already had the runtime values merged in. This is a
    # stripped-down version of the logic used to layer environments
    # from dependencies in the first place.
    foreach($k in $env["Buildtime"].keys) {
        $val = $env["Buildtime"][$k]
        if(@("PATH", "LIB", "INCLUDE") -contains $k) {
            $val = _Resolve-Paths $val
        }

        if(Test-Path env:\$k) {
            # There was a previous value; need to figure out
            # how to proceed
            if((Get-Content -Path env:\$k) -eq $val) {
                # If the value is the same as what we've got,
                # there's nothing to do
                continue
            }

            switch(__env_var_type $k) {
                "primitive" {
                    New-Item -Name $k -Value $val -ItemType Variable -Path Env: -Force | Out-Null
                }
                "aggregate" {
                    New-Item -Name $k -Value (push_to_path $val (Get-Content -Path env:\$k) (__env_aggregate_separator $k)) -ItemType Variable -Path Env: -Force | Out-Null
                }
            }
            Write-BuildLine "Value of $k is $(Get-Content env:\$k)"
        } else {
            # There was no previous value; just set this one
            New-Item -Name $k -Value $val -ItemType Variable -Path Env: -Force | Out-Null
            Write-BuildLine "Value of $k is $(Get-Content env:\$k)"
        }
    }
}

# Given that a variable is an aggregate (i.e., PATH-style) variable,
# return the separator character used to delimit items in the value.
function __env_aggregate_separator($VarName) {
    $hint_var = Get-Variable "HAB_ENV_$VarName_TYPE" -ErrorAction SilentlyContinue

    if($hint_var) {
        # Look for user-specified hints first
        $hint_var.Value
    } elseif($__well_known_aggregate_env_vars.ContainsKey($varname)) {
        # Look in our built-in map to see if we know anything about it
        $__well_known_aggregate_env_vars[$VarName]
    } else {
        # Just assume it's the default
        ';'
    }
}

# Read in the RUNTIME_ENVIRONMENT files from all direct dependencies
# (in `pkg_deps` / `pkg_build_deps` order!) and layer them as appropriate.
function __populate_environment_from_deps {
    param(
        [ValidateSet('BuildTime','RunTime')]
        [System.String]$Environment
    )

    $dep_array=$pkg_deps
    if($Environment -eq "BuildTime") {
        $dep_array = $pkg_build_deps
    }


    foreach($dep in $dep_array) {
        $path_to_dep = Get-HabPackagePath $dep.Split("/")[1]

        $dep_ident = (Get-Content "$path_to_dep/IDENT").Trim()

        if(Test-Path "$path_to_dep/RUNTIME_ENVIRONMENT") {
            foreach($line in (Get-Content "$path_to_dep/RUNTIME_ENVIRONMENT")) {
                $varval = $line.split("=")
                # Any values of `PATH`, `LIB`, and `INCLUDE` are skipped as we
                # will be computing these variables independently of the
                # RUNTIME_ENVIRONMENT metadata files. Additionally, this acts
                # as backwards compatibility for all `RUNTIME_ENVIRONMENT`
                # files that contain a `PATH` key.
                if(@("PATH", "LIB", "INCLUDE") -contains $varval[0]) {
                  continue
                }

                if($env[$Environment].ContainsKey($varval[0])) {
                    # There was a previous value; need to figure out
                    # how to proceed

                    # Where did the value come from originally?
                    if($varval[1] -eq $env[$Environment][$varval[0]]) {
                        # If the value is the same as what we've got,
                        # there's nothing to do
                        continue
                    }

                    switch(__env_var_type $varval[0]) {
                        "primitive" {
                            Write-Warning "Overwriting `$env:$($varval[0]) originally set from $($provenance[$Environment][$varval[0]])"
                            __set_env $Environment $varval[0] $varval[1] $dep_ident
                        }
                        "aggregate" {
                            Write-Warning "Prepending to `$env:$($varval[0]) originally set from $($provenance[$Environment][$varval[0]])"

                            # if aggregate, push to front with separator
                            __push_env $Environment $varval[0] $varval[1] (__env_aggregate_separator $varval[0]) $dep_ident
                        }
                    }
                }
                else {
                    # There was no previous value; just add this one
                    $env[$Environment][$varval[0]]=$varval[1]
                    $provenance[$Environment][$varval[0]]="${dep_ident}"
                }
            }
        }
    }
}

# Internal function implementing core "set" logic for environment variables.
function __set_env($EnvName, $VarName, $VarValue, $ident){
    $env[$Environment][$VarName]=$VarValue
    $provenance[$Environment][$VarName]=$ident
}

# Internal function implementing core "push" logic for environment variables.
function __push_env($EnvName, $VarName, $VarValue, $separator, $ident) {
    # If there is no current value (that is, $current_value == ""), we
    # can still push onto that with no loss of generality. Because
    # push_to_path also dedupes the result, this allows us to take
    # $value inputs that are themselves paths, which may have
    # duplicate or blank entries (as is the case with some existing
    # Habitat metadata files) and this will effectively "clean" them
    # for us!
    $current_value=$env[$Environment][$VarName]
    $new_value=$(push_to_path $VarValue $current_value $Separator)
    $env[$Environment][$VarName]=$new_value

    $existing_provenance = $provenance[$Environment][$VarName]
    $provenance[$Environment][$VarName]=$(push_to_path $ident $existing_provenance)
}

# Pushes $ITEM onto $PATH (using optional $SEPARATOR) and then
# deduplicates entries.
#
# push_to_path "foo" "bar:foo:baz"
#   -> "foo:bar:baz"
#
# push_to_path "foo" ""
#   -> "foo"
#
# push_to_path "foo" "bar;baz" ";"
#   -> "foo;bar;baz"
#
function push_to_path($item, $path, $separator = ";") {
    if(!$path -or ($path -eq "")) {
        $temp=$item
    }
    else {
        $temp="$item$separator$path"
    }
    dedupe_path $temp $separator
}

function dedupe_path($path, $separator = ";"){
    $pathArray = $path.Split($separator)
    $pathArray = $pathArray | Select -Unique
    [String]::Join($separator, $pathArray)
}

function __env_var_type($VarName) {
    $hint_var = Get-Variable "HAB_ENV_${VarName}_TYPE" -ErrorAction SilentlyContinue

    if($hint_var) {
        # Look for user-specified hints first
        $hint_var.Value
    }
    elseif($__well_known_aggregate_env_vars.ContainsKey($varname)) {
        # Look in our built-in map to see if we know anything about it
        'aggregate'
    }
    else {
        # We know nothing about it; treat it as a primitive
        Write-Warning "Treating `$$varName as a primitive type. If you would like to change this, add `"HAB_ENV_${VarName}_TYPE='aggregate'`" to your plan."
        'primitive'
    }
}

# Simply converts contents of pkg_bin_dirs, pkg_lib_dirs and pkg_include_dirs
# into a PATH, LIB and INCLUDE variable
function __process_paths($Environment) {
    if($Environment -ne "Runtime") { return }

    # Contents of `pkg_xxx_dirs` are relative to the plan root;
    # prepend the full path to this release so everything resolves
    # properly once the package is installed.
    $prefixDrive = (Resolve-Path $originalPath).Drive.Root
    $strippedPrefix = $pkg_prefix.Substring($prefixDrive.length)
    if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

    if ($pkg_bin_dirs.Length -gt 0) {
        $path = $($pkg_bin_dirs | % { "$strippedPrefix\$_" }) -join ';'
        __push_env $Environment "PATH" $path ";" "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
    }

    if ($pkg_lib_dirs.Length -gt 0) {
        $lib = $($pkg_lib_dirs | % { "$strippedPrefix\$_" }) -join ';'
        __push_env $Environment "LIB" $lib ";" "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
    }

    if ($pkg_include_dirs.Length -gt 0) {
        $include = $($pkg_include_dirs | % { "$strippedPrefix\$_" }) -join ';'
        __push_env $Environment "INCLUDE" $include ";" "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
    }
}

# For dependencies that do not have a RUNTIME_ENVIRONMENT file, we'll
# at least assemble a PATH, LIB, and INCLUDE variable that can be used. To be complete
# and utterly paranoid, we'll iterate through the TDEPS of the
# dependency and assemble a PATH by layering all the PATHs of each of
# those dependencies.
#
# Don't need to do this for packages that already have a
# RUNTIME_ENVIRONMENT file, because this operation will have already
# been done when those packages were built. This is to help preserve
# backward compatibility.
function __assemble_legacy_paths($DepPath, $pathFile) {
    $path=""

    $tdeps = (_Get-TdepsFor $DepPath)
    foreach($tdep in $tdeps) {
        # If a PATH exists for this transitive dependency, collect
        # it, prepending to the beginning of our accumulating path
        $path_to_tdep=Join-Path $tdep $pathFile
        if(Test-Path $path_to_tdep) {
            $tdep_path = Get-Content $path_to_tdep | Out-String
            if($tdep_path.length -gt 0) {
                $path = push_to_path $tdep_path $path
            }
        }
    }
    # If the current dependency has PATH data itself, prepend it to
    # what we've been accumulating so far
    $path_to_dep=Join-Path $DepPath $pathFile
    if(Test-Path $path_to_dep) {
        $dep_path = (Get-Content $path_to_dep | Out-String).Trim()
        if($dep_path.length -gt 0) {
            $path = push_to_path $dep_path $path
        }
    }

    # Finally, return the path
    $path
}

function Set-BuildtimeEnv($VarName, $VarValue, [switch]$force) {
    set_env "BuildTime" @PSBoundParameters
}

function Set-RuntimeEnv($VarName, $VarValue, [switch]$force) {
    set_env "RunTime" @PSBoundParameters
}

function set_env($Environment, $VarName, $VarValue, [switch]$force) {
    __fail_on_protected_env_var_manipulation $VarName

    if($env[$Environment].ContainsKey($VarName)) {
        if(!$force) {
            _Exit-With "Already have a value for `$$VarName, set by $($provenance[$Environment][$VarName]): $($env[$Environment][$VarName]). If you really wish to overwrite this value, pass the '-force' option when setting it." 1
        } else {
            Write-Warning "Already have a value for `$$VarName, set by $($provenance[$Environment][$VarName]): $($env[$Environment][$VarName]). Overwriting value because the '-Force' flag was passed"
        }
    }

    __set_env $Environment $VarName $VarValue "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
}

function __fail_on_protected_env_var_manipulation($VarName) {
    $protected=@{
        PATH="pkg_bin_dirs"
        LIB="pkg_lib_dirs"
        LD_RUN_PATH="pkg_lib_dirs"
        LDFLAGS="pkg_lib_dirs"
        INCLUDE="pkg_include_dirs"
        CFLAGS="pkg_include_dirs"
        CPPFLAGS="pkg_include_dirs"
        CXXFLAGS="pkg_include_dirs"
        PKG_CONFIG_PATH="pkg_pconfig_dirs"
    }
    foreach($p in $protected.Keys) {
        if($VarName -eq $p) {
            _Exit-With "Cannot directly manipulate environment variable $VarName! Add appropriate entries to the '$($protected[$VarName])' variable in plan.ps1 instead!"
        }
    }
}

function Push-BuildtimeEnv($VarName, $VarValue) {
    Write-BuildLine "PUSH $VarName TO BUILD"
    do_push_env "BuildTime" @PSBoundParameters
}

function Push-RuntimeEnv($VarName, $VarValue) {
    Write-BuildLine "PUSH $VarName TO RUN"
    do_push_env "RunTime" @PSBoundParameters
}

function do_push_env($Environment, $VarName, $VarValue) {
    __fail_on_protected_env_var_manipulation $VarName

    __push_env $Environment $VarName $VarValue (__env_aggregate_separator $VarName) "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
}

# Updates the value of `$pkg_version` and recomputes any relevant variables.
# This function must be called before the `Invoke-Pepare` build phase otherwise
# it will fail the build process.
#
# This function depends on the Plan author implementing a `pkg_version`
# function which prints a computed version string on standard output. Then,
# this function must be explicitly called in an appropriate build phase--most
# likely `Invoke-Before`. For example:
#
# ```sh
# $pkg_origin="acme"
# $pkg_name="myapp"
#
# function pkg_version {
#   cat "$SRC_PATH/version.txt"
# }
#
# function Invoke-Before {
#   Invoke-DefaultBefore
#   Set-PkgVersion
# }
# ```
function Set-PkgVersion {
    if($_verify_vars) {
      $e="Plan called 'Set-PkgVersion' in phase 'Invoke-Prepare' or later"
      $e="$e which is not supported. Package version must be determined before"
      $e="$e 'Invoke-Prepare' phase."
      _exit-with $e 21
    }
  
    if(test-path function:\pkg_version) {
      $script:pkg_version=pkg_version
      Write-BuildLine "Version updated to '$pkg_version'"
    } else {
      write-debug "pkg_version function not found, retaining pkg_version=$pkg_version"
    }
  
    # `$pkg_dirname` needs to be recomputed, unless it was explicitly set by the
    # Plan author.
    if($_pkg_dirname_initially_unset) {
      $script:pkg_dirname="$pkg_name-$pkg_version"
    }
    $script:pkg_prefix="$HAB_PKG_PATH/$pkg_origin/$pkg_name/$pkg_version/$pkg_release"
    $script:pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/$pkg_origin-$pkg_name-$pkg_version-$pkg_release-${pkg_target}.${_artifact_ext}"
    # If the `$CACHE_PATH` and `$SRC_PATH` are the same, then we are building
    # third party software using `$pkg_source` and
    # downloading/verifying/unpacking it.
    if("$CACHE_PATH" -eq "$SRC_PATH") {
      $update_src_path=$true
    }
    $CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"
    # Only update `$SRC_PATH` if we are building third party software using
    # `$pkg_source`.
    if($update_src_path) {
      $script:SRC_PATH=$CACHE_PATH
    }
    # Replace the unset placeholders with the computed value
    $env:PATH=$(__resolve_version_placeholder $env:PATH $pkg_version)
    Write-BuildLine "Updating PATH=$env:PATH"
  
    # TODO (CM): Do not like this separation of concerns (or lack of
    # separation, as the case may be).
    #
    # NOTE: we specifically handle PATH above (and make that live in the
    # environment). We are implicitly assuming that any other instances
    # of the version placeholder are not going to need to be propagated
    # back into the active environment.
    __resolve_all_version_placeholders $env["Runtime"] $pkg_version
    __resolve_all_version_placeholders $env["Buildtime"] $pkg_version
    __resolve_all_version_placeholders $provenance["Runtime"] $pkg_version
    __resolve_all_version_placeholders $provenance["Buildtime"] $pkg_version
}

# Replace all instances of the "__pkg__version__unset__" placeholder
# in the given string with the real version number.
function __resolve_version_placeholder($original, $real_version){
    $original.Replace("__pkg__version__unset__", $real_version)
}

# Replace all instances of the "__pkg__version__unset__" placeholder
# in the values of the given hashtable with the real version number.
function __resolve_all_version_placeholders($env_table, $real_version) {
    $new_table = @{}
    foreach($k in $env_table.keys) {
        $new_table[$k] = (__resolve_version_placeholder $env_table[$k] $real_version)
    }

    foreach($k in $new_table.keys) {
        $env_table[$k] = $new_table[$k]
    }
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
        if ($pkg_svc_run -ne "") {
          Write-BuildLine "Writing $pkg_prefix/run script to run $pkg_svc_run"
          Set-Content -Path "$pkg_prefix/run" -Value @"
cd (Join-Path "`$env:FS_ROOT" "$pkg_svc_path")

`$cmd = @"
$pkg_svc_run
"`@
Invoke-Expression -Command `$cmd
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

    $prefixDrive = (Resolve-Path $originalPath).Drive.Root
    $strippedPrefix = $pkg_prefix.Substring($prefixDrive.length)
    if(!$strippedPrefix.StartsWith('\')) { $strippedPrefix = "\$strippedPrefix" }

    if ($pkg_bin_dirs.Length -gt 0) {
        $($pkg_bin_dirs | % { "$strippedPrefix\$_" }) -join ';' |
            Out-File "$pkg_prefix\PATH" -Encoding ascii
    }

    if ($pkg_lib_dirs.Length -gt 0) {
        $($pkg_lib_dirs | % { "$strippedPrefix\$_" }) -join ';' |
            Out-File "$pkg_prefix\LIB_DIRS" -Encoding ascii
     }

    if ($pkg_include_dirs.Length -gt 0) {
        $($pkg_include_dirs | % { "$strippedPrefix\$_" }) -join ';' |
            Out-File "$pkg_prefix\INCLUDE_DIRS" -Encoding ascii
     }

    if ($pkg_expose.Length -gt 0) {
        "$($pkg_expose -join ' ')" |
            Out-File "$pkg_prefix\EXPOSES" -Encoding ascii
    }

    if ($pkg_exports) {
        foreach ($export in $pkg_exports.GetEnumerator()) {
            "$($export.Key)=$($export.Value)" | Out-File "$pkg_prefix\EXPORTS" -Encoding ascii -Append
        }
    }

    if ($pkg_binds) {
        foreach ($bind in $pkg_binds.GetEnumerator()) {
            "$($bind.Key)=$($bind.Value)" | Out-File "$pkg_prefix\BINDS" -Encoding ascii -Append
        }
    }

    if ($pkg_binds_optional) {
        foreach ($bind in $pkg_binds_optional.GetEnumerator()) {
            "$($bind.Key)=$($bind.Value)" | Out-File "$pkg_prefix\BINDS_OPTIONAL" -Encoding ascii -Append
        }
    }

    $runtime_path = _Assemble-RuntimePath
    if ($runtime_path) {
      "$runtime_path" | Out-File "$pkg_prefix\RUNTIME_PATH" -Encoding ascii

      # Backwards Compatibility: Set the `PATH` key for the runtime environment
      # if a computed runtime path is necessary which will be used by Habitat
      # releases between 0.53.0 (released 2018-02-05) and up to including
      # 0.55.0 (released 2018-03-20). All future releases will ignore the
      # `PATH` entry in favor of using the `RUNTIME_PATH` metadata file.
      $env["RunTime"]["PATH"] = "$runtime_path"
    }

    if ($env.Runtime.length -gt 0) {
        foreach ($var in $env.Runtime.GetEnumerator()) {
            "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\RUNTIME_ENVIRONMENT" -Encoding ascii -Append
        }
    }

    if ($provenance.Runtime.length -gt 0) {
        foreach ($var in $provenance.Runtime.GetEnumerator()) {
            "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\RUNTIME_ENVIRONMENT_PROVENANCE" -Encoding ascii -Append
        }
    }

    if ($env.Buildtime.length -gt 0) {
        foreach ($var in $env.Buildtime.GetEnumerator()) {
            "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\BUILDTIME_ENVIRONMENT" -Encoding ascii -Append
        }
    }

    if ($provenance.Buildtime.length -gt 0) {
        foreach ($var in $provenance.Buildtime.GetEnumerator()) {
            "$($var.Key)=$($var.Value)" | Out-File "$pkg_prefix\BUILDTIME_ENVIRONMENT_PROVENANCE" -Encoding ascii -Append
        }
    }

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

    & "$_7z_cmd" a -ttar "$tarf" $tempBase | Out-Null
    & "$_7z_cmd" a -txz "$xzf" "$tarf" | Out-Null
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

    # At this point, we know it built successfully, so delete the pre_build file
    $preBuild = "$pkg_output_path\pre_build.env"
    if (Test-Path $preBuild) { Remove-Item $preBuild -Force }

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

# If the value of `$pkg_target` is a replacement token, then the program is
# being run out of a raw source tree (which must be supported), otherwise
# `$pkg_target` would have a static value set when the Habitat package for this
# program was built.
if ($pkg_target -eq "@@pkg_target@@") {
  if ($env:BUILD_PKG_TARGET) {
    # If a build environment variable is set with the desired package target,
    # then update the value of `$pkg_target`. This case is used in
    # bootstrapping the Habitat packaging system.
    $script:pkg_target = "$env:BUILD_PKG_TARGET"
    Write-BuildLine "Setting pkg_target='$pkg_target' from `$env:BUILD_PKG_TARGET"
  } else {
    # Otherwise, set a suitable value for `$pkg_target`. This is prior behavior
    # and is backwards compatible and behavior-preserving.
    $script:pkg_target = "x86_64-windows"
    Write-BuildLine "Setting pkg_target='$pkg_target' as fallback default"
  }
}

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
    $places = "$PLAN_CONTEXT\plan.ps1 and $PLAN_CONTEXT\habitat\plan.ps1"
    _Exit-With "A Plan file was found at $places. Only one is allowed at a time" 42
}
# We check if the provided path has a `plan.ps1` in it in either location. If
# not, we'll quickly bail.
if (-Not (Test-Path "$PLAN_CONTEXT\plan.ps1")) {
    if (Test-Path "$PLAN_CONTEXT\habitat\plan.ps1") {
        $PLAN_CONTEXT = "$PLAN_CONTEXT\habitat"
    } else {
        $places = "$PLAN_CONTEXT\plan.ps1 or $PLAN_CONTEXT\habitat\plan.ps1"
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

    foreach ($var in @("pkg_origin", "pkg_name")) {
        if (-Not (Test-Path variable:script:$var)) {
            _Exit-With "Failed to build. '$var' must be set." 1
        } elseif ((Get-Variable $var -Scope script).Value -eq "") {
            _Exit-With "Failed to build. '$var' must be set and non-empty." 1
        }
    }

    # Test to ensure package name and origin contain only valid characters
    foreach ($var in @("pkg_name", "pkg_origin")) {
      if (-Not ((Get-Content Variable:\$var) -match '^[A-Za-z0-9_-]+$')) {
          _Exit-With "Failed to build. Package '$var' contains invalid characters." 1
      }
    }

    # Pass over `$pkg_svc_run` to replace any `$pkg_name` placeholder tokens
    # from prior pkg_svc_* variables that were set before the Plan was loaded.
    if ("$pkg_svc_run" -ne "") {
        $pkg_svc_run = "$pkg_svc_run".Replace("@__pkg_name__@", "$pkg_name")
    }

    # Ensure that the version is set (or can be set!) properly
    if("$pkg_version" -eq "" -and (test-path function:\pkg_version)) {
        $pkg_version="__pkg__version__unset__"
    } elseif("$pkg_version" -eq "") {
        $e="Failed to build. 'pkg_version' must be set or 'pkg_version' function"
        e="$e must be implemented and then invoking by calling 'Set-PkgVersion'."
        _exit-with $e 1
    }

    # Set `$pkg_filename` to the basename of `$pkg_source`, if it is not already
    # set by the `plan.ps1`.
    if ("$pkg_filename" -eq "" -and "$pkg_source" -ne "") {
        $script:pkg_filename = "$(Split-Path $pkg_source -Leaf)"
    }

    # Set `$pkg_dirname` to the `$pkg_name` and `$pkg_version`, if it is not
    # already set by the `plan.ps1`.
    if ("$pkg_dirname" -eq "") {
        $script:pkg_dirname = "${pkg_name}-${pkg_version}"
        $script:_pkg_dirname_initially_unset = $true
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
    # Create initial package arrays
    _init-Dependencies

    # Inject, download, and resolve the scaffolding dependencies
    _Resolve-ScaffoldingDependencies

    # Populate package arrays to enable helper functions for early scaffolding
    # load hooks
    _Set_DependencyArrays

    # Load scaffolding packages if they are being used.
    if ($pkg_scaffolding) {
        $scaff = $pkg_scaffolding.Split("/")[-1]
        $lib="$(Get-HabPackagePath $scaff)/lib/scaffolding.ps1"
        Write-BuildLine "Loading Scaffolding $lib"
        if(!(Test-Path $lib)) {
            _Exit-With "Failed to load Scaffolding from $lib" 17
        }

        . $lib

        if(Test-Path function:\Load-Scaffolding) {
            Load-Scaffolding
        }
    }

    # Download and resolve the build dependencies
    _Resolve-BuildDependencies

    # Download and resolve the run dependencies
    _Resolve-RunDependencies

    # Finalize and normalize all resolved dependencies with all build and run
    # dependencies
    _Set_DependencyArrays

    _Assert-Deps

    # Write out a prebuild file so workers can have some metadata about failed builds
    _Write-Pre-Build-File

    Invoke-SetupEnvironmentWrapper

    _Set-BuildPath

    New-Item "$HAB_CACHE_SRC_PATH" -ItemType Directory -Force | Out-Null

    # Run any code after the environment is set but before the build starts
    Invoke-Before

    # Download the source
    Invoke-Download

    # Verify the source
    Invoke-Verify

    # Clean the cache
    Invoke-Clean

    # Unpack the source
    Invoke-Unpack

    # Set up the build environment
    _Set-Environment

    _Assert-Vars

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
    $env:path = $INITIAL_PATH
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
