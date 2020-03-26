# Test Supervisor package cleanup using the `--keep-latest-packages` option

$pkg="core/hab-sup"

Describe "Supervisor package cleanup" {
    It "loads old Supervisor versions" {
        hab pkg install "core/hab-sup/1.5.30" --channel unstable
        hab pkg install "core/hab-sup/1.5.42" --channel unstable
        hab pkg install "core/hab-sup/1.5.50" --channel unstable
        hab pkg install "core/hab-sup/1.5.60" --channel unstable
        Wait-CommandLinesOfOutput "hab pkg list $pkg" 4
    }

    Context "start the Supervisor without package cleanup" {
        $supLog = New-TemporaryFile
        Start-Supervisor -LogFile $supLog -Timeout 45 | Out-Null

        It "does not remove old Supervisor packages" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 5
        }
        Stop-Supervisor
        Start-Sleep 3 # Wait for the supervisor to actually stop
    }

    Context "start the Supervisor with package cleanup" {
        $supLog = New-TemporaryFile
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--keep-latest-packages=2"
        ) | Out-Null

        It "removes old Supervisor packages" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 2
            hab pkg list $pkg | Select-Object -Index 0 | Should -BeLike "$pkg/1.5.60*"
        }
    }

    AfterAll {
        Stop-Supervisor
    }
}
