# A simple test that the launcher doesn't go into a tight loop restarting the
# supervisor if the supervisor fails to start up. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

Add-Type -TypeDefinition (Get-Content "$PSScriptroot/../../.expeditor/scripts/end_to_end/SupervisorRunner.cs" | Out-String)

$env:FS_ROOT = (Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName()))
New-Item $env:FS_ROOT -ItemType Directory -Force

# Installing the launcher here because FS_ROOT is an imperfect abstraction. It turns out that even
# if FS_ROOT is in place, we still look for the launcher in `/hab` when we start up.
hab pkg install chef/hab-launcher

Describe "Supervisor startup failure" {
    chmod -R a-w $env:FS_ROOT
    $sup = New-Object SupervisorRunner
    $supLog = New-SupervisorLogFile("supervisor_startup_failure")
    $supPid = $sup.Run($supLog)

    It "exits launcher before timeout" {
        $retries=0
        $max_retries=50
        $exitFailure = $false
        while(!$supPid.HasExited) {
            if($retries++ -gt $max_retries) {
                $exitFailure = $true
            } else {
                Start-Sleep 1
            }
        }
        $exitFailure | Should -Be $false
    }
}
