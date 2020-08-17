# A simple test that the launcher restarts a supervisor when it exits abruptly
# The one optional argument set the exit code of the supervisor (default: 1).
# By default this runs against the installed habitat binaries. To override and
# test locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

Describe "Supervisor restarts on abrupt exit" {
    $exit_file = New-TemporaryFile
    $env:HAB_FEAT_TEST_EXIT = $exit_file
    $launcher_proc = Start-Supervisor -Timeout 45 -LogFile "sup.log"
    $supervisor_proc = Get-Process hab-sup
    Set-Content $exit_file -Value 1
    
    $testScript = { $supervisor_proc.HasExited }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to exit" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45
    
    $testScript = { Get-Process hab-sup -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to be restarted" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45
    
    $new_supervisor_proc = Get-Process hab-sup
    $new_launcher_proc = Get-Process hab-launch

    It "will not exit the launcher" {
        $launcher_proc.HasExited | Should -Be $false
    }
    It "will spawn a new supervisor" {
        $new_supervisor_proc.Id | Should -Not -Be $supervisor_proc.Id
    }
    It "will not spawn a new launcher" {
        $new_launcher_proc.Id | Should -Be $launcher_proc.Id
    }

    AfterAll {
        Stop-Supervisor
    }
}

Describe "Supervisor restarts on 'hab sup restart'" {
    $launcher_proc = Start-Supervisor -Timeout 45 -LogFile "sup.log"
    $supervisor_proc = Get-Process hab-sup
    Invoke-NativeCommand hab sup restart

    $testScript = { $supervisor_proc.HasExited }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to exit" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45
    
    $testScript = { Get-Process hab-sup -ErrorAction SilentlyContinue }
    $timeoutScript = { Write-Error "Timed out waiting for Supervisor to be restarted" }
    Wait-True -TestScript $testScript -TimeoutScript $timeoutScript -Timeout 45

    $new_supervisor_proc = Get-Process hab-sup
    $new_launcher_proc = Get-Process hab-launch

    It "will not exit the launcher" {
        $launcher_proc.HasExited | Should -Be $false
    }
    It "will spawn a new supervisor" {
        $new_supervisor_proc.Id | Should -Not -Be $supervisor_proc.Id
    }
    It "will not spawn a new launcher" {
        $new_launcher_proc.Id | Should -Be $launcher_proc.Id
    }

    AfterAll {
        Stop-Supervisor
    }
}
