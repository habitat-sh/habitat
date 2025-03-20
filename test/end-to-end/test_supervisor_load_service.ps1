hab pkg install core/nginx --channel stable

$job = Start-Job { hab sup run }
Wait-Supervisor -Timeout 120

Describe "hab svc load" {
    $loadOut = hab svc load core/nginx
    Wait-SupervisorService nginx -Timeout 20

    It "Successfully loads service" {
        $loadOut | Should -Be "The core/nginx service was successfully loaded"
    }

    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }

    It "Actually runs nginx" {
        # This will error with a 403 because nginx is not running any sites
        try { Invoke-WebRequest "http://localhost" }
        catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
        [string]$headers.Server | Should -BeLike "nginx/*"
    }

    AfterAll {
        hab svc unload core/nginx
        Wait-SupervisorServiceUnload nginx -Timeout 20
        Stop-Job -Job $job
        Remove-Job -Job $job
    }
}
if ($IsLinux) {
    $job = Start-Job { bash -c 'umask 077; hab sup run' }
    Wait-Supervisor -Timeout 120

    Describe "hab svc load on hardened system" {
        $loadOut = hab svc load core/nginx
        Wait-SupervisorService nginx -Timeout 20

        It "Successfully loads service" {
            $loadOut | Should -Be "The core/nginx service was successfully loaded"
        }

        It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
        }

        It "Actually runs nginx" {
            # This will error with a 403 because nginx is not running any sites
            try { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "nginx/*"
        }

        It "is running with umask 0077" {
            $s = grep Umask /proc/$(pgrep hab-sup)/status
            $s | Should -Match "Umask:\s+0077"
        }

        AfterAll {
            hab svc unload core/nginx
            Wait-SupervisorServiceUnload nginx -Timeout 20
            Stop-Job -Job $job
            Remove-Job -Job $job
        }
    }
}
