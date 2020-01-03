# A simple test that the launcher doesn't hang if the IPC connection to the
# supervisor doesn't complete in a timely manner. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

$env:HAB_LAUNCH_SUP_CONNECT_TIMEOUT_SECS=2
$env:HAB_FEAT_BOOT_FAIL=1
$env:HAB_LAUNCH_NO_SUP_VERSION_CHECK="true"

Add-Type -TypeDefinition (Get-Content "$PSScriptroot/../../.expeditor/scripts/end_to_end/SupervisorRunner.cs" | Out-String)

$sup_log = "sup.log"

# Preinstall these packages. If we don't, then we spend the bulk of
# our time in the following `while` loop downloading them, rather than
# actually exercising the functionality we're after. That leads to
# spurious failures, depending on how long the downloading takes.
#
# Doing things this way, we eliminate that concern.
hab pkg install core/hab-sup --channel="${env:HAB_BLDR_CHANNEL}"
hab pkg install core/hab-launcher --channel="${env:HAB_BLDR_CHANNEL}"

Describe "Supervisor boot failure" {
    $sup = New-Object SupervisorRunner
    $supPid = $sup.Run($sup_log)
    
    It "exits launcher before timeout" {
        $retries=0
        $max_retries=5
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
    It "should exit with failure" {
        $supPid.ExitCode | Should -Not -Be 0
    }
    It "logs connection failure" {
        $sup_log | Should -FileContentMatch "Unable to accept connection from Supervisor"
    }
}
