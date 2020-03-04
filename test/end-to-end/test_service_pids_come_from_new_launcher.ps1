# When using a current Launcher the Supervisor should not create PID
# files for the services it manages.

Describe "Service PIDs from Launcher feature" {
    Start-Supervisor -Timeout 20
    Load-SupervisorService -PackageName "core/redis" -Timeout 20
    Wait-Process redis-server -Timeout 10

    It "should still create a PID file for use in hooks" {
        Test-Path "/hab/svc/redis/PID" | Should -Be $true
    }

    Context "Supervisor is restarted" {
        $supProc = Get-Process hab-sup
        $redisProc = Get-Process redis-server

        # Write a bogus PID to the file; the Supervisor should not
        # think this is the actual PID of the service.
        #
        # NOTE: This is ONLY to prove, in the context of this test,
        # that the Supervisor is not getting its idea of the process
        # PID from the file. The Supervisor does not currently (and
        # never did) try to keep the contents of the file in sync with
        # the running service PID on an ongoing basis.
        $bogusPid = $redisProc.Id + 5
        $bogusPid | Out-File "/hab/svc/redis/PID"

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

        It "should not have gotten the service PID from the PID file" {
            # The contents of the PID file are still incorrect; the
            # Supervisor doesn't rewrite it after it restarts (it
            # would after a full restart, but that would bring up a
            # new service process anyway)
            Get-Content "/hab/svc/redis/PID" | Should -Be $bogusPid

            # Despite the bogus PID in the file, the Supervisor itself
            # knows the correct PID
            Get-HabServicePID "core/redis" | Should -Be $newRedisProc.Id
        }
    }
}
