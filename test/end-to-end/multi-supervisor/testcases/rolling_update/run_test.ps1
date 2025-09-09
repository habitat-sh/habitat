# This is a simple "happy path" test of a rolling update.
# We will load services on five nodes then promote an update and expect the new release to show
# up after waiting 15 seconds. Then we demote the package and validate that the nodes
# rolled back. The package will "hang" upon receiving its SIGTERM which will trigger the supervisor
# to forcefully terminate the service. This tests an edge case where the package incarnation was being
# reset to 0 and causing nodes to get stuck and not update or roll back.
# Note: we set HAB_UPDATE_STRATEGY_FREQUENCY_MS to 3000 in the docker-compose.override.yml.

$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
switch ($arch) {
    'X64' {
        $script:initialRelease = 'habitat-testing/force-kill/0.1.0/20230214152940'
        $script:updatedRelease = 'habitat-testing/force-kill/0.1.0/20230214154036'
    }
    'Arm64' {
        $script:initialRelease = 'habitat-testing/force-kill/0.1.0/20250730230943'
        $script:updatedRelease = 'habitat-testing/force-kill/0.1.0/20250730231035'
    }
    Default {
        throw "Unsupported architecture: $arch"
    }
}

$script:testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update and Rollback" {
    hab pkg promote $initialRelease $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma1.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma2.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma3.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel

    It "loads initial release on alpha" {
        Wait-Release -Ident $initialRelease -Remote "alpha"
    }
    It "loads initial release on beta" {
        Wait-Release -Ident $initialRelease -Remote "beta"
    }
    It "loads initial release on gamma1" {
        Wait-Release -Ident $initialRelease -Remote "gamma1"
    }
    It "loads initial release on gamma2" {
        Wait-Release -Ident $initialRelease -Remote "gamma2"
    }
    It "loads initial release on gamma3" {
        Wait-Release -Ident $initialRelease -Remote "gamma3"
    }

    Context "promote update" {
        BeforeAll {
            Write-Host "[$(Get-Date -Format o)] Promoting updatedRelease to $testChannel"
            hab pkg promote $updatedRelease $testChannel
        }

        It "updates release on alpha" {
            Wait-Release -Ident $updatedRelease -Remote "alpha"
        }
        It "updates release on beta" {
            Wait-Release -Ident $updatedRelease -Remote "beta"
        }
        It "updates release on gamma1" {
            Wait-Release -Ident $updatedRelease -Remote "gamma1"
        }
        It "updates release on gamma2" {
            Wait-Release -Ident $updatedRelease -Remote "gamma2"
        }
        It "updates release on gamma3" {
            Wait-Release -Ident $updatedRelease -Remote "gamma3"
        }
    }

    Context "demote update" {
        BeforeAll {
            hab pkg demote $updatedRelease $testChannel
        }

        It "rolls back release on alpha" {
            Wait-Release -Ident $initialRelease -Remote "alpha"
        }
        It "rolls back release on beta" {
            Wait-Release -Ident $initialRelease -Remote "beta"
        }
        It "rolls back release on gamma1" {
            Wait-Release -Ident $initialRelease -Remote "gamma1"
        }
        It "rolls back release on gamma2" {
            Wait-Release -Ident $initialRelease -Remote "gamma2"
        }
        It "rolls back release on gamma3" {
            Wait-Release -Ident $initialRelease -Remote "gamma3"
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
