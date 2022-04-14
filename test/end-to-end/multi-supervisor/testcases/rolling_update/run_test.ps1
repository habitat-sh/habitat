# This is a simple "happy path" test of a rolling update.
# We will load services on two nodes to achieve quorum and
# then promote an update and expect the new release to show
# up after waiting 15 seconds. Note: we set HAB_UPDATE_STRATEGY_FREQUENCY_MS
# to 3000 in the docker-compose.override.yml.

$testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update and Rollback" {
    $initialRelease="habitat-testing/nginx/1.17.4/20191115184838"
    $updatedRelease="habitat-testing/nginx/1.17.4/20191115185517"
    $release3="habitat-testing/nginx/1.17.4/20191115185900"
    hab pkg promote $initialRelease $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "alpha.habitat.dev" -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "beta.habitat.dev" -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "gamma.habitat.dev" -Strategy rolling -UpdateCondition "track-channel" -Channel $testChannel

    @("alpha", "beta", "gamma") | ForEach-Object {
        It "loads initial release on $_" {
            Wait-Release -Ident $initialRelease -Remote $_
        }
    }

    Context "promote update" {
        hab pkg promote $updatedRelease $testChannel

        @("alpha", "beta", "gamma") | ForEach-Object {
            It "updates release on $_" {
                Wait-Release -Ident $updatedRelease -Remote $_
            }
        }
    }

    Context "promote update to release3" {
        hab pkg promote $release3 $testChannel

        @("alpha", "beta", "gamma") | ForEach-Object {
            It "updates release on $_" {
                Wait-Release -Ident $release3 -Remote $_
            }
        }
    }

    Context "demote update" {
        hab pkg demote $release3 $testChannel

        @("alpha", "beta", "gamma") | ForEach-Object {
            It "rollback release on $_" {
                Wait-Release -Ident $updatedRelease -Remote $_
            }
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
