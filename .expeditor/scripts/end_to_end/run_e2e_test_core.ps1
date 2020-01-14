# This script expects `setup_environment.sh` or `setup_environment.ps1` to be sourced before execution

param (
    [string]$TestName,
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)

. $PSScriptroot/../shared.ps1

$ErrorActionPreference = "stop"

if (!$IsCoreCLR) {
    Write-Error "End-to-end tests are only compatiable with Powershell Core"
}

###################################################################################################

$EndToEndTestingOrigin = "habitat-testing"
$TestStartTime = Get-Date
if ($IsLinux -Or $IsMacOS) {
    $env:SystemDrive = "/"
}

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

function Wait-PathHasContent($Path, $Time, $Timeout) {
    $testScript = {
        (Test-Path -Path $Path) -And ((Get-Content -Path $Path).Length -gt 0)
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for '$Path' to have content after '$($Time.ToString("yyyy-MM-ddTHH:MM:ssZ"))'" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-PathHasContentUpdatedAfter($Path, $Time, $Timeout) {
    Wait-PathHasContent $Path $Time $Timeout
    Wait-PathUpdatedAfter $Path $Time $Timeout
}

function Wait-Supervisor($Timeout = 1) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to start..."
    $testScript = { Test-Connection -ComputerName 127.0.0.1 -TCPPort 9631 }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to start" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "Supervisor is now running."
}

function Start-Supervisor($Timeout = 1, $LogFile = (New-TemporaryFile), $SupArgs = @()) {
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

function Wait-SupervisorService($ServiceName, $Timeout = 20, $Remote) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to start $ServiceName ..."
    if(!$Remote) { $Remote = "localhost" }
    $testScript = { 
        try {
            $status = (Invoke-WebRequest "http://${Remote}:9631/services/$ServiceName/default" |
                ConvertFrom-Json).process.state
            $status -eq "up"
        }
        catch { } # We ignore 404s and other unsuccesful codes
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to start $ServiceName" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "$ServiceName is now up."
}
  
function Load-SupervisorService($PackageName, $Timeout = 20, $Remote, $Bind, $Channel, $Topology, $Strategy, $HealthCheckInterval) {
    $svcName = ($PackageName -split "/")[1]
    $commandArgs = @("hab", "svc", "load", $PackageName)
    if($BuilderUrl) {
        $commandArgs += @("--url", $BuilderUrl)
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
    if($HealthCheckInterval) {
        $commandArgs += @("--health-check-interval", $HealthCheckInterval)
    }
    $_ = Invoke-NativeCommand @commandArgs
    Wait-SupervisorService $svcName -Timeout $Timeout -Remote $Remote
    $svcName
}
  
function Wait-SupervisorServiceUnload($ServiceName, $Timeout = 1) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to unload $ServiceName ..."
    $testScript = { 
        try {
            $_ = Invoke-WebRequest "http://localhost:9631/services/$ServiceName/default"
            $false
        }
        catch {
            $statusCode = $_.Exception.Response.StatusCode.value__
            $statusCode -eq 404
        }
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to unload $ServiceName" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "$ServiceName is now unloaded."
}

function Unload-SupervisorService($PackageName, $Timeout = 1) {
    $svcName = ($PackageName -split "/")[1]
    $_ = Invoke-NativeCommand hab svc unload $PackageName
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
    if ($IsLinux) {
        pkill --signal=KILL hab-launch
    } else {
        Stop-Process | Get-Process hab-launch
    }
}

function Wait-Process($ProcessName, $Timeout = 1) {
    $testScript =  { Get-Process $ProcessName -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $ProcessName to start" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Wait-Release($Ident, $Remote, $Timeout = 20) {
    $serviceName = ($Ident.Split("/"))[1]
    $testScript =  {
        $currentIdent = (Invoke-WebRequest "http://${Remote}.habitat.dev:9631/services/$serviceName/default" | ConvertFrom-Json).pkg.ident
        $currentIdent -eq $Ident
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for $Remote to Update to $Release" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
}

function Get-Leader($Remote, $ServiceGroup) {
    $json = (Invoke-WebRequest "http://$Remote.habitat.dev:9631/census" | ConvertFrom-Json)
    $id = $json.census_groups.$ServiceGroup.leader_id
    $name = $json.census_groups.$ServiceGroup.population.$id.sys.hostname
    @{
        Id = $id;
        Name = $name
    }
}

function Stop-ComposeSupervisor($Remote) {
    Invoke-NativeCommand docker exec "${env:COMPOSE_PROJECT_NAME}_${Remote}_1" hab sup term
    Start-Sleep 5
}

###################################################################################################

$pesterPath = Join-Path $(hab pkg path core/pester) module Pester.psd1
Import-Module $pesterPath

if(Test-Path $TestName) {
    $testPath = $TestName
} else {
    $testPath = Join-Path test end-to-end "$($TestName).ps1"
}
Invoke-Pester $testPath -EnableExit
