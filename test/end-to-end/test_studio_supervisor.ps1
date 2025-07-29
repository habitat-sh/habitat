$cliVersion = ((hab --version) -split " ")[1]
$env:HAB_STUDIO_SECRET_HAB_INTERNAL_BLDR_CHANNEL="dev-v1.6"
hab origin key generate $env:HAB_ORIGIN


Describe "Studio supervisor" {
    It "version should match hab cli" {
        $result = (Invoke-StudioRun "hab sup --version")
        $result[-1] | Should -match "sup $(($cliVersion -split '/')[0])/*"
    }

    It "should be running" {
        $studioArgs = @("studio", "enter")
        if($env:DOCKER_STUDIO_TEST) {
            $port = (Get-Random -Minimum 8000 -Maximum 9000)
            $env:HAB_DOCKER_OPTS = "-p ${port}:9631 -l buildkitejob=$env:BUILDKITE_JOB_ID"
            $studioArgs += "-D"
        } else {
            $port = 9631
            $studioArgs += "-R"
        }
        $procArgs = @{
            FilePath     = "hab"
            ArgumentList = $studioArgs
            WindowStyle  = "hidden"
        }
        Start-Process @procArgs
        Wait-Supervisor -Timeout 60 -Port $port
        (Invoke-WebRequest "http://localhost:$port/butterfly" -UseBasicParsing).StatusCode | Should -Be 200
    }

    AfterAll {
        if($env:DOCKER_STUDIO_TEST) {
            docker ps -q --filter "label=buildkitejob=$env:BUILDKITE_JOB_ID" | ForEach-Object { docker stop $_ }
        }
    }
}
