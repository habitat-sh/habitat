# Test Supervisor package cleanup using the `--keep-latest-packages` option

$pkg="chef/hab-sup"

Describe "Supervisor package cleanup" {
    It "loads old Supervisor versions" {
        hab pkg install "chef/hab-sup/2.0.171" --channel unstable
        hab pkg install "chef/hab-sup/2.0.172" --channel unstable
        hab pkg install "chef/hab-sup/2.0.173" --channel unstable
        hab pkg install "chef/hab-sup/2.0.174" --channel unstable
        Wait-CommandLinesOfOutput "hab pkg list $pkg" 4
    }

    Context "start the Supervisor without package cleanup" {
        $supLog = New-SupervisorLogFile("start_the_supervisor_without_package_cleanup")
        Start-Supervisor -LogFile $supLog -Timeout 45 | Out-Null

        It "does not remove old Supervisor packages" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 5
        }
        Stop-Supervisor
        Start-Sleep 3 # Wait for the supervisor to actually stop
    }

    Context "start the Supervisor with package cleanup" {
        $supLog = New-SupervisorLogFile("start_the_supervisor_with_package_cleanup")
        $expected = hab pkg list chef/hab-sup | Select-Object -Last 2
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--keep-latest-packages=2"
        ) | Out-Null

        It "removes old Supervisor packages" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 2
            hab pkg list $pkg | Should -Be $expected
        }
    }

    AfterAll {
        Stop-Supervisor
    }
}
