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
        $script:release2="habitat-testing/nginx/1.17.4/20191115185517"
        $script:release3="habitat-testing/nginx/1.17.4/20191115185900"
    }
    'Arm64' {
        $script:release1="habitat-testing/nginx/1.25.4/20250731123138"
        $script:release2="habitat-testing/nginx/1.25.4/20250731123657"
        $script:release3="habitat-testing/nginx/1.25.4/20250731123956"
    }
    Default {
        throw "Unsupported architecture: $arch"
    }
}

$script:testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update demotes a package in the middle of an update" {
    hab pkg promote $release1 $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel
    Load-SupervisorService "habitat-testing/nginx" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel
    Load-SupervisorService "habitat-testing/nginx" -Remote "gamma.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel -UpdateCondition track-channel

    It "loads initial release on alpha" {
        Wait-Release -Ident $release1 -Remote "alpha"
    }
    It "loads initial release on beta" {
        Wait-Release -Ident $release1 -Remote "beta"
    }
    It "loads initial release on gamma" {
        Wait-Release -Ident $release1 -Remote "gamma"
    }

    Context "Promote Package" {
        BeforeAll {
            $script:leader = Get-Leader "bastion" "nginx.default"
            hab pkg promote $release2 $testChannel
        }

        It "updates $($leader.Name) to $release2" {
            Wait-Release -Ident $release2 -Remote $leader.Name
        }
    }

    Context "Demote Package" {
        BeforeAll {
            hab pkg demote $release2 $testChannel
        }

        It "reverts alpha to $release1" {
            Wait-Release -Ident $release1 -Remote "alpha"
        }
        It "reverts beta to $release1" {
            Wait-Release -Ident $release1 -Remote "beta"
        }
        It "reverts gamma to $release1" {
            Wait-Release -Ident $release1 -Remote "gamma"
        }
    }

    Context "Promote Package after demote" {
        BeforeAll {
            hab pkg promote $release3 $testChannel
        }

        It "updates alpha to $release3" {
            Wait-Release -Ident $release3 -Remote "alpha" -Timeout 30
        }
        It "updates beta to $release3" {
            Wait-Release -Ident $release3 -Remote "beta" -Timeout 30
        }
        It "updates gamma to $release3" {
            Wait-Release -Ident $release3 -Remote "gamma" -Timeout 30
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
