# This tests that removing the leader from a functioning 2 node leader topology
# service group will continue to perform a succesful rolling update after a new
# member is added to the group and quorum is reestablished.
#
# We will load services on two nodes and perform a rolling update. Next we stop
# the supervisor on the leader node and then load an older version of the service
# on a new node reestablishing quorum. Next we perform an update and expect all
# nodes to update. Prior to https://github.com/habitat-sh/habitat/pull/7167, if
# the newly added node is elected the leader, the follower which has a newer
# version of the service will end up in a state where it is continually updating
# to the older version of the leader, restarting the service and loading the newer
# service, then updating to the older leader version and so on until the end of
# its precious life. Now followers should never consider an older version a
# candidate for updating.

$arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
switch ($arch) {
    'X64' {
        $script:release1 = "habitat-testing/nginx/1.17.4/20191115184838"
        $script:release2 = "habitat-testing/nginx/1.17.4/20191115185517"
        $script:release3 = "habitat-testing/nginx/1.17.4/20191115185900"
    }
    'Arm64' {
        $script:release1 = "habitat-testing/nginx/1.25.4/20250731123138"
        $script:release2 = "habitat-testing/nginx/1.25.4/20250731123657"
        $script:release3 = "habitat-testing/nginx/1.25.4/20250731123956"
    }
    Default {
        throw "Unsupported architecture: $arch"
    }
}

$script:testChannel = "rolling-$([DateTime]::Now.Ticks)"

Describe "Rolling Update after a follower is removed and quorum is not lost" {
    hab pkg promote $release1 $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "alpha.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
    Load-SupervisorService "habitat-testing/nginx" -Remote "beta.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel

    It "loads initial release on alpha" { 
        Wait-Release -Ident $release1 -Remote "alpha" 
    }
    It "loads initial release on beta" { 
        Wait-Release -Ident $release1 -Remote "beta"
    }

    Context "promote $release2" {
        BeforeAll {
            hab pkg promote $release2 $testChannel
        }
        It "updates to $release2 on alpha" {
            Wait-Release -Ident $release2 -Remote "alpha"
        }
        It "updates to $release2 on beta" {
            Wait-Release -Ident $release2 -Remote "beta"
        }

        Context "Stop leader and reestablish quorum with an older release on gamma then promote $release3" {
            $script:all = 'alpha','beta'
            $script:leader = Get-Leader "bastion" "nginx.default"
            $script:killed = $leader.Name
            $script:survivor = $all | Where-Object { $_ -ne $killed }
            $script:targets = @($survivor, 'gamma')
            $script:target1 = $targets[0]
            $script:target2 = $targets[1]
            
            BeforeAll {
                Stop-ComposeSupervisor $killed
                docker exec "${env:COMPOSE_PROJECT_NAME}-gamma-1" hab pkg install $release1
                Start-Sleep 10
                Load-SupervisorService "habitat-testing/nginx" -Remote "gamma.habitat.dev" -Topology leader -Strategy rolling -Channel $testChannel
                hab pkg promote $release3 $testChannel

            }
            It "updates to $release3 on $target1" {
                Wait-Release -Ident $release3 -Remote $target1
            }
            It "updates to $release3 on $target2" {
                Wait-Release -Ident $release3 -Remote $target2
            }
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannel --origin habitat-testing
    }
}
