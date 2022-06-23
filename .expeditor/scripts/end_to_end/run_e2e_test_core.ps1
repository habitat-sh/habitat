# This script expects `setup_environment.sh` or `setup_environment.ps1` to be sourced before execution
[Diagnostics.CodeAnalysis.SuppressMessage("PSUseApprovedVerbs", '', Scope="function", Target="*Load-SupervisorService")]
param (
    [string]$TestName
)

. $PSScriptroot/../shared.ps1

$ErrorActionPreference = "stop"

if (!$IsCoreCLR) {
    Write-Error "End-to-end tests are only compatiable with Powershell Core"
}

###################################################################################################

Function Get-EndToEndTestingOrigin { "habitat-testing" }

if ($IsLinux -Or $IsMacOS) {
    $env:SystemDrive = "/"
}

# It may take a while to download a given service package, and
# services may have long init scripts. Let's be generous in how long
# we're willing to wait. Also note that tests may take considerably
# longer to complete on buildkite than they do locally.
$DefaultServiceTimeout = 90

function Wait-True([ScriptBlock]$TestScript, [ScriptBlock]$TimeoutScript, [int]$Timeout) {
    $startTime = Get-Date
    while (!$TestScript.Invoke()) {
        $timeTaken = [DateTime]::Now.Subtract($startTime)
        if ($timeTaken.TotalSeconds -ge $Timeout) {
            $TimeoutScript.Invoke()
            break
        }
        Start-Sleep -Seconds 1
    }
}

