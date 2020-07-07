Write-Host "Initializing package {{pkg.ident}} "
Start-Sleep 1
Write-Host "... reticulating splines ..."
Write-Host "... integrating curves ..."
Write-Host "... relaxing splines ..."
Write-Host "... calculating inverse probability matrices ..."
Start-Sleep 1
Write-Host "Deliberately taking a long time in the init hook"
For ($i=0; $i -lt 10; $i++) {
      Start-Sleep 1
      Write-Host "Sleeping ($i)/10..."
}
Write-Host "init hook DONE"
