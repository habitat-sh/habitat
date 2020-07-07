# For *some reason* this incoming environment variable has quotes,
# which throws everything off, but apparently only on Windows.
$env:HAB_AUTH_TOKEN = $env:PIPELINE_HAB_AUTH_TOKEN.Replace("`"", "")

Write-Host "PIPELINE_HAB_AUTH_TOKEN='$env:PIPELINE_HAB_AUTH_TOKEN'"
Write-Host "HAB_AUTH_TOKEN='$env:HAB_AUTH_TOKEN'"

$testChannelOne="dynamic-update-one-$([DateTime]::Now.Ticks)"
$testChannelTwo="dynamic-update-two-$([DateTime]::Now.Ticks)"
$nginx_pkg="habitat-testing/nginx"
if ($IsWindows) {
    $nginx_release="habitat-testing/nginx/1.18.0/20200626143933"
} else {
    $nginx_release="habitat-testing/nginx/1.18.0/20200626184200"
}

Describe "hab svc update" {
    AfterAll {
        hab bldr channel destroy $testChannelOne --origin habitat-testing
        hab bldr channel destroy $testChannelTwo --origin habitat-testing
        Unload-SupervisorService -PackageName $nginx_pkg -Timeout 20
        Stop-Supervisor
    }

    Start-Supervisor -Timeout 45

    Context "with a bound service" {
        Load-SupervisorService $nginx_pkg
        Wait-Release -Ident $nginx_release

        BeforeEach {
            Load-SupervisorService habitat-testing/test-probe --bind=thing_with_a_port:nginx.default
            Wait-Release -Ident habitat-testing/test-probe
        }

        AfterEach {
            Unload-SupervisorService -PackageName habitat-testing/test-probe -Timeout 30
        }

        AfterAll {
            Unload-SupervisorService -PackageName $nginx_pkg -Timeout 20
        }

        It "starts test-probe bound to nginx" {
            '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'binds = ["thing_with_a_port:nginx.default"]'
        }

        Context "removing binds via hab svc update" {
            $proc = Get-Process test-probe

            # This is the same as saying "remove binds"... we might
            # find a better way to express this, though.
            hab svc update habitat-testing/test-probe --bind

            # Currently, test-probe has some long-running post-stop
            # and init hooks. They should be done within 30 seconds,
            # but we'll give *plenty* of extra time for the full
            # restart, just in case gremlins appear.
            Start-Sleep -Seconds 45

            It "has the new binds in the spec file" {
                '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly "binds = []"
            }

            It "DOES restart the service process" {
                $currentProc = Get-Process test-probe
                $proc.Id | Should -Not -Be $currentProc.Id
            }
        }

        Context "updating another field does not wipe out existing binds" {
            $proc = Get-Process test-probe

            hab svc update habitat-testing/test-probe --strategy=at-once

            # This shouldn't take too long to register, and shouldn't
            # trigger a restart, but to catch potential regressions
            # that *do* restart the service, we'll wait for the long
            # post-stop and init hooks to run, as before.
            Start-Sleep -Seconds 45

            It "leaves the existing binds alone" {
                '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'binds = ["thing_with_a_port:nginx.default"]'
            }

            It "updates the update strategy" {
                '/hab/sup/default/specs/test-probe.spec' | Should -FileContentMatchExactly 'update_strategy = "at-once"'
            }

            It "does not restart the service process" {
                $currentProc = Get-Process test-probe
                $proc.Id | Should -Be $currentProc.Id
            }
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
        hab pkg promote $nginx_release $testChannelOne
        hab pkg promote $nginx_release $testChannelTwo

        Load-SupervisorService $nginx_pkg -Strategy "at-once" -Channel $testChannelOne
        Wait-Release -Ident $nginx_release

        $proc = Get-Process nginx

        It "starts updating from the first channel" {
            '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelOne`""
        }

        Context "modify update channel" {
            hab svc update $nginx_pkg --channel $testChannelTwo

            # Give *plenty* of time to pick up the new spec (as well as
            # time for a service to restart, if things are broken and
            # that's a thing that could happen).
            Start-Sleep -Seconds 10

            It "has the new channel in the spec file" {
                '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelTwo`""
                '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly 'update_strategy = "at-once"'
            }

            It "does not restart the service process" {
                $currentProc = Get-Process nginx
                $proc.Id | Should -Be $currentProc.Id
            }
        }
    }
}
