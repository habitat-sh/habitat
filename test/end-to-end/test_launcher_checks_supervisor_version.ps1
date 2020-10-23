# A simple test that the launcher correctly checks the version of the
# supervisor binary is compatible.
#
# To override and test locally-built code, set overrides in the environment of
# the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

Add-Type -TypeDefinition (Get-Content "$PSScriptroot/../../.expeditor/scripts/end_to_end/SupervisorRunner.cs" | Out-String)

function New-BinaryStub($version) {
    $binary = New-TemporaryFile

    Set-Content -Path $binary -Value @"
#!/bin/bash
echo "hab-sup $version"
"@

    chmod +x $binary
    $binary
}

function Test-LauncherFailure($version, $testName) {
    $sup_binary = New-BinaryStub $version

    $env:HAB_SUP_BINARY=$sup_binary
    $sup = New-Object SupervisorRunner
    $supLog = New-SupervisorLogFile($testName)
    $supPid = $sup.Run($supLog)

    $retries=0
    $max_retries=3
    while(!$supPid.HasExited) {
        if($retries++ -gt $max_retries) {
            $supPid.Kill()
            return $false
        } else {
            Start-Sleep 1
        }
    }

    return ($supPid.ExitCode -ne 0)
}

# These tests will timeout unless the launcher and all of its dependencies
# are already on disk.
hab pkg install core/hab-launcher --channel $HAB_BLDR_CHANNEL

Describe "Launcher version check" {
    It "Exits with error when running an incompatible supervisor version" {
        Test-LauncherFailure "0.55.0/20180321222338" "exits_with_error_with_incompatible_supervisor" | Should -Be $true
    }
    It "Does not exit with error running an incompatible supervisor version and skipping version check" {
        $env:HAB_LAUNCH_NO_SUP_VERSION_CHECK=1
        Test-LauncherFailure "0.55.0/20180321222338" "no_error_with_incompatible_supervisor_and_skipping_version_check" | Should -Be $false
    }
    It "Does not exit with error when running a compatible supervisor version" {
        Test-LauncherFailure "0.56.0/20180530235935" "no_error_with_compatible_supervisor" | Should -Be $false
    }
    It "Does not exit with error when running a dev supervisor version" {
        Test-LauncherFailure "0.62.0-dev" "no_error_with_dev_supervisor" | Should -Be $false
    }
    It "Exits with error when running a supervisor with an invalid version" {
        Test-LauncherFailure "one-point-twenty-one" "exits_with_error_with_invalid_version" | Should -Be $true
    }
    AfterEach {
        $env:HAB_LAUNCH_NO_SUP_VERSION_CHECK=$null
    }
}
