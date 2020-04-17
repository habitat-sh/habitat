function Stop-ProcessTree($Id) {
    Get-CimInstance Win32_Process | Where-Object { $_.ParentProcessId -eq $Id } | ForEach-Object { Kill-Tree $_.ProcessId }
    Stop-Process -Id $Id -ErrorAction SilentlyContinue
}

Describe "hab studio enter with studio_profile.ps1" {
    BeforeAll {
        Set-Content studio_profile.ps1 -value "write-host 'hohoho';kill `$PID"
        hab origin key generate $env:HAB_ORIGIN
    }
    Context "No HAB_STUDIO_NOPROFILE set" {
        $env:HAB_STUDIO_NOPROFILE = $null

        It "sources studio_profile.ps1" {
            if($env:DOCKER_STUDIO_TEST) {
                $habEnterCmd = "hab studio enter -D"
            } else {
                $habEnterCmd = "hab studio enter"
            }
            $result = Invoke-Expression $habEnterCmd
            $result[-1] | Should -Be "hohoho"
        }
    }
    Context "HAB_STUDIO_NOPROFILE is set" {
        $env:HAB_STUDIO_NOPROFILE = $true

        It "does not source studio_profile.ps1" {
            $studioArgs = @("studio", "enter")
            if($env:DOCKER_STUDIO_TEST) {
                $env:HAB_DOCKER_OPTS = "-l buildkitejob=$env:BUILDKITE_JOB_ID"
                $studioArgs += "-D"
            }
            $procArgs = @{
                FilePath               = "hab"
                ArgumentList           = $studioArgs
                RedirectStandardOutput = "out.log"
                PassThru               = $true
            }
            $proc = Start-Process @procArgs
            Wait-PathIncludesContent -Path out.log -Content "[HAB-STUDIO] Habitat:\src>"
            Kill-Tree $proc.Id
        }
    }
    AfterAll {
        Remove-Item studio_profile.ps1
        Remove-Item out.log
        if($env:DOCKER_STUDIO_TEST) {
            docker ps -q --filter "label=buildkitejob=$env:BUILDKITE_JOB_ID" | ForEach-Object { docker stop $_ }
        }
    }
}
