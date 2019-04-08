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

param (
    [switch]$h,
    [switch]$n,
    [switch]$q,
    [switch]$v,
    [switch]$R,
    [switch]$D,
    [switch]$w,
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
    -D  Use a Docker Studio instead of a native Studio
DEPRECATED FLAG:
    -w  Use a local Windows studio instead of a Docker Studio

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
    HAB_LICENSE       Set to 'accept' or 'accept-no-persist' to accept the Habitat license
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
    "$PSScriptRoot\powershell",
    "$PSScriptRoot\hab",
    "$PSScriptRoot\7zip",
    "$PSScriptRoot",
    "$env:WINDIR\system32",
    "$env:WINDIR",
    (Join-Path $HAB_STUDIO_ROOT "hab\bin")
  )

  $env:PATH = [String]::Join(";", $pathArray)

  if($env:HAB_ORIGIN_KEYS) {
    $secret_keys = @()
    $public_keys = @()
    $env:HAB_ORIGIN_KEYS.Split(" ") | % {
      $pk = & hab origin key export $_ --type=public | Out-String
      $sk = & hab origin key export $_ --type=secret | Out-String
      # hab key import does not like carriage returns
      $secret_keys += $sk.Replace("`r", "")
      $public_keys += $pk.Replace("`r", "")
    }

    $env:FS_ROOT=$HAB_STUDIO_ROOT
    $env:HAB_CACHE_KEY_PATH = Join-Path $env:FS_ROOT "hab\cache\keys"
    $public_keys | % { $_ | & hab origin key import }
    $secret_keys | % { $_ | & hab origin key import }
  }
  else {
    $env:FS_ROOT=$HAB_STUDIO_ROOT
    $env:HAB_CACHE_KEY_PATH = Join-Path $env:FS_ROOT "hab\cache\keys"
  }


  Set-Secrets

  New-PSDrive -Name "Habitat" -PSProvider FileSystem -Root $HAB_STUDIO_ROOT -Scope Script | Out-Null
  Set-Location "Habitat:\src"
}

