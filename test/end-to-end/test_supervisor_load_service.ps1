$env:HAB_NOCOLORING = "true"
$env:HAB_NONINTERACTIVE = "true"

$pkg = "core/nginx"
$svc = "nginx"
hab pkg install $pkg --channel stable
$pkgSpecsPath = (Join-Path $env:SystemDrive hab sup default specs $svc) + ".spec"

Describe "hab svc load" {
    Context "No HAB_BLDR_URL, HAB_BLDR_CHANNEL environment" {
        $loadOut = hab svc load $pkg
        Wait-SupervisorService $svc -Timeout 20

        It "Succesfully loads service" {
            $loadOut | Should -Be "The $pkg service was successfully loaded"
        }
        It "Reports service on HTTP Gateway as UP" {
            ((Invoke-WebRequest "http://localhost:9631/services/$svc/default" -UseBasicParsing).content | ConvertFrom-Json).process.state | Should -Be "up"
        }
        It "Actually runs $svc" {
            # This will error with a 403 because nginx is not running any sites
            try  { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "$svc/*"
        }
        It "Has correct defaults in the service spec" {
            $pkgSpecsPath | Should -FileContentMatch 'channel = "stable"'
            $pkgSpecsPath | Should -FileContentMatch 'bldr_url = "https://bldr.habitat.sh"'
        }
        BeforeAll {
            $env:HAB_BLDR_URL = ''
            $env:HAB_BLDR_CHANNEL = ''
            $job = Start-Supervisor -Timeout 45
        }
        AfterAll {
            Unload-SupervisorService -PackageName $pkg -Timeout 20
            Stop-Job -Job $job
            Remove-Job -Job $job
        }
    }

    Context "HAB_BLDR_URL, HAB_BLDR_CHANNEL from (hab svc load) environment wins" {
        # ENVVars are set in the client environment
        $env:HAB_BLDR_URL = 'https://clienturl.tld'
        $env:HAB_BLDR_CHANNEL = 'clientchan'
        $loadOut = hab svc load $pkg
        Wait-SupervisorService $svc -Timeout 20

        It "Succesfully loads service" {
            $loadOut | Should -Be "The $pkg service was successfully loaded"
        }
        It "Has correct defaults in the service spec" {
            $pkgSpecsPath | Should -FileContentMatch 'channel = "clientchan"'
            $pkgSpecsPath | Should -FileContentMatch 'bldr_url = "https://clienturl.tld"'
        }
        BeforeAll {
            # ENVVars are set in the Supervisor environment
            # but the ones from the client environment will be used
            $env:HAB_BLDR_URL = 'https://supurl.tld'
            $env:HAB_BLDR_CHANNEL = 'supchannel'
            $job = Start-Supervisor -Timeout 45
        }
        AfterAll {
            Unload-SupervisorService -PackageName $pkg -Timeout 20
            Stop-Job -Job $job
            Remove-Job -Job $job
        }
    }

    Context "HAB_BLDR_URL, HAB_BLDR_CHANNEL from running supervisor environment" {
        # ENVVars are not set in the client environment
        $env:HAB_BLDR_URL = ''
        $env:HAB_BLDR_CHANNEL = ''
        $loadOut = hab svc load $pkg
        Wait-SupervisorService $svc -Timeout 20

        It "Succesfully loads service" {
            $loadOut | Should -Be "The $pkg service was successfully loaded"
        }
        It "Has correct defaults in the service spec" {
            $pkgSpecsPath | Should -FileContentMatch 'channel = "supchannel"'
            $pkgSpecsPath | Should -FileContentMatch 'bldr_url = "https://supurl.tld"'
        }
        BeforeAll {
            # ENVVars are set in the Supervisor environment
            # since nothing is passed from the client, the supervisor values will be used
            $env:HAB_BLDR_URL = 'https://supurl.tld'
            $env:HAB_BLDR_CHANNEL = 'supchannel'
            $job = Start-Supervisor -Timeout 45
        }
        AfterAll {
            Unload-SupervisorService -PackageName $pkg -Timeout 20
            Stop-Job -Job $job
            Remove-Job -Job $job
        }
    }
}
