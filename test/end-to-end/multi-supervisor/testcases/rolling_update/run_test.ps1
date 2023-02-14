# This is a simple "happy path" test of a rolling update.
# We will load services on five nodes then promote an update and expect the new release to show
# up after waiting 15 seconds. Then we demote the package and validate that the nodes
# rolled back. The package will "hang" upon receiving its SIGTERM which will trigger the supervisor
# to forcefully terminate the service. This tests an edge case where the package incarnation was being
# reset to 0 and causing nodes to get stuck and not update or roll back.
# Note: we set HAB_UPDATE_STRATEGY_FREQUENCY_MS to 3000 in the docker-compose.override.yml.

$testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update and Rollback" {
    $initialRelease="habitat-testing/force-kill/0.1.0/20230214152940"
    $updatedRelease="habitat-testing/force-kill/0.1.0/20230214154036"
    hab pkg promote $initialRelease $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma1.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma2.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/force-kill" -Remote "gamma3.habitat.dev" -Topology leader -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel

    @("alpha", "beta", "gamma1", "gamma2", "gamma3") | ForEach-Object {
        It "loads initial release on $_" {
            Wait-Release -Ident $initialRelease -Remote $_
        }
    }

    Context "promote update" {
        hab pkg promote $updatedRelease $testChannel

        @("alpha", "beta", "gamma1", "gamma2", "gamma3") | ForEach-Object {
            It "updates release on $_" {
                Wait-Release -Ident $updatedRelease -Remote $_
            }
        }
    }

    Context "demote update" {
        hab pkg demote $updatedRelease $testChannel

        @("alpha", "beta", "gamma1", "gamma2", "gamma3") | ForEach-Object {
            It "rollback release on $_" {
                Wait-Release -Ident $initialRelease -Remote $_
            }
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
