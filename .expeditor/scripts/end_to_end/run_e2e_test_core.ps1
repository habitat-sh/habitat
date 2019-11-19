# This script expects `setup_environment.sh` or `setup_environment.ps1` to be sourced before execution

param (
    [string]$TestName,
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)

. .expeditor/scripts/shared.ps1

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

function Start-Supervisor($Timeout = 1) {
    hab sup run --no-color &
    Wait-Supervisor -Timeout $Timeout
}

function Wait-SupervisorService($ServiceName, $Timeout = 1) {
    Write-Host "Waiting up to $Timeout seconds for Supervisor to start $ServiceName ..."
    $testScript = { 
        try {
            $status = (Invoke-WebRequest "http://localhost:9631/services/$ServiceName/default" |
                ConvertFrom-Json).process.state
            $status -eq "up"
        }
        catch { } # We ignore 404s and other unsuccesful codes
    }
    $timeoutScript = { Write-Error "Timed out waiting $Timeout seconds for Supervisor to start $ServiceName" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout $Timeout
    Write-Host "$ServiceName is now up."
}
  
function Load-SupervisorService($PackageName, $Timeout = 1) {
    $svcName = ($PackageName -split "/")[1]
    $_ = Invoke-NativeCommand hab svc load $PackageName --url="$BuilderUrl"
    Wait-SupervisorService $svcName -Timeout $Timeout
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

###################################################################################################

Write-Host "--- Installing latest core/pester from $env:HAB_BLDR_URL, stable channel"
Invoke-NativeCommand hab pkg install core/pester `
    --channel=stable `
    --url="$BuilderUrl"
$pesterPath = Join-Path $(hab pkg path core/pester) module Pester.psd1
Import-Module $pesterPath
Write-Host "---"

$testPath = Join-Path test end-to-end "$($TestName).ps1"
Invoke-Pester $testPath -EnableExit