function Wait-PathUpdatedAfter($Path, $Time, $Timeout) {
    $testScript = {
        (Test-Path -Path $Path) -And ((Get-Item -Path $Path).LastWriteTime -gt $Time)
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for '$Path' to be updated after '$($Time.ToString("yyyy-MM-ddTHH:MM:ssZ"))'" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-PathIncludesContent($Path, $Content, $Timeout = ($DefaultServiceTimeout)) {
    $testScript = {
        (Test-Path -Path $Path) -And ((Get-Content -Path $Path | Out-String).Contains($Content))
    }
    $timeoutScript = {
        if(Test-Path -Path $Path) {
            $got = "'$(Get-Content -Path $Path | Out-String)'"
        } else {
            $got = "...oh...actually...the file doesn't even exist"
        }
        Write-Error "Timed out waiting $Timeout seconds for '$Path' to include content '$Content' but got $got"
    }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-PathHasContent($Path, $Time, $Timeout) {
    $testScript = {
        (Test-Path -Path $Path) -And ((Get-Content -Path $Path).Length -gt 0)
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for '$Path' to have content after '$($Time.ToString("yyyy-MM-ddTHH:MM:ssZ"))'" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-PathHasContentUpdatedAfter($Path, $Time, $Timeout) {
    Wait-PathHasContent @PSBoundParameters
    Wait-PathUpdatedAfter @PSBoundParameters
}

function Wait-Supervisor($Timeout = 1, $port = 9631) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to start..."
    $testScript = { Test-Connection -ComputerName 127.0.0.1 -TcpPort $port }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to start on port $port" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "Supervisor is now running."
}

function Wait-StopSupervisor($Timeout = 10, $port = 9631) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to stop..."
    $testScript = { -Not (Test-Connection -ComputerName 127.0.0.1 -TcpPort $port) }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to stop on port $port" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "Supervisor is now stopped."
}

# Generate a new unique log file name, given a base name.
#
# The base name should be something descriptive that allows you to
# trace a given log file back to the test case that generated it.
#
# This is used instead of the standard `New-TemporaryFile`, since that
# does not seem to allow you to specify any non-random components,
# which we will need for both correlating outputs to tests, as well as
# easily collecting log outputs in Buildkite.
#
# Example:
#
#    > New-SupervisorLogFile "monkeypants"
#    monkeypants-oqUwJ.log
#
function New-SupervisorLogFile($BaseName) {
    Join-String -OutputPrefix "$BaseName-" -OutputSuffix ".log" -InputObject (-join ((65..90) + (97..122) | Get-Random -Count 5 | ForEach-Object {[char]$_}))
}

function Start-Supervisor($LogFile, $Timeout = 1, $SupArgs = @()) {
    Add-Type -TypeDefinition (Get-Content "$PSScriptroot/SupervisorRunner.cs" | Out-String)
    $sup = New-Object SupervisorRunner
    $supPid = $sup.Run($LogFile, $SupArgs)
    try {
        Wait-Supervisor -Timeout $Timeout
        $supPid
    } catch {
        if(!$supPid.HasExited) { $supPid.Kill() }
        throw $_
    }
}

function Wait-SupervisorService($ServiceName, $Timeout = ($DefaultServiceTimeout), $Remote, $Org) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to start $ServiceName ..."
    if(!$Remote) { $Remote = "localhost" }
    $testScript = {
        try {
            if ($Org) {
                $url = "http://${Remote}:9631/services/$ServiceName/default/$Org"
            } else {
                $url = "http://${Remote}:9631/services/$ServiceName/default"
            }
            $status = (Invoke-WebRequest "$url" | ConvertFrom-Json).process.state
            $status -eq "up"
        } catch { $false } # We ignore 404s and other unsuccesful codes
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to start $ServiceName" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "$ServiceName is now up."
}

function Load-SupervisorService($PackageName, $Timeout = ($DefaultServiceTimeout), [switch]$Force, $Remote, $Bind, $Channel, $Topology, $Strategy, $UpdateCondition, $HealthCheckInterval, $Org) {
    $svcName = ($PackageName -split "/")[1]
    $commandArgs = @("hab", "svc", "load", $PackageName)
    if($Force) {
        $commandArgs += @("--force")
    }
    if($Remote) {
        $commandArgs += @("--remote-sup", $Remote)
    }
    if($Bind) {
        $commandArgs += @("--bind", $Bind)
    }
    if($Channel) {
        $commandArgs += @("--channel", $Channel)
    }
    if($Topology) {
        $commandArgs += @("--topology", $Topology)
    }
    if($Strategy) {
        $commandArgs += @("--strategy", $Strategy)
    }
    if($UpdateCondition) {
        $commandArgs += @("--update-condition", $UpdateCondition)
    }
    if($HealthCheckInterval) {
        $commandArgs += @("--health-check-interval", $HealthCheckInterval)
    }
    Invoke-NativeCommand @commandArgs | Out-Null
    Wait-SupervisorService $svcName -Timeout $Timeout -Remote $Remote $Org
    $svcName
}

function Wait-SupervisorServiceUnload($ServiceName, $Timeout = 1) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to unload $ServiceName ..."
    $testScript = {
        try {
            Invoke-WebRequest "http://localhost:9631/services/$ServiceName/default" | Out-Null
            $false
        } catch {
            $statusCode = $_.Exception.Response.StatusCode.value__
            $statusCode -eq 404
        }
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to unload $ServiceName" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "$ServiceName is now unloaded."
}

function Unload-SupervisorService($PackageName, $Timeout = 5) {
    $svcName = ($PackageName -split "/")[1]
    Invoke-NativeCommand hab svc unload $PackageName | Out-Null
    Wait-SupervisorServiceUnload $svcName -Timeout $Timeout
}

function Invoke-StudioRun {
    if($env:DOCKER_STUDIO_TEST) {
        hab studio run -D @args
    } else {
        hab studio run @args
    }
}

function New-TemporaryDirectory {
    $parent = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

function Restart-Supervisor {
    if ($IsLinux) {
        pkill --signal=HUP hab-launch
        Start-Sleep 3 # wait for the signal to be processed
    } else {
        Stop-Process | Get-Process hab-sup
    }
    Wait-Supervisor -Timeout 5 # 5 seconds should be plenty of time
}

function Stop-Supervisor {
    Stop-Process -ErrorAction 'silentlycontinue' -Name hab-launch
    Wait-StopSupervisor
}

function Wait-ProcessStart($ProcessName, $Timeout = 1) {
    $testScript =  { Get-Process $ProcessName* -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $ProcessName to start" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-ProcessExit($Process, $Timeout = 1) {
    $testScript =  { $Process.HasExited }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $Process.Name to exit" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-Release($Ident, $Remote, $Timeout = ($DefaultServiceTimeout)) {
    $serviceName = ($Ident.Split("/"))[1]
    $testScript =  {
        if ($Remote) {
            $Remote = "${Remote}.habitat.dev"
        } else {
            $Remote = "localhost"
        }
        $currentIdent = (Invoke-WebRequest "http://${Remote}:9631/services/$serviceName/default" | ConvertFrom-Json).pkg.ident
        $currentIdent -eq $Ident
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $Remote to Update to $Release" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-CommandLinesOfOutput($Cmd, $Lines, $Timeout = 20) {
    $testScript =  { (Invoke-Expression $Cmd | Measure-Object -Line).Lines -eq $Lines }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $Cmd to output exactly $Lines lines" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Get-Leader($Remote, $ServiceGroup) {
    $json = (Invoke-WebRequest "http://$Remote.habitat.dev:9631/census" | ConvertFrom-Json)
    $id = $json.census_groups.$ServiceGroup.leader_id
    $name = $json.census_groups.$ServiceGroup.population.$id.sys.hostname
    @{
        Id   = $id;
        Name = $name
    }
}

function Invoke-Build($PackageName) {
    hab pkg build test/fixtures/$PackageName --reuse
    if ($IsLinux) {
        # This changes the format of last_build from `var=value` to `$var='value'`
        # so that powershell can parse and source the script
        Set-Content -Path "results/last_build.ps1" -Value ""
        Get-Content "results/last_build.env" | ForEach-Object { Add-Content "results/last_build.ps1" -Value "`$$($_.Replace("=", '="'))`"" }
    }
}

Function Invoke-BuildAndInstall($PackageName) {
    Invoke-Build $PackageName
    . ./results/last_build.ps1
    hab pkg install ./results/$pkg_artifact
}

function Stop-ComposeSupervisor($Remote) {
    Invoke-NativeCommand docker exec "${env:COMPOSE_PROJECT_NAME}_${Remote}_1" hab sup term
    Start-Sleep 5
}

# Returns the PID of the given service, according to the Supervisor itself.
#
# Example:
#    Get-HabServicePid "core/redis"
#    > 29602
#
# Fails if the Supervisor is not running the service.
function Get-HabServicePID($PackageName) {
    # If the package is running, the output of `hab svc status` will look like this:
    #
    # package                           type        desired  state  elapsed (s)  pid    group
    # core/redis/4.0.14/20190319155852  standalone  up       up     7717         29602  redis.default
    #
    # We take the last line, and then extract the value from the "pid" column.
    #
    # (When https://github.com/habitat-sh/habitat/issues/7525 lands,
    # we can do this in a more self-documenting way.)
    (((hab svc status $PackageName)[-1] -split "\s+")[5])
}


###################################################################################################

Import-Module (Join-Path -Path $(hab pkg path core/pester) module Pester.psd1)

if(Test-Path $TestName) {
    $testPath = $TestName
} else {
    $testPath = Join-Path -Path test end-to-end "$($TestName).ps1"
}
Invoke-Pester $testPath -EnableExit
