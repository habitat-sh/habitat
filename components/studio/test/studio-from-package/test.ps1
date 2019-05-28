$ErrorActionPreference = "Stop"

$env:HAB_LICENSE = "accept-no-persist"

hab studio rm

& test/shared/test-all.ps1 -studio_command "hab studio"

exit $LASTEXITCODE
