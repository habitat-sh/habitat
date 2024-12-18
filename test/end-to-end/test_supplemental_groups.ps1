# This test is Linux only, as it deals with Linux-specific functionality

# The habitat-testing/supplemental-group-tester package has an `init`
# hook (run by the Supervisor) and a `run` hook (run by the Launcher)
# that both try to read a single file `/tmp/supplemental-group-tester-file`.
#
# The package has service user and group of `hab`.
#
# The testing file should not be directly readable by the `hab` user
# or group, but should instead be readable by some other group. The
# `hab` user should be made a member of that group. If all goes well,
# then both the `init` and `run` hooks should be able to read this
# file, allowing them to progress. A successful `init` hook run will
# allow the `run` hook to begin, and a successful file check in the
# `run` hook will then allow it to continue into an infinite sleep
# loop. Before doing that, though, it will write the string `SUCCESS`
# to the file `/tmp/supplemental-group-tester-file-sentinel`; if that
# file exists, everything is successful.

$test_file="/tmp/supplemental-group-tester-file"
$sentinel_file="/tmp/supplemental-group-tester-sentinel"
$supplemental_group="extra_group"

# Ensure the sentinel isn't there already, for some reason
Remove-Item "$sentinel_file" -Force -ErrorAction SilentlyContinue

# Create the testing file... contents aren't important
"Hello World" | Out-File -FilePath $test_file

# Create a new group, give the file to that group, prevent non-group
# members from accessing the file, and add the `hab` user to the
# group.
#
# Dropping back to raw Unix utilities for all this, for convenience.
groupadd $supplemental_group
chgrp $supplemental_group $test_file
chown 740 $test_file
usermod -G $supplemental_group hab

Describe "supplemental group behavior" {
    AfterAll {
        Stop-Supervisor
    }

    $supLog = New-SupervisorLogFile("test_supplemental_groups")
    Start-Supervisor -LogFile $supLog -Timeout 45

    It "should be able to run a service that depends on supplemental groups being set" {
        # Install the package first so we don't have to wait during
        # the load.
        hab pkg install "habitat-testing/supplemental-group-tester" --channel=LTS-2024
        Load-SupervisorService "habitat-testing/supplemental-group-tester"

        # The service should not be able to come up if supplemental
        # groups are not properly passed in the Supervisor and the
        # Launcher. Once it has made the checks to confirm the groups
        # are set up properly, it will write "SUCCESS" to the sentinel
        # file. If we can see that, we'll consider it a passing test.
        Start-Sleep 5
        Get-Content "$sentinel_file" | Should -Be "SUCCESS"
    }
}
