hab pkg install core/windows-service
hab pkg exec core/windows-service install

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
}
