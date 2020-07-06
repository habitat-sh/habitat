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

# For *some reason* this incoming environment variable has quotes,
# which throws everything off, but apparently only on Windows.
$env:HAB_AUTH_TOKEN = $env:PIPELINE_HAB_AUTH_TOKEN.Replace("`"", "")

Write-Host "PIPELINE_HAB_AUTH_TOKEN='$env:PIPELINE_HAB_AUTH_TOKEN'"
Write-Host "HAB_AUTH_TOKEN='$env:HAB_AUTH_TOKEN'"

$testChannelOne="dynamic-update-one-$([DateTime]::Now.Ticks)"
$testChannelTwo="dynamic-update-two-$([DateTime]::Now.Ticks)"
$pkg="habitat-testing/nginx"
if ($IsWindows) {
    $release="habitat-testing/nginx/1.18.0/20200626143933"
} else {
    $release="habitat-testing/nginx/1.18.0/20200626184200"
}

Describe "modification of Supervisor-specific service configuration" {
    Start-Supervisor -Timeout 45

    hab pkg promote $release $testChannelOne
    hab pkg promote $release $testChannelTwo

    Load-SupervisorService $pkg -Strategy "at-once" -Channel $testChannelOne
    Wait-Release -Ident $release

    $proc = Get-Process nginx

    It "starts updating from the first channel" {
        '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelOne`""
    }

    Context "modify update channel" {
        # We must include the strategy as well, because `hab svc load`
        # is not a patchy operation.
        hab svc load $pkg --force --strategy "at-once" --channel $testChannelTwo

        # Give *plenty* of time to pick up the new spec (as well as
        # time for a service to restart, if things are broken and
        # that's a thing that could happen).
        Start-Sleep -Seconds 10

        It "has the new channel in the spec file" {
            '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "channel = `"$testChannelTwo`""
            '/hab/sup/default/specs/nginx.spec' | Should -FileContentMatchExactly "strategy = `"at-once`""
        }

        It "does not restart the service process" {
            $currentProc = Get-Process nginx
            $proc.Id | Should -Be $currentProc.Id
        }
    }

    AfterAll {
        hab bldr channel destroy $testChannelOne --origin habitat-testing
        hab bldr channel destroy $testChannelTwo --origin habitat-testing
        Unload-SupervisorService -PackageName $pkg -Timeout 20
        Stop-Supervisor
    }
}
