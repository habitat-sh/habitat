hab pkg install core/nginx --channel stable
$job = start-job { hab sup run }
Wait-Supervisor -Timeout 45

Describe "hab svc load" {
    $loadOut = hab svc load core/nginx
    Start-Sleep -Seconds 5

    It "Succesfully loads service" {
        $loadOut | Should -Be "The core/nginx service was successfully loaded"
    }
    It "Reports service on HTTP Gateway as UP" {
        ((Invoke-WebRequest "http://localhost:9631/services/nginx/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
    }
    It "Actually runs nginx" {
        # This will error with a 403 because nginx is not running any sites
        try  { Invoke-WebRequest "http://localhost" -Method HEAD -UseBasicParsing }
        catch [System.Net.WebException] { $response = $_.Exception.Response }
        $response.Server | Should -BeLike "nginx/*"
    }

    AfterAll {
        hab svc unload core/nginx
        Stop-Job -Job $job
        Remove-Job -Job $job
    }
}
