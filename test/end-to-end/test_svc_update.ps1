# Initialize shared setup values in the current scope for sibling Pester blocks.
# Pester 4 teardown reads these values from the file scope.
$initialize = {
    # For *some reason* this incoming environment variable has quotes,
    # which throws everything off, but apparently only on Windows.
    $env:HAB_AUTH_TOKEN = $env:HAB_AUTH_TOKEN.Replace("`"", "")

    Set-Variable -Name testChannelOne -Value ("dynamic-update-one-$([DateTime]::Now.Ticks)") -Scope Local
    Set-Variable -Name testChannelTwo -Value ("dynamic-update-two-$([DateTime]::Now.Ticks)") -Scope Local
    Set-Variable -Name nginx_pkg -Value ("habitat-testing/nginx") -Scope Local
    if ($IsWindows) {
        Set-Variable -Name nginx_release -Value ("habitat-testing/nginx/1.18.0/20200626143933") -Scope Local
    } else {
        Set-Variable -Name nginx_release -Value ("habitat-testing/nginx/1.18.0/20200626184200") -Scope Local
    }
    if ($IsWindows) {
        Set-Variable -Name test_probe_release -Value ("habitat-testing/test-probe/0.1.0/20230620183252") -Scope Local
    } else {
        Set-Variable -Name test_probe_release -Value ("habitat-testing/test-probe/0.1.0/20230621013852") -Scope Local
    }
}
if ($useWindowsPester5) {
    BeforeAll $initialize
} else {
    . $initialize
}

Describe "hab svc update" {
    $setup = {
        $supLog = New-SupervisorLogFile("test_svc_update")
        Start-Supervisor -LogFile $supLog -Timeout 45
        Load-SupervisorService $nginx_release
        Wait-Release -Ident $nginx_release
    }
    if ($useWindowsPester5) {
        BeforeAll $setup
    } else {
        . $setup
    }

    AfterAll {
        hab bldr channel destroy $testChannelOne --origin habitat-testing
        hab bldr channel destroy $testChannelTwo --origin habitat-testing
        Unload-SupervisorService -PackageName $nginx_pkg -Timeout 20
        Stop-Supervisor
    }

    Context "with a bound service" {
        $setup = {
            Load-SupervisorService habitat-testing/test-probe -Bind "thing_with_a_port:nginx.default"
            Wait-Release -Ident $test_probe_release
            Start-Sleep 15 # test-probe has a long init hook
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "starts test-probe bound to nginx" {
            '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'binds = \["thing_with_a_port:nginx.default"\]'
        }

        AfterEach {
            Unload-SupervisorService -PackageName habitat-testing/test-probe -Timeout 30
        }
    }

    Context "removing binds via hab svc update" {
        $setup = {
            Load-SupervisorService habitat-testing/test-probe -Bind "thing_with_a_port:nginx.default"
            Wait-Release -Ident $test_probe_release
            Start-Sleep 15 # test-probe has a long init hook

            Set-Variable -Name proc -Value (Get-Process "test-probe") -Scope Local

            # This is the same as saying "remove binds"... we might
            # find a better way to express this, though.
            hab svc update habitat-testing/test-probe --bind

            # Currently, test-probe has some long-running post-stop
            # and init hooks. They should be done within 30 seconds,
            # but we'll give *plenty* of extra time for the full
            # restart, just in case gremlins appear.
            Start-Sleep -Seconds 45
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "has the new binds in the spec file" {
            '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'binds = \[\]'
        }

        It "DOES restart the service process" {
            $currentProc = Get-Process "test-probe"
            $proc.Id | Should -Not -Be $currentProc.Id
        }

        $cleanup = {
            Unload-SupervisorService -PackageName habitat-testing/test-probe -Timeout 30
        }
        if ($useWindowsPester5) {
            AfterAll $cleanup
        } else {
            . $cleanup
        }
    }

    Context "updating another field does not wipe out existing binds" {
        $setup = {
            Load-SupervisorService habitat-testing/test-probe -Bind "thing_with_a_port:nginx.default"
            Wait-Release -Ident $test_probe_release
            Start-Sleep 15 # test-probe has a long init hook

            Set-Variable -Name proc -Value (Get-Process test-probe) -Scope Local

            hab svc update habitat-testing/test-probe --strategy=at-once

            # This shouldn't take too long to register, and shouldn't
            # trigger a restart, but to catch potential regressions
            # that *do* restart the service, we'll wait for the long
            # post-stop and init hooks to run, as before.
            Start-Sleep -Seconds 45
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "leaves the existing binds alone" {
            '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'binds = \["thing_with_a_port:nginx.default"\]'
        }

        It "updates the update strategy" {
            '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'update_strategy = "at-once"'
        }

        It "does not restart the service process" {
            $currentProc = Get-Process test-probe
            $proc.Id | Should -Be $currentProc.Id
        }

        $cleanup = {
            Unload-SupervisorService -PackageName habitat-testing/test-probe -Timeout 30
        }
        if ($useWindowsPester5) {
            AfterAll $cleanup
        } else {
            . $cleanup
        }
    }

    # This is an imperfect test, but useful nevertheless.
    #
    # There are some aspects of a Habitat service's runtime configuration
    # (from the Supervisor's perspective) that can be changed without
    # requiring a service restart. At the time of writing, this is limited
    # to details of how a service is updated (e.g., what channel updates
    # are coming from).
    #
    # Testing this can be a little tricky, because changing how a service
    # is updated can actually cause the service to restart *for a new
    # update*. There aren't currently great ways to probe the Supervisor
    # to ask details about the update process for a service (e.g., by
    # using a `hab` command, which would be pretty nice, actually).
    #
    # Instead, what we'll do here is set up two channels with the same
    # package in each. We'll start the service set to update from the one
    # channel, then switch it to the other. We'll confirm the PID of the
    # process has not changed, while looking at the contents of the
    # service's spec file, which should have changed.
    #
    # It's not great, in that it's not a positive confirmation (e.g.,
    # breaking the process of discovering a change to the spec file on
    # disk would yield the same result), but it's better than nothing.
    Context "an updating service" {
        $setup = {
            hab pkg promote $nginx_release $testChannelOne
            hab pkg promote $nginx_release $testChannelTwo

            Load-SupervisorService $nginx_pkg -Force -Strategy "at-once" -Channel $testChannelOne
            Start-Sleep 10

            Set-Variable -Name proc -Value (Get-Process "nginx*") -Scope Local
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "reflects channel in spec file" {
            '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelOne`""
        }

        It "reflects channel in api" {
            ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).channel | Should -Be $testChannelOne
        }

        $orderedTests = {
            $setup = {
                hab svc update $nginx_pkg --channel $testChannelTwo

                # Give *plenty* of time to pick up the new spec (as well as
                # time for a service to restart, if things are broken and
                # that's a thing that could happen).
                Start-Sleep -Seconds 10
            }
            if ($useWindowsPester5) {
                BeforeAll $setup
            } else {
                . $setup
            }

            It "has the new channel in the spec file" {
                '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelTwo`""
                '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly 'update_strategy = "at-once"'
            }

            It "reflects updated channel in api" {
                ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).channel | Should -Be $testChannelTwo
            }

            It "does not restart the service process" {
                $currentProc = Get-Process "nginx*"
                $proc.Id | Should -Be $currentProc.Id
            }
        }
        if ($useWindowsPester5) {
            Context "after changing the channel" $orderedTests
        } else {
            . $orderedTests
        }
    }
}
