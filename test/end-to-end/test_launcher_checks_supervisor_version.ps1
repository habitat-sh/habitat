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

function Test-LauncherFailure($version) {
    $sup_binary = New-BinaryStub $version

    $env:HAB_SUP_BINARY=$sup_binary
    $sup = New-Object SupervisorRunner
    $supPid = $sup.Run("sup.log")
    
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

Describe "Launcher version check" {
    It "Exits with error when running an incompatible supervisor version" {
        Test-LauncherFailure "0.55.0/20180321222338" | Should -Be $true
    }
    It "Does not exit with error running an incompatible supervisor version and skipping version check" {
        $env:HAB_LAUNCH_NO_SUP_VERSION_CHECK=1
        Test-LauncherFailure "0.55.0/20180321222338" | Should -Be $false
    }
    It "Does not exit with error when running a compatible supervisor version" {
        Test-LauncherFailure "0.56.0/20180530235935" | Should -Be $false
    }
    It "Does not exit with error when running a dev supervisor version" {
        Test-LauncherFailure "0.62.0-dev" | Should -Be $false
    }
    It "Exits with error when running a supervisor with an invalid version" {
        Test-LauncherFailure "one-point-twenty-one" | Should -Be $true
    }
    AfterEach {
        $env:HAB_LAUNCH_NO_SUP_VERSION_CHECK=$null
    }
}
