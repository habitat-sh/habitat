# When using a Launcher from before the PIDS_FROM_LAUNCHER feature was
# created, we should still be using PID files for individual services,
# even if we've enabled the feature.

Describe "PIDS_FROM_LAUNCHER feature" {
    $env:HAB_FEAT_PIDS_FROM_LAUNCHER=1

    # This was the last stable Linux launcher prior to the
    # PIDS_FROM_LAUNCHER feature.
    hab pkg install core/hab-launcher/12605/20191112144831

    Start-Supervisor -Timeout 20
    Load-SupervisorService -PackageName "core/redis" -Timeout 20
    Wait-Process redis-server -Timeout 10

    It "should create PID file" {
        Test-Path "/hab/svc/redis/PID" | Should -Be $true
    }

    Context "Supervisor is restarted" {
        $supProc = Get-Process hab-sup
        $redisProc = Get-Process redis-server
        Restart-Supervisor
        Wait-Process redis-server -Timeout 10
        $newSupProc = Get-Process hab-sup
        $newRedisProc = Get-Process redis-server

        It "starts a new supervisor process" {
            $supProc.Id | Should -Not -Be $newSupProc.Id
        }
        It "runs the same redis process" {
            $redisProc.Id | Should -Be $newRedisProc.Id
        }
    }
}
