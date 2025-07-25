hab pkg install core/windows-service

Describe "Habitat Windows Service" {
    It "Starts the service" {
        Start-Service Habitat
        Wait-Supervisor -Timeout 45
        (Invoke-WebRequest "http://localhost:9631/butterfly" -UseBasicParsing).StatusCode | Should -Be 200
    }
    It "Stops the service" {
        Stop-Service Habitat
        Get-Process hab-sup -ErrorAction SilentlyContinue | Should -Be $null
        Get-Process hab-launch -ErrorAction SilentlyContinue | Should -Be $null
    }

    Context "Running a service with current launcher" {
        BeforeAll {
            Stop-Service Habitat
            Remove-Item "c:\hab\svc\windows-service\logs\habitat.log"
            Start-Service Habitat
            Wait-Supervisor -Timeout 45
            hab svc load core/nginx --channel stable
            Wait-SupervisorService nginx
        }

        It "Runs nginx" {
            # This will error with a 403 because nginx is not running any sites
            try  { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "nginx/*"
        }

        It "Runs service as system" {
            "c:\hab\svc\windows-service\logs\habitat.log" | Should -FileContentMatch "Starting service as user=system"
        }

        AfterAll {
            hab svc unload core/nginx
            Start-Sleep -Seconds 3 # tears
            Stop-Service Habitat
        }
    }

    Context "Running a service with older launcher" {
        BeforeAll {
            Stop-Service Habitat
            Remove-Item "c:\hab\svc\windows-service\logs\habitat.log"
            # This was the last stable Windows Launcher prior to the Launcher
            # being able to provide version numbers to the Supervisor directly.
            hab pkg install core/hab-launcher/13927/20200618210540
            $launcherPath = "$(hab pkg path core/hab-launcher/13927/20200618210540)\bin\hab-launch.exe"
            $configPath = "c:\hab\svc\windows-service\HabService.dll.config"
            [xml]$configXml = Get-Content $configPath
            $launcherPathNode = ($configXml.configuration.appSettings.SelectNodes("add[@key='launcherPath']"))[0]
            $launcherPathNode.SetAttribute("value", $launcherPath) | Out-Null
            $configXml.Save($configPath)
            Start-Service Habitat
            Wait-Supervisor -Timeout 45
            hab svc load core/nginx --channel stable
            Wait-SupervisorService nginx
        }

        It "Runs nginx" {
            # This will error with a 403 because nginx is not running any sites
            try  { Invoke-WebRequest "http://localhost" }
            catch [Microsoft.PowerShell.Commands.HttpResponseException] { $headers = $_.Exception.Response.Headers }
            [string]$headers.Server | Should -BeLike "nginx/*"
        }

        It "Runs service as machine user" {
            $lowerComputerName = $env:COMPUTERNAME.ToLower()
            "c:\hab\svc\windows-service\logs\habitat.log" | Should -FileContentMatch "Starting service as user=$lowerComputerName\`$"
        }

        AfterAll {
            hab svc unload core/nginx
            Stop-Service Habitat
        }
    }
}
