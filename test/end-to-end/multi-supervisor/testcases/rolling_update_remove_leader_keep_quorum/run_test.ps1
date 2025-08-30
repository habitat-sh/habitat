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

$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
switch ($arch) {
    'X64' {
        $script:release1="habitat-testing/nginx/1.17.4/20191115184838"
        $script:release2="habitat-testing/nginx/1.17.4/20191115185900"
    }
    'Arm64' {
        $script:release1="habitat-testing/nginx/1.25.4/20250731123138"
        $script:release2="habitat-testing/nginx/1.25.4/20250731123956"
    }
    Default {
        throw "Unsupported architecture: $arch"
    }
}

$script:testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update after leader is removed and quorum is not lost" {
    hab pkg promote $release1 $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "gamma.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel

    It "loads initial release on alpha" {
        Wait-Release -Ident $release1 -Remote "alpha"
    }
    It "loads initial release on beta" {
        Wait-Release -Ident $release1 -Remote "beta"
    }
    It "loads initial release on gamma" {
        Wait-Release -Ident $release1 -Remote "gamma"
    }

    Context "Remove leader" {
        $script:all = 'alpha','beta','gamma'
        $script:leader = Get-Leader "bastion" "nginx.default"
        $script:killed = $leader.Name
        $script:survivors = $all | Where-Object { $_ -ne $killed }
        $script:survivor1 = $survivors[0]
        $script:survivor2 = $survivors[1]

        BeforeAll {
            Stop-ComposeSupervisor $killed
            hab pkg promote $release2 $testChannel
        }

        It "updates to $release2 on $survivor1" {
            Wait-Release -Ident $release2 -Remote $survivor1
        }
        It "updates to $release2 on $survivor2" {
            Wait-Release -Ident $release2 -Remote $survivor2
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
