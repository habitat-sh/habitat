# Test the at-once service update strategy
# The timing of this test assumes the following environment variables are set:
# HAB_UPDATE_STRATEGY_FREQUENCY_MS=3000
# HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK=1

$supLog = New-SupervisorLogFile("test_at_once_service_updater")
Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
        "--keep-latest-packages=1"
) | Out-Null
$testChannel="at-once-$([DateTime]::Now.Ticks)"
$pkg="habitat-testing/nginx"
$initialRelease="habitat-testing/nginx/1.17.4/20191115184838"
$updatedRelease="habitat-testing/nginx/1.17.4/20191115185517"

Describe "at-once update and rollback" {
    hab pkg promote $initialRelease $testChannel
    Load-SupervisorService $pkg -Strategy "at-once" -UpdateCondition "track-channel" -Channel $testChannel

    It "loads initial release" {
        Wait-Release -Ident $initialRelease
    }

    Context "promote update" {
        hab pkg promote $updatedRelease $testChannel

        It "updates release" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 2
            Wait-Release -Ident $updatedRelease
            # The first package should eventually be automatically uninstalled
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 1
            hab pkg list $pkg | Should -Be $updatedRelease
        }
    }

    Context "demote update" {
        hab pkg demote $updatedRelease $testChannel

        It "rollback release" {
            Wait-Release -Ident $initialRelease
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
        Unload-SupervisorService -PackageName $pkg -Timeout 20
        Stop-Supervisor
    }
}
