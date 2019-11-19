hab pkg install core/nginx --channel stable
$job = start-job { hab sup run }
hab pkg install core/nginx
Wait-Supervisor -Timeout 120

Describe "hab svc load" {
    $loadOut = hab svc load core/nginx
    Wait-SupervisorService nginx -Timeout 20

    It "Succesfully loads service" {
        $loadOut | Should -Be "The core/nginx service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "Actually runs nginx" {
        # This will error with a 403 because nginx is not running any sites
        try  { Invoke-WebRequest "http://localhost" }
        catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
        [string]$headers.Server | Should -BeLike "nginx/*"
    }

    AfterAll {
        hab svc unload core/nginx
        Stop-Job -Job $job
        Remove-Job -Job $job
    }
}
