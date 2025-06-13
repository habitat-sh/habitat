Add-Type -TypeDefinition (Get-Content "$PSScriptroot/../../.expeditor/scripts/end_to_end/SupervisorRunner.cs" | Out-String)

# Download the dependencies first to prevent timing issues, long timeouts, etc.
hab pkg install chef/hab-sup --channel="${env:HAB_BLDR_CHANNEL}"
hab pkg install chef/hab-launcher --channel="${env:HAB_BLDR_CHANNEL}"

# Write the given content to `/hab/sup/default/LOCK`
function Write-Lockfile($content) {
    # Ensure the directory path is present first
    New-Item -ItemType Directory -Path "/hab/sup/default" -ErrorAction SilentlyContinue
    # Write the contents to the lock file path
    $content | Out-File "/hab/sup/default/LOCK"
}

# Find a PID that doesn't correspond to an actual process; if things go wrong,
# we don't want to accidentally kill something important!
function Select-BogusPID() {
    $test_pid = 2112
    while (Get-Process -Id $test_pid -ErrorAction SilentlyContinue) {
        $test_pid += 1
    }
    $test_pid
}

# Spawn a separate process that sleeps for 100 seconds. We do this to simply
# have a running process that we control.
#
# Returns the process information.
function Start-SleeperProcess() {
    Start-Process $PSHOME\pwsh -PassThru -ArgumentList "-Command  `"&{ Start-Sleep -Seconds 100 }`""
}

Describe "Supervisor LOCK file" {
    Context "with stale LOCK file containing non-running PID" {
        Write-Lockfile(Select-BogusPID)

        It "Starts the Supervisor anyway" {
            $supLog = New-SupervisorLogFile("with_stale_lock_file_containing_non_running_pid")
            Start-Supervisor -Timeout 45 -LogFile $supLog
        }

        AfterEach {
            Stop-Supervisor
        }
    }

    Context "with stale LOCK file containing a running, non-Launcher PID" {
        $sleeper = Start-SleeperProcess
        Write-Lockfile($sleeper.Id)
        AfterEach {
            $sleeper | Stop-Process -ErrorAction SilentlyContinue
            Stop-Supervisor
        }

        It "Starts the Supervisor anyway" {
            $supLog = New-SupervisorLogFile("with_stale_lock_file_containing_a_running_non_launcher_pid")
            Start-Supervisor -Timeout 45 -LogFile $supLog
        }
    }

    Context "with a real, legitimate LOCK file" {
        $legitimateLog = New-SupervisorLogFile("legitimate_supervisor")
        Start-Supervisor -Timeout 45 -LogFile $legitimateLog

        AfterEach {
            Stop-Supervisor
        }

        It "Fails to start another Supervisor" {
            $sup = New-Object SupervisorRunner
            $supLog = New-SupervisorLogFile("with_a_real_legitimate_lock_file")
            $supPid = $sup.Run($supLog)

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
            $supPid.ExitCode | Should -Not -Be 0
            $supLog | Should -FileContentMatch "Is another Supervisor process running?"
        }
    }
}
