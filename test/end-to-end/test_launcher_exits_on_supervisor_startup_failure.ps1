# A simple test that the launcher doesn't go into a tight loop restarting the
# supervisor if the supervisor fails to start up. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

Add-Type -TypeDefinition (Get-Content "$PSScriptroot/../../.expeditor/scripts/end_to_end/SupervisorRunner.cs" | Out-String)

$env:TESTING_FS_ROOT = (Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName()))
New-Item $env:TESTING_FS_ROOT -ItemType Directory -Force

# Installing the launcher here because TESTING_FS_ROOT is an imperfect
# abstraction that needs to be removed. It turns out that even if
# TESTING_FS_ROOT is in place, we still look for the launcher in
# `/hab` when we start up.
#
# Once we remove TESTING_FS_ROOT completely, we'll need to rethink how
# this test works, since we can't really make `/` read-only
hab pkg install core/hab-launcher

Describe "Supervisor startup failure" {
	chmod -R a-w $env:TESTING_FS_ROOT
    $sup = New-Object SupervisorRunner
    $supPid = $sup.Run("sup.log")

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
