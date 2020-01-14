# This tests that removing the leader from a functioning leader topology
# service group that has enough nodes to maintain quorum after the leader is
# lost, it will continue to perform a succesful rolling update after a new
# leader is elected.
#
# We will load services on three nodes and then stop the supervisor on
# the leader node prompting a new election where one of the two follower nodes
# becomes a leader. Next we perform an update and expect both nodes to update.
# Prior to https://github.com/habitat-sh/habitat/pull/7167, the update after a
# new leader is elected would never occur because the new leader would continue
# to behave like a follower and wait for instructions to update.

$testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update after leader is removed and quorum is not lost" {
    $release1="habitat-testing/nginx/1.17.4/20191115184838"
    $release2="habitat-testing/nginx/1.17.4/20191115185900"
    hab pkg promote $release1 $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "gamma.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel

    @("alpha", "beta", "gamma") | % { 
        It "loads initial release on $_" {
            Wait-Release -Ident $release1 -Remote $_
        }
    }

    Context "Remove leader" {
        $leader = Get-Leader "bastion" "nginx.default"
        Stop-ComposeSupervisor $leader.Name
        hab pkg promote $release2 $testChannel

        @("alpha", "beta", "gamma") | ? { $_ -ne $leader.Name } | % { 
            It "updates to $release2 on $_" {
                Wait-Release -Ident $release2 -Remote $_
            }
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
