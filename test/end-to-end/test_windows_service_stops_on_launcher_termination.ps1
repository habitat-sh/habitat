hab pkg install core/windows-service

Describe "Terminate Launcher" {
    Start-Service Habitat
    Wait-Supervisor -Timeout 45
    Get-Process hab-launch | Stop-Process -Force
    Start-Sleep -Seconds 2

    It "Stops service" {
        (Get-Service Habitat).Status | Should -Be "Stopped"
    }
}

Describe "Bogus Launcher args" {
    Stop-Service Habitat
    (Get-Content c:/hab/svc/windows-service/HabService.dll.config).replace('--no-color', '--poop') |
      Set-Content c:/hab/svc/windows-service/HabService.dll.config
    Start-Service Habitat
    Start-Sleep -Seconds 2

    It "Stops service" {
        (Get-Service Habitat).Status | Should -Be "Stopped"
    }
}
