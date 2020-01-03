# When using a current Launcher and enabling the PIDS_FROM_LAUNCHER
# feature, the Supervisor should not create PID files for the services
# it manages.

Describe "PIDS_FROM_LAUNCHER feature" {
    $env:HAB_FEAT_PIDS_FROM_LAUNCHER=1
    Start-Supervisor -Timeout 20
    Load-SupervisorService -PackageName "core/redis" -Timeout 20
    Wait-Process redis-server -Timeout 10

    It "should not create PID file" {
        Test-Path "/hab/svc/redis/PID" | Should -Be $false
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
