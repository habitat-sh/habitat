Write-Host "Deliberately long post-stop hook executing: {{pkg.ident}}"
For ($i = 0; $i -lt 15; $i++) {
    Start-Sleep 1
    Write-Host "Sleeping ($i)/15..."
}
Write-Host "post-stop hook DONE"
