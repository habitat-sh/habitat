# Test the at-once service update strategy
# The timing of this test assumes the following environment variables are set:
# HAB_UPDATE_STRATEGY_PERIOD_MS=3000
# HAB_UPDATE_STRATEGY_PERIOD_BYPASS_CHECK=1

BeforeAll {
    $supLog = New-SupervisorLogFile("test_at_once_service_updater")
    Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
            "--keep-latest-packages=1"
    ) | Out-Null
    Set-Variable -Name testChannel -Value ("at-once-$([DateTime]::Now.Ticks)") -Scope Local
    Set-Variable -Name pkg -Value ("habitat-testing/nginx") -Scope Local
    Set-Variable -Name initialRelease -Value ("habitat-testing/nginx/1.17.4/20191115184838") -Scope Local
    Set-Variable -Name updatedRelease -Value ("habitat-testing/nginx/1.17.4/20191115185517") -Scope Local
}

Describe "at-once update and rollback" {
    BeforeAll {
        hab pkg promote $initialRelease $testChannel
        Load-SupervisorService $pkg -Strategy "at-once" -UpdateCondition "track-channel" -Channel $testChannel
    }

    It "loads initial release" {
        Wait-Release -Ident $initialRelease
    }

    Context "promote update" {
        BeforeAll {
            hab pkg promote $updatedRelease $testChannel
        }

        It "updates release" {
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 2
            Wait-Release -Ident $updatedRelease
            # The first package should eventually be automatically uninstalled
            Wait-CommandLinesOfOutput "hab pkg list $pkg" 1
            hab pkg list $pkg | Should -Be $updatedRelease
        }
    }

    Context "demote update" {
        BeforeAll {
            hab pkg demote $updatedRelease $testChannel
        }

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
