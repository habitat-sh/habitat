hab pkg install core/nginx --channel stable

$job = Start-Job { hab sup run }
Wait-Supervisor -Timeout 120

Describe 'hab svc load' {
    $loadOut = hab svc load core/nginx
    Wait-SupervisorService nginx -Timeout 20

    It 'Successfully loads service' {
        ($loadOut | Out-String) | Should -Match 'The core/nginx service was successfully loaded'
    }

    It 'Reports service on HTTP Gateway as UP' {
        ((Invoke-WebRequest 'http://localhost:9631/services/nginx/default' -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be 'up'
    }

    It 'Actually runs nginx' {
        # This will error with a 403 because nginx is not running any sites
        try {
            Invoke-WebRequest 'http://localhost'
        } catch [Microsoft.PowerShell.Commands.HttpResponseException] {
            $headers = $_.Exception.Response.Headers
        }
        [string]$headers.Server | Should -BeLike 'nginx/*'
    }

    AfterAll {
        hab svc unload core/nginx
        Wait-SupervisorServiceUnload nginx -Timeout 20
        Stop-Job -Job $job
        Remove-Job -Job $job
    }
}

if ($IsLinux) {

    # To test perms on /hab/svc and# /hab/svc/$SERVICE/{hooks,logs} we blow
    # away that hierarchy at this point to start clean
    (bash -c 'rm -rf /hab/svc')

    $job = Start-Job { bash -c 'umask 077; hab sup run' }
    Wait-Supervisor -Timeout 120

    Describe 'hab svc load on Linux with umask 077' {

        $loadOut = hab svc load core/nginx
        Wait-SupervisorService nginx -Timeout 20

        It 'Successfully loads service' {
            ($loadOut | Out-String) |
                Should -Match 'The core/nginx service was successfully loaded'
        }

        It 'Reports service on HTTP Gateway as UP' {
            ((Invoke-WebRequest 'http://localhost:9631/services/nginx/default' -UseBasicParsing).content |
                ConvertFrom-Json).process.state |
                    Should -Be 'up'
        }

        It 'Actually runs nginx' {
            # This will error with a 403 because nginx is not running any sites
            try {
                Invoke-WebRequest 'http://localhost'
            } catch [Microsoft.PowerShell.Commands.HttpResponseException] {
                $headers = $_.Exception.Response.Headers
            }
            [string]$headers.Server | Should -BeLike 'nginx/*'
        }

        It 'nginx is running with umask 0077' {
            $out = (grep Umask /proc/$(pgrep hab-sup)/status)
            $out | Should -Match 'Umask:\s+0077'
        }

        It 'has correct permissions on relevant svc directories' {
            $out = (bash -c "ls -l /hab/ | grep svc | cut -d ' ' -f 1")
            ($out | Out-String) | Should -Match 'drwxr-xr-x'
            $out = (bash -c "ls -l /hab/svc/nginx | grep hooks | cut -d ' ' -f 1")
            ($out | Out-String) | Should -Match 'drwxr-xr-x'
            $out = (bash -c "ls -l /hab/svc/nginx | grep logs | cut -d ' ' -f 1")
            ($out | Out-String) | Should -Match 'drwxr-xr-x'
        }

        AfterAll {
            hab svc unload core/nginx
            Wait-SupervisorServiceUnload nginx -Timeout 20
            Stop-Job -Job $job
            Remove-Job -Job $job
        }
    }
}
