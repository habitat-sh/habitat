Write-Host "Running health_check hook: {{pkg.ident}}"
Start-Sleep 2
Write-Host "health_check finished!"
Exit "$(Get-Content {{pkg.path}}\health_exit)"