function Enter-Studio {
  if($printHelp) {
    Write-EnterHelp
    return
  }
  if(!(Test-Path $HAB_STUDIO_ROOT)) {
    mkdir $HAB_STUDIO_ROOT | Out-Null
  }
  $env:HAB_STUDIO_ENTER_ROOT = Resolve-Path $HAB_STUDIO_ROOT
  if (Test-InContainer) {
    # The Windows Docker TTY does not render non standard
    # characters. Each is rendered as a '?'. So we are going
    # to just render standard ascii symbols. No pretty clouds
    # or check marks.
    $env:HAB_GLYPH_STYLE="ascii"
  }
  New-Studio
  $env:STUDIO_SCRIPT_ROOT = $PSScriptRoot
  $shouldStartStudio = $false

  if(!(Test-InContainer) -and (Get-Process -Name hab-sup -ErrorAction SilentlyContinue)) {
    Write-Warning "A Habitat Supervisor is already running on this machine."
    Write-Warning "Only one Supervisor can run at a time."
    Write-Warning "A Supervisor will not be started in this Studio."
  } else {
    $habSvc = Get-Service Habitat -ErrorAction SilentlyContinue
    if(!$habSvc -or ($habSvc.Status -eq "Stopped")) {
      $shouldStartStudio = $true

      # Set console encoding to UTF-8 so that any redirected glyphs
      # from the supervisor log are propperly encoded
      [System.Console]::OutputEncoding = [System.Text.Encoding]::UTF8

      # We start the Supervisor and handle its output logging in C# which will handle
      # the process's OutputDataReceived and ErrorDataReceived events in a separate thread.
      # While we could use PowerShell's Register-ObjectEvent instead. That uses a PSJob
      # which will be blocked while the interactive studio is open
      Add-Type -TypeDefinition (Get-Content "$PSScriptRoot\SupervisorBootstrapper.cs" | Out-String)

      # If the termcolor crate cannot find a console, which it will not
      # since we launch the supervisor in the background, it will fall back
      # to ANSI codes on Windows unless we explicitly turn off color. Lets
      # do that if on a windows version that does not support ANSI codes in
      # its console
      $ansi_min_supported_version = [Version]::new(10, 0, 10586)
      $osVersion = [Version]::new((Get-CimInstance -ClassName Win32_OperatingSystem).Version)
      $isAnsiSupported = $false
      if ($osVersion -ge $ansi_min_supported_version) {
        $isAnsiSupported = $true
      }

      mkdir $env:HAB_STUDIO_ENTER_ROOT\hab\sup\default -Force | Out-Null
      [SupervisorBootstrapper]::Run($isAnsiSupported)
    }

    Write-Host  "** The Habitat Supervisor has been started in the background." -ForegroundColor Cyan
    Write-Host  "** Use 'hab svc start' and 'hab svc stop' to start and stop services." -ForegroundColor Cyan
    Write-Host  "** Use the 'Get-SupervisorLog' command to stream the Supervisor log." -ForegroundColor Cyan
    Write-Host  "** Use the 'Stop-Supervisor' to terminate the Supervisor." -ForegroundColor Cyan
    Write-Host  ""
  }
  Write-HabInfo "Entering Studio at $HAB_STUDIO_ROOT"
  & "$PSScriptRoot\powershell\pwsh.exe" -NoProfile -ExecutionPolicy bypass -NoLogo -NoExit -Command {
    function prompt {
      Write-Host "[HAB-STUDIO]" -NoNewLine -ForegroundColor Green
      " $($executionContext.SessionState.Path.CurrentLocation)$('>' * ($nestedPromptLevel +1)) "
    }
    function build {
      & "$env:STUDIO_SCRIPT_ROOT\hab-plan-build.ps1" @args
    }
    function Test-InContainer {
      (Get-Service -Name cexecsvc -ErrorAction SilentlyContinue) -ne $null
    }

    function Get-SupervisorLog {
      # If we are not running in a container then the powershell studio was
      # spawned by hab.exe and ctrl+c behaves poorly in this scenario. So
      # we work around the issue by tailing the log in a new window that can
      # simply be closed. If we are inside a container then we cannot launch
      # new windows but thats ok because docker run launched the studio directly
      # from powershell so the ctrl+c issue is not a problem so we can do
      # a simple tail
      if (!(Test-InContainer)) {
        Start-Process "$env:STUDIO_SCRIPT_ROOT\powershell\pwsh.exe" -ArgumentList "-Command `"& {Get-Content $env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\out.log -Tail 100 -Wait}`""
      }
      else {
        # Container studios run habitat as a service which logs to a
        # configured file location
        $svcPath = Join-Path $env:SystemDrive "hab\svc\windows-service"
        [xml]$configXml = Get-Content (Join-Path $svcPath log4net.xml)
        $logPath = (Resolve-Path $configXml.log4net.appender.file.value).Path

        Get-Content $logPath -Tail 100 -Wait
      }
    }

    function Stop-Supervisor {
      $habSvc = Get-Service Habitat -ErrorAction SilentlyContinue
      if($habSvc -and ($habSvc.Status -eq "Running")) {
        Stop-Service Habitat
      }
      elseif(Test-Path "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK") {
        Stop-Process -Id (Get-Content "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK")
        Remove-Item "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK" -Force -ErrorAction SilentlyContinue
      }
    }

    New-PSDrive -Name "Habitat" -PSProvider FileSystem -Root $env:HAB_STUDIO_ENTER_ROOT | Out-Null
    Set-Location "Habitat:\src"
  }

  if($shouldStartStudio -and (Test-Path "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK")) {
    Stop-Process -Id (Get-Content "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK")
    $retry = 0
    while(($retry -lt 5) -and (Test-Path "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK")) {
      $retry += 1
      Write-Host "Waiting for Supervisor to finish..."
      Start-Sleep -Seconds 5
    }
    Remove-Item "$env:HAB_STUDIO_ENTER_ROOT\hab\sup\default\LOCK" -Force -ErrorAction SilentlyContinue
  }
}

function Invoke-StudioRun($cmd) {
  if($printHelp -or ([String]::IsNullOrEmpty($cmd))) {
    Write-RunHelp
    return
  }
  New-Studio
  Write-HabInfo "Running '$cmd' in Studio at $HAB_STUDIO_ROOT"
  Invoke-Expression $cmd
}

function Invoke-StudioBuild($location, $reuse) {
  # This trap will cause powershell to return an exit code of 1
  trap { "An error occured in the build!" }

  if($printHelp -or ([String]::IsNullOrEmpty($location))) {
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
  if ($HAB_STUDIO_ROOT -eq "$env:SystemDrive\") {
    Write-HabInfo "Studio is rooted in system drive. Skipping Studio removal."
  } else {
    if(Test-Path $HAB_STUDIO_ROOT) {
      Write-HabInfo "Destroying Studio at $HAB_STUDIO_ROOT"
      Remove-Item $HAB_STUDIO_ROOT -Recurse -Force
    }
  }
}

function Test-InContainer {
  (Get-Service -Name cexecsvc -ErrorAction SilentlyContinue) -ne $null
}

function Remove-UnsafeSecrets {
  @('HAB_ORIGIN', 'PATH', 'HAB_LICENSE') | ForEach-Object {
    if(Test-Path "env:\HAB_STUDIO_SECRET_$_") {
      Remove-Item "env:\HAB_STUDIO_SECRET_$_"
    }
  }
}

function Set-Secrets {
  Remove-UnsafeSecrets
  Get-ChildItem env: | ? { $_.Name.StartsWith('HAB_STUDIO_SECRET_') } | % {
    New-Item -Name $_.Name.Replace('HAB_STUDIO_SECRET_', '') -Value $_.Value -Path Env: -Force | Out-Null
    Remove-Item -Path "Env:\$($_.Name)" -Force
  }
}

$ErrorActionPreference="stop"

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
$HAB_STUDIO_ROOT = $ExecutionContext.SessionState.Path.GetUnresolvedProviderPathFromPSPath($HAB_STUDIO_ROOT)

if($k) {
  $env:HAB_ORIGIN_KEYS = $k
}
else {
  $env:HAB_ORIGIN_KEYS = $env:HAB_ORIGIN
}

if ((Test-Path "$env:USERPROFILE\.hab\accepted-licenses\habitat") -or (Test-Path "$env:SYSTEMDRIVE\hab\accepted-licenses\habitat")) {
  $env:HAB_LICENSE = "accept-no-persist"
}

if($h) { $script:printHelp = $true }
if($n) { $script:doNotMount = $true }
if($q) { $script:quiet = $true }

$currentVerbose = $VerbosePreference
if($v) { $VerbosePreference = "Continue" }

if($w) {
  Write-Warning "The -w argument has been deprecated."
  Write-Warning "A local Studio is now the default on Windows."
  Write-Warning "Alternatively you may use the -D argument to use a Docker Studio."
}

if(!$w -and !(Test-InContainer)) {
  Write-Warning "Using a local Studio. To use a Docker studio, use the -D argument."
}

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
