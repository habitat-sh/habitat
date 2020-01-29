$cliVersion = ((hab --version) -split " ")[1]
$env:HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL="dev"
hab origin key generate $env:HAB_ORIGIN

# Note we kill all containers before and after this test
# At some point we can remove the BeforeAll but neet it now
# for straggler test VMs

Describe "Studio supervisor" {
    BeforeAll {
        if($env:DOCKER_STUDIO_TEST) {
            docker ps -q | ForEach-Object { docker stop $_ }
        }
    }

    It "version should match hab cli" {
        $result = (Invoke-StudioRun "hab sup --version")
        $result[-1] | Should -match "sup $(($cliVersion -split '/')[0])/*"
    }

    It "should be running" {
        $studioArgs = @("studio", "enter")
        if($env:DOCKER_STUDIO_TEST) {
            $studioArgs += "-D"
        } else {
            $studioArgs += "-R"
        }
        $procArgs = @{
            FilePath     = "hab"
            ArgumentList = $studioArgs
            WindowStyle  = "hidden"
        }
        Start-Process @procArgs
        Wait-Supervisor -Timeout 60
        (Invoke-WebRequest "http://localhost:9631/butterfly" -UseBasicParsing).StatusCode | Should -Be 200
    }

    AfterAll {
        if($env:DOCKER_STUDIO_TEST) {
            docker ps -q | ForEach-Object { docker stop $_ }
        }
    }
}
