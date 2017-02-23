# # Usage
#
# ```powershell
# $ hab-studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ...]
# ```
#
# See the `Write-Help` function below for complete usage instructions.
#
# # Synopsis
#
# blah
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2016 Chef Software, Inc.
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

param (
    [switch]$h,
    [switch]$n,
    [switch]$q,
    [switch]$v,
    [switch]$R,
    [string]$command,
    [string]$commandVal,
    [string]$k,
    [string]$o,
    [string]$s
)

# # Internals

# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
function Write-Help {
  Write-Host @"
$program $version

$author

Habitat Studios - Plan for success!

USAGE:
        $program [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

COMMON FLAGS:
    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output

COMMON OPTIONS:
    -k <HAB_ORIGIN_KEYS>  Installs secret origin keys (default:\$HAB_ORIGIN )
    -o <HAB_STUDIO_ROOT>  Sets a Studio root (default: /hab/studios/<DIR_NAME>)
    -s <SRC_PATH>         Sets the source path (default: \$PWD)

SUBCOMMANDS:
    build     Build using a Studio
    enter     Interactively enter a Studio
    help      Prints this message
    new       Creates a new Studio
    rm        Destroys a Studio
    run       Run a command in a Studio
    version   Prints version information

ENVIRONMENT VARIABLES:
    HAB_ORIGIN        Propagates this variable into any studios
    HAB_ORIGIN_KEYS   Installs secret keys (\`-k' option overrides)
    HAB_STUDIOS_HOME  Sets a home path for all Studios (default: /hab/studios)
    HAB_STUDIO_ROOT   Sets a Studio root (\`-r' option overrides)
    NO_SRC_PATH       If set, do not mount source path (\`-n' flag overrides)
    QUIET             Prints less output (\`-q' flag overrides)
    SRC_PATH          Sets the source path (\`-s' option overrides)
    VERBOSE           Prints more verbose output (\`-v' flag overrides)

SUBCOMMAND HELP:
    $program <SUBCOMMAND> -h

EXAMPLES:

    # Create a new default Studio
    $program new

    # Enter the default Studio
    $program enter

    # Run a command in the default Studio
    $program run hab --version

    # Destroy the default Studio
    $program rm

    # Create and enter a Studio with a custom root
    $program -o /opt/slim

    # Run a command in the slim Studio, showing only the command output
    $program -q -o /opt/slim run busybox ls -l /

    # Verbosely destroy the slim Studio
    $program -v -o /opt/slim rm

"@
}

function Write-BuildHelp {
  Write-Host @"
${program}-build $version

$author

Habitat Studios - execute a build using a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] build [FLAGS] [PLAN_DIR]

FLAGS:
    -R  Reuse a previous Studio state (default: clean up before building)

EXAMPLES:

    # Build a Redis plan
    $program build plans/redis

    # Reuse previous Studio for a build
    $program build -R plans/glibc

"@
}

function Write-EnterHelp {
  Write-Host @"
${program}-enter $version

$author

Habitat Studios - interactively enter a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] enter

"@
}

function Write-NewHelp {
  Write-Host @"
${program}-new $version

$author

Habitat Studios - create a new Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] new

"@
}

function Write-RmHelp {
  Write-Host @"
${program}-rm $version

$author

Habitat Studios - destroy a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] rm

"@
}

function Write-RunHelp {
  Write-Host @"
${program}-run $version

$author

Habitat Studios - run a command in a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] run [CMD] [ARG ..]

CMD:
    Command to run in the Studio

ARG:
    Arguments to the command

EXAMPLES:

    $program run wget --version

"@
}

function Write-HabInfo($Message) {
  if($quiet) { return }
  Write-Host "   ${program}: " -ForegroundColor Cyan -NoNewline
  Write-Host $Message
}

# ## Subcommand functions
#
# These are the implementations for each subcommand in the program.

function New-Studio {
  if($printHelp) {
    Write-NewHelp
    return
  }
  Write-HabInfo "Creating Studio at $HAB_STUDIO_ROOT"

  if(!(Test-Path $HAB_STUDIO_ROOT)) {
    mkdir $HAB_STUDIO_ROOT | Out-Null
  }

  Set-Location $HAB_STUDIO_ROOT
  if(!(Test-Path src) -and !($doNotMount)) {
    mkdir src | Out-Null
    New-Item -Name src -ItemType Junction -target $SRC_PATH.Path | Out-Null
  }

  $pathArray = @(
    "$PSScriptRoot\hab",
    "$PSScriptRoot\7zip",
    "$PSScriptRoot",
    "$env:WINDIR\system32",
    "$env:WINDIR"
  )

  $env:PATH = [String]::Join(";", $pathArray)

  if($env:HAB_ORIGIN_KEYS) {
    $keys = @()
    $env:HAB_ORIGIN_KEYS.Split(" ") | % {
      $keys += & hab origin key export $_ --type=secret | Out-String
    }

    $env:FS_ROOT=$HAB_STUDIO_ROOT
    $keys | % { $_ | & hab origin key import }
  }
  else {
    $env:FS_ROOT=$HAB_STUDIO_ROOT
  }

  New-PSDrive -Name "Habitat" -PSProvider FileSystem -Root $HAB_STUDIO_ROOT -Scope Script | Out-Null
  Set-Location "Habitat:\src"
}

function Enter-Studio {
  if($printHelp) {
    Write-EnterHelp
    return
  }
  New-Studio
  Write-HabInfo "Entering Studio at $HAB_STUDIO_ROOT"
  $env:HAB_STUDIO_ENTER_ROOT = $HAB_STUDIO_ROOT
  $env:STUDIO_SCRIPT_ROOT = $PSScriptRoot
  & "$PSScriptRoot\powershell\powershell.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -NoExit -Command {
    function prompt {
      Write-Host "[HAB-STUDIO]" -NoNewLine -ForegroundColor Green
      " $($executionContext.SessionState.Path.CurrentLocation)$('>' * ($nestedPromptLevel +1)) "
    }
    function build {
      & "$env:STUDIO_SCRIPT_ROOT\hab-plan-build.ps1" @args
    }
    # This captures 'hab start' commands and launches them in a new window
    # We do this because breaking out of a service via ctrl-C breaks
    # nested shells. Any other hab command should simply be passed through
    function hab {
      if($args.length -gt 1 -and ($args[0] -eq "start")) {
        Start-Process hab.exe -ArgumentList $args
      }
      else {
        & hab.exe @args
      }
    }
    New-PSDrive -Name "Habitat" -PSProvider FileSystem -Root $env:HAB_STUDIO_ENTER_ROOT | Out-Null
    Set-Location "Habitat:\src"
  }
}

function Invoke-StudioRun($cmd) {
  if($printHelp -or ($cmd -eq $null)) {
    Write-RunHelp
    return
  }
  New-Studio
  Write-HabInfo "Running '$cmd' in Studio at $HAB_STUDIO_ROOT"
  New-PSDrive -Name "Habitat" -PSProvider FileSystem -Root $HAB_STUDIO_ROOT | Out-Null
  Set-Location "Habitat:\src"
  Invoke-Expression $cmd
}

function Invoke-StudioBuild($location, $reuse) {
  if($printHelp -or ($location -eq $null)) {
    Write-BuildHelp
    return
  }
  if(!$reuse) { Remove-Studio}

  New-Studio
  Write-HabInfo "Building '$location' in Studio at $HAB_STUDIO_ROOT"

  & "$PSScriptRoot\hab-plan-build.ps1" $location
}

function Remove-Studio {
  if($printHelp) {
    Write-RmHelp
    return
  }
  Write-HabInfo "Destroying Studio at $HAB_STUDIO_ROOT"

  if(Test-Path $HAB_STUDIO_ROOT) { Remove-Item $HAB_STUDIO_ROOT -Recurse -Force }
}

# The current version of Habitat Studio
$script:version='@version@'
# The author of this program
$script:author='@author@'
# The short version of the program name which is used in logging output
$script:program="hab-studio"

if($env:SRC_PATH) {
  $script:SRC_PATH = Resolve-Path $env:SRC_PATH
}
else {
  $script:SRC_PATH = Get-Location
}
if($s) { $script:SRC_PATH = Resolve-Path $s }
$script:dir_name = $SRC_PATH.Path.Replace("$($SRC_PATH.Drive):\","").Replace("\","--")

if(!$env:HAB_STUDIOS_HOME) {
  $script:HAB_STUDIOS_HOME = "/hab/studios"
}
else {
  $script:HAB_STUDIOS_HOME = $env:HAB_STUDIOS_HOME
}

if(!$env:HAB_STUDIO_ROOT) {
  $script:HAB_STUDIO_ROOT = "$HAB_STUDIOS_HOME/$dir_name"
}
else {
  $script:HAB_STUDIO_ROOT = $env:HAB_STUDIO_ROOT
}
if($o) { $script:HAB_STUDIO_ROOT = $o }

if($k) {
  $env:HAB_ORIGIN_KEYS = $k
}
else {
  $env:HAB_ORIGIN_KEYS = $env:HAB_ORIGIN
}

if($h) { $script:printHelp = $true }
if($n) { $script:doNotMount = $true }
if($q) { $script:quiet = $true }

$currentVerbose = $VerbosePreference
if($v) { $VerbosePreference = "Continue" }

try {
  switch ($command) {
    "new" { New-Studio }
    "run" { Invoke-StudioRun $commandVal }
    "rm" { Remove-Studio }
    "enter" { Enter-Studio }
    "build" { Invoke-StudioBuild $commandVal $R }
    "version" { Write-Host "$program $version" }
    "help" { Write-Help }
    default {
      Write-Help
      Write-Error "Invalid Argument $command"
    }
  }
}
finally { $VerbosePreference = $currentVerbose }